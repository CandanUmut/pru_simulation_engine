use bevy::math::primitives::Sphere;
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::pru::cell::PruCell;

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
}

impl PruUniverse {
    /// Construct a new universe description with zeroed counters.
    pub fn new(grid_dimensions: UVec3, spacing: f32, base_dt: f32) -> Self {
        Self {
            grid_dimensions,
            spacing,
            base_dt,
            total_cells: 0,
        }
    }
}

/// Startup system: build a small 3D lattice of PRU cells with random lock values.
pub fn setup_universe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Configure a modest grid that is fast to render while showcasing the lattice.
    let grid_dimensions = UVec3::new(10, 10, 10);
    let spacing = 1.4;
    let base_dt = 1.0 / 60.0;

    let mut universe = PruUniverse::new(grid_dimensions, spacing, base_dt);
    commands.insert_resource(universe.clone());

    let mut rng = StdRng::seed_from_u64(42);
    let cell_mesh = meshes.add(Mesh::from(Sphere { radius: 0.12 }));

    let center_offset = (grid_dimensions.as_vec3() - Vec3::ONE) * 0.5 * spacing;

    for x in 0..grid_dimensions.x {
        for y in 0..grid_dimensions.y {
            for z in 0..grid_dimensions.z {
                let position = Vec3::new(x as f32, y as f32, z as f32) * spacing - center_offset;
                let ua_mass_lock: f64 = rng.gen_range(0.4..1.6);
                let ub_geom_lock: f64 = rng.gen_range(-1.0..1.0);

                let cell = PruCell::new(position, ua_mass_lock, ub_geom_lock);

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
                    Name::new(format!("PRU Cell ({x}, {y}, {z})")),
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
