use bevy::prelude::*;

use crate::app::SimulationState;
use crate::pru::cell::{PruCell, PruDynamics};
use crate::pru::gravity_relational::{apply_relational_gravity, RelationalKernel};
use crate::pru::universe::PruUniverse;

// =========================
// PHASE 3: MACRO GRAVITY & LARGE-SCALE STRUCTURE
// Status: IN PROGRESS (naive pairwise gravity, relational lattice gravity, motion integration, energy metrics)
// =========================

/// Choice of macro-gravity solver.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GravityMode {
    /// Baseline O(N^2) pairwise solver for debugging and small-N comparisons.
    NaiveNBody,
    /// PRU-style lattice solver that uses precomputed neighbor kernels.
    RelationalLattice,
}

/// Tunable parameters controlling the effective gravity model.
#[derive(Resource, Clone)]
pub struct GravityParams {
    /// Effective gravitational constant (dimensionless scaling of the UA-derived mass product).
    pub g_effective: f32,
    /// Softening length to avoid singularities at tiny separations or to tame the relational kernel gain.
    pub softening_length: f32,
    /// Simple velocity damping to keep the naive integrator stable.
    pub damping: f32,
    /// Clamp extremely large accelerations that would destabilize the scene.
    pub max_acceleration: f32,
    /// Whether gravity forces are applied (integration still runs for inertial motion).
    pub enabled: bool,
    /// Active solver controlling how accelerations are computed.
    pub mode: GravityMode,
}

impl Default for GravityParams {
    fn default() -> Self {
        Self {
            g_effective: 0.6,
            softening_length: 0.25,
            damping: 0.01,
            max_acceleration: 120.0,
            enabled: true,
            mode: GravityMode::RelationalLattice,
        }
    }
}

/// Rolling energy diagnostics for the gravity simulation.
#[derive(Resource, Clone, Copy, Default)]
pub struct SimulationEnergy {
    pub kinetic: f64,
    pub potential: f64,
    pub total: f64,
    pub initial_total: Option<f64>,
    pub relative_drift: Option<f64>,
}

/// Simulate pending fixed steps using a naive O(N^2) pairwise gravity rule.
///
/// The implementation keeps the logic in one place so future grid/octree-based
/// accelerators can swap in while preserving the integrator and UI plumbing.
pub fn simulate_gravity_step(
    params: Res<GravityParams>,
    universe: Res<PruUniverse>,
    kernel: Option<Res<RelationalKernel>>,
    mut sim_state: ResMut<SimulationState>,
    cell_data_query: Query<(&PruCell, &PruDynamics)>,
    mut bodies: Query<(&mut PruCell, &mut PruDynamics, &mut Transform)>,
) {
    let steps = sim_state.take_pending_steps();
    if steps == 0 {
        return;
    }

    let dt = sim_state.dt;
    let softening2 = params.softening_length * params.softening_length;

    for _ in 0..steps {
        // Reset accelerations before accumulating forces for this fixed step.
        for (_, mut dyn_state, _) in bodies.iter_mut() {
            dyn_state.acceleration = Vec3::ZERO;
        }

        if params.enabled {
            match params.mode {
                GravityMode::NaiveNBody => {
                    // Pairwise force accumulation using Bevy's combination iterator.
                    let mut combos = bodies.iter_combinations_mut();
                    while let Some([(cell_a, mut dyn_a, _), (cell_b, mut dyn_b, _)]) =
                        combos.fetch_next()
                    {
                        let displacement = cell_b.position - cell_a.position;
                        let dist2 = displacement.length_squared() + softening2;
                        if dist2 <= 0.0 {
                            continue;
                        }

                        let inv_dist = dist2.sqrt().recip();
                        let inv_dist3 = inv_dist * inv_dist * inv_dist;
                        let mass_product = dyn_a.mass * dyn_b.mass;
                        if mass_product <= 0.0 {
                            continue;
                        }

                        let force_mag = params.g_effective * mass_product * inv_dist3;
                        let direction = displacement * inv_dist;

                        let accel_a = direction * (force_mag / dyn_a.mass);
                        let accel_b = direction * (force_mag / dyn_b.mass);

                        dyn_a.acceleration += accel_a;
                        dyn_b.acceleration -= accel_b;
                    }
                }
                GravityMode::RelationalLattice => {
                    if let Some(kernel) = kernel.as_ref() {
                        // Snapshot the lattice masses so we can feed a dense lookup table to the
                        // relational kernel. This keeps runtime work to neighbor lookups instead
                        // of all-pairs force evaluation.
                        let cell_data: Vec<(UVec3, f32)> = cell_data_query
                            .iter()
                            .map(|(cell, dyn_state)| (cell.grid_coords, dyn_state.mass))
                            .collect();
                        apply_relational_gravity(
                            &params,
                            &universe,
                            kernel,
                            &cell_data,
                            &mut bodies,
                        );
                    }
                }
            }
        }

        // Integrate motion (semi-implicit Euler).
        for (mut cell, mut dyn_state, mut transform) in bodies.iter_mut() {
            if dyn_state.acceleration.length_squared()
                > params.max_acceleration * params.max_acceleration
            {
                dyn_state.acceleration = dyn_state
                    .acceleration
                    .clamp_length_max(params.max_acceleration);
            }

            let accel = dyn_state.acceleration;
            dyn_state.velocity += accel * dt;
            dyn_state.velocity *= 1.0 - params.damping * dt;
            cell.position += dyn_state.velocity * dt;
            transform.translation = cell.position;
        }
    }
}

/// Compute kinetic and potential energy for diagnostics shown in the HUD.
pub fn compute_energy_metrics(
    params: Res<GravityParams>,
    mut energy: ResMut<SimulationEnergy>,
    bodies: Query<(&PruCell, &PruDynamics)>,
) {
    let mut kinetic = 0.0f64;
    for (_cell, dyn_state) in bodies.iter() {
        kinetic += 0.5 * dyn_state.mass as f64 * dyn_state.velocity.length_squared() as f64;
    }

    let mut potential = 0.0f64;
    {
        let mut combos = bodies.iter_combinations();
        while let Some([(cell_a, dyn_a), (cell_b, dyn_b)]) = combos.fetch_next() {
            let displacement = cell_b.position - cell_a.position;
            let distance = (displacement.length_squared()
                + params.softening_length * params.softening_length)
                .sqrt();
            if distance > 0.0 {
                let term = -params.g_effective as f64 * dyn_a.mass as f64 * dyn_b.mass as f64
                    / distance as f64;
                potential += term;
            }
        }
    }

    energy.kinetic = kinetic;
    energy.potential = potential;
    energy.total = kinetic + potential;

    if energy.initial_total.is_none() && energy.total.abs() > 1e-9 {
        energy.initial_total = Some(energy.total);
    }

    if let Some(initial) = energy.initial_total {
        if initial.abs() > 1e-9 {
            energy.relative_drift = Some((energy.total - initial) / initial);
        }
    }
}
