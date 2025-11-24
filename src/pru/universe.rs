use bevy::math::primitives::Sphere;
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::VecDeque;

use crate::pru::cell::{DerivedFields, PruCell, PruDynamics};
use crate::pru::gravity::GravityParams;

/// Resource describing the high-level PRU universe configuration.
#[derive(Resource, Clone)]
pub struct PruUniverse {
    /// Discrete grid dimensions of the PRU lattice.
    pub grid_dimensions: UVec3,
    /// World-space spacing between adjacent cells.
    pub spacing: f32,
    /// Base fixed delta time per tick (seconds).
    pub base_dt: f32,
    /// Aggregate count of spawned cells.
    pub total_cells: usize,
    /// Whether macro-gravity is enabled for dynamic motion.
    pub gravity_enabled: bool,
}

impl PruUniverse {
    /// Construct a new universe description with zeroed counters.
    pub fn new(grid_dimensions: UVec3, spacing: f32, base_dt: f32) -> Self {
        Self {
            grid_dimensions,
            spacing,
            base_dt,
            total_cells: 0,
            gravity_enabled: true,
        }
    }
}

/// Rolling metrics gathered from the derived field calculations.
#[derive(Resource)]
pub struct FieldMetrics {
    pub avg_density: f32,
    pub min_density: f32,
    pub max_density: f32,
    pub avg_curvature: f32,
    pub density_history: VecDeque<f32>,
    pub max_history: usize,
}

impl Default for FieldMetrics {
    fn default() -> Self {
        Self {
            avg_density: 0.0,
            min_density: 0.0,
            max_density: 0.0,
            avg_curvature: 0.0,
            density_history: VecDeque::from(vec![0.0; 32]),
            max_history: 64,
        }
    }
}

/// Startup system: build a small 3D lattice of PRU cells with random lock values.
pub fn setup_universe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gravity: ResMut<GravityParams>,
) {
    // Configure a modest grid that is fast to render while showcasing the lattice.
    let grid_dimensions = UVec3::new(10, 10, 10);
    let spacing = 1.4;
    let base_dt = 1.0 / 60.0;

    let mut universe = PruUniverse::new(grid_dimensions, spacing, base_dt);
    commands.insert_resource(universe.clone());
    gravity.enabled = universe.gravity_enabled;

    let mut rng = StdRng::seed_from_u64(42);
    let cell_mesh = meshes.add(Mesh::from(Sphere { radius: 0.12 }));

    let center_offset = (grid_dimensions.as_vec3() - Vec3::ONE) * 0.5 * spacing;

    for x in 0..grid_dimensions.x {
        for y in 0..grid_dimensions.y {
            for z in 0..grid_dimensions.z {
                let position = Vec3::new(x as f32, y as f32, z as f32) * spacing - center_offset;
                let ua_mass_lock: f64 = rng.gen_range(0.4..1.6);
                let ub_geom_lock: f64 = rng.gen_range(-1.0..1.0);

                let grid_coords = UVec3::new(x, y, z);
                let cell = PruCell::new(position, grid_coords, ua_mass_lock, ub_geom_lock);
                let mass = (ua_mass_lock as f32).max(0.05);
                let velocity = Vec3::new(
                    rng.gen_range(-0.05..0.05),
                    rng.gen_range(-0.05..0.05),
                    rng.gen_range(-0.05..0.05),
                );
                let dynamics = PruDynamics {
                    mass,
                    velocity,
                    ..Default::default()
                };

                let material_color = color_from_locks(ua_mass_lock, ub_geom_lock);
                let material = materials.add(StandardMaterial {
                    base_color: material_color,
                    metallic: 0.05,
                    perceptual_roughness: 0.7,
                    ..Default::default()
                });

                commands.spawn((
                    PbrBundle {
                        mesh: cell_mesh.clone(),
                        material,
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    },
                    cell,
                    DerivedFields::default(),
                    Name::new(format!("PRU Cell ({x}, {y}, {z})")),
                    dynamics,
                ));

                universe.total_cells += 1;
            }
        }
    }

    // Update the resource with the final cell count.
    commands.insert_resource(universe);
}

fn color_from_locks(ua: f64, ub: f64) -> Color {
    let mass = (ua as f32).clamp(0.0, 2.0);
    let geom = ((ub as f32) + 1.0) * 0.5; // map -1..1 to 0..1

    let r = 0.2 + 0.6 * geom;
    let g = 0.2 + 0.6 * (1.0 - geom);
    let b = 0.4 + 0.5 * (1.0 - mass * 0.5);
    Color::srgb(r.min(1.0), g.min(1.0), b.min(1.0))
}

/// Compute per-cell derived fields (density & curvature proxies) and update rolling metrics.
pub fn compute_derived_fields(
    universe: Res<PruUniverse>,
    cell_query: Query<(&PruCell, &PruDynamics)>,
    mut derived_query: Query<(&PruCell, &mut DerivedFields)>,
    mut metrics: ResMut<FieldMetrics>,
) {
    let smoothing_radius = universe.spacing * 2.5;
    let smoothing_inv = 1.0 / (smoothing_radius * 0.5).max(0.0001);

    let cell_data: Vec<(Vec3, f32, f32)> = cell_query
        .iter()
        .map(|(cell, dyn_state)| (cell.position, dyn_state.mass, cell.ub_geom_lock as f32))
        .collect();

    if cell_data.is_empty() {
        return;
    }

    let mut density_sum = 0.0;
    let mut curvature_sum = 0.0;
    let mut min_density = f32::MAX;
    let mut max_density = f32::MIN;

    for (cell, mut derived) in derived_query.iter_mut() {
        let mut density = 0.0f32;
        let mut ub_weighted = 0.0f32;
        let mut ub_weight_sum = 0.0f32;

        for (pos, mass, ub) in cell_data.iter() {
            let r = (*pos - cell.position).length();
            let weight = (-0.5 * (r * smoothing_inv).powi(2)).exp();
            density += *mass * weight;
            if r > 0.0 {
                ub_weighted += *ub * weight;
                ub_weight_sum += weight;
            }
        }

        derived.local_density = density.max(0.0);
        derived.curvature_proxy = if ub_weight_sum > 0.0 {
            (cell.ub_geom_lock as f32) - ub_weighted / ub_weight_sum
        } else {
            0.0
        };

        density_sum += derived.local_density;
        curvature_sum += derived.curvature_proxy.abs();
        min_density = min_density.min(derived.local_density);
        max_density = max_density.max(derived.local_density);
    }

    let total_cells = derived_query.iter().count() as f32;
    if total_cells > 0.0 {
        metrics.avg_density = density_sum / total_cells;
        metrics.min_density = min_density;
        metrics.max_density = max_density;
        metrics.avg_curvature = curvature_sum / total_cells;

        let avg_density = metrics.avg_density;
        metrics.density_history.push_back(avg_density);
        while metrics.density_history.len() > metrics.max_history {
            metrics.density_history.pop_front();
        }
    }
}
