use bevy::prelude::*;

use crate::pru::cell::{PruCell, PruDynamics};
use crate::pru::gravity::GravityParams;
use crate::pru::universe::PruUniverse;

/// Precomputed neighbor offsets describing the fixed PRU lattice connectivity.
///
/// The kernel encodes the "relational" gravity idea: interactions are limited to
/// a small, local stencil that is reused each tick instead of recomputing
/// pairwise forces. The offsets themselves are discrete lattice jumps and never
/// change across the simulation.
pub const NEIGHBOR_OFFSETS: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];

/// Lookup-table weights derived once from the lattice spacing.
///
/// Each entry is a directionally-oriented weight that approximates
/// (1 / r^3) * r_hat for the offset measured in lattice units. The weights are
/// precomputed so runtime updates only perform cheap multiplications against the
/// local mass density field.
#[derive(Resource, Clone)]
pub struct RelationalKernel {
    pub offsets: Vec<IVec3>,
    pub weights: Vec<Vec3>,
}

impl RelationalKernel {
    pub fn new(spacing: f32) -> Self {
        let mut offsets = Vec::with_capacity(NEIGHBOR_OFFSETS.len());
        let mut weights = Vec::with_capacity(NEIGHBOR_OFFSETS.len());

        for offset in NEIGHBOR_OFFSETS.iter() {
            let world_offset = offset.as_vec3() * spacing;
            let distance_sq = world_offset.length_squared().max(1e-6);
            let inv_r3 = distance_sq.powf(-1.5);
            let direction = world_offset.normalize_or_zero();

            offsets.push(*offset);
            weights.push(direction * inv_r3);
        }

        Self { offsets, weights }
    }
}

/// Initialize the relational kernel resource once the universe is available.
///
/// This system keeps the kernel in a resource so the gravity step can run with
/// only neighbor lookups and table reads. It mirrors the PRU thesis idea of a
/// precomputed interaction graph instead of a per-frame all-to-all solve.
pub fn initialize_relational_kernel(mut commands: Commands, universe: Res<PruUniverse>) {
    let kernel = RelationalKernel::new(universe.spacing);
    commands.insert_resource(kernel);
}

/// Compute gravity using the precomputed kernel and the current mass density
/// field living on the PRU lattice.
///
/// The algorithm:
/// 1. Build a dense mass buffer indexed by lattice coordinates (a pure lookup
///    table with the same shape as the universe).
/// 2. For each cell, walk the fixed neighbor offsets and accumulate the
///    contributions using the cached kernel weights.
/// 3. Write the resulting acceleration into `PruDynamics` so the integrator can
///    update velocities/positions.
///
/// This keeps per-tick complexity at O(N * neighbors) and emphasizes local,
/// relational updates instead of a global all-pairs loop.
pub fn apply_relational_gravity(
    params: &GravityParams,
    universe: &PruUniverse,
    kernel: &RelationalKernel,
    cell_data: &[(UVec3, f32)],
    bodies: &mut Query<(&mut PruCell, &mut PruDynamics, &mut Transform)>,
) {
    let dims = universe.grid_dimensions;
    let volume = (dims.x * dims.y * dims.z) as usize;
    let mut mass_field = vec![0.0f32; volume];

    let idx = |coord: UVec3| -> usize {
        (coord.x * dims.y * dims.z + coord.y * dims.z + coord.z) as usize
    };

    for (coords, mass) in cell_data.iter() {
        mass_field[idx(*coords)] = *mass;
    }

    for (cell, mut dynamics, _) in bodies.iter_mut() {
        let mut accel = Vec3::ZERO;

        for (offset, weight) in kernel.offsets.iter().zip(kernel.weights.iter()) {
            let neighbor = cell.grid_coords.as_ivec3() + *offset;
            if neighbor.x < 0
                || neighbor.y < 0
                || neighbor.z < 0
                || neighbor.x >= dims.x as i32
                || neighbor.y >= dims.y as i32
                || neighbor.z >= dims.z as i32
            {
                continue;
            }

            let neighbor_coords = neighbor.as_uvec3();
            let neighbor_mass = mass_field[idx(neighbor_coords)];

            // Optional softening acts as a damped gain on the kernel to avoid
            // runaway accelerations when the lattice is tightly packed.
            let softened_gain = 1.0 / (1.0 + params.softening_length.max(0.0));
            accel += *weight * (params.g_effective * neighbor_mass * softened_gain);
        }

        dynamics.acceleration = accel;
    }
}
