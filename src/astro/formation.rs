use std::collections::HashMap;

use bevy::prelude::*;

use crate::app::SimulationState;
use crate::pru::cell::{DerivedFields, PruCell};
use crate::pru::universe::PruUniverse;

use super::black_hole::BlackHole;
use super::galaxy::{Galaxy, GalaxyIdCounter};
use super::star::{star_color_from_temperature, Star};

/// Tunable thresholds controlling when structures emerge.
#[derive(Resource, Clone)]
pub struct FormationSettings {
    pub star_density_threshold: f32,
    pub black_hole_density_threshold: f32,
    pub black_hole_curvature_threshold: f32,
    pub galaxy_density_threshold: f32,
    pub formation_interval: u64,
    pub galaxy_refresh_interval: u64,
    pub region_size: u32,
}

impl Default for FormationSettings {
    fn default() -> Self {
        Self {
            star_density_threshold: 1.8,
            black_hole_density_threshold: 3.0,
            black_hole_curvature_threshold: 0.25,
            galaxy_density_threshold: 1.2,
            formation_interval: 8,
            galaxy_refresh_interval: 24,
            region_size: 3,
        }
    }
}

#[derive(Resource, Default)]
pub struct FormationSchedule {
    pub last_star_tick: u64,
    pub last_galaxy_tick: u64,
}

pub fn spawn_stars_from_density(
    mut commands: Commands,
    universe: Res<PruUniverse>,
    sim_state: Res<SimulationState>,
    settings: Res<FormationSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut schedule: ResMut<FormationSchedule>,
    cell_query: Query<(&PruCell, &DerivedFields)>,
    existing_stars: Query<&Transform, With<Star>>,
) {
    if sim_state.tick - schedule.last_star_tick < settings.formation_interval {
        return;
    }
    schedule.last_star_tick = sim_state.tick;

    let star_mesh = meshes.add(Mesh::from(Sphere { radius: 0.3 }));
    let avoidance_radius = universe.spacing * 0.8;

    for (cell, derived) in cell_query.iter() {
        if derived.local_density < settings.star_density_threshold {
            continue;
        }

        let already_present = existing_stars
            .iter()
            .any(|t| (t.translation - cell.position).length() < avoidance_radius);
        if already_present {
            continue;
        }

        let radius = (derived.local_density * 0.08).clamp(0.05, 0.6);
        let temperature = 4000.0 + derived.local_density * 3000.0;
        let luminosity = derived.local_density * 2.0;
        let color = star_color_from_temperature(temperature);
        let emissive_scale = 1.2 + luminosity * 0.2;
        let emissive = Color::LinearRgba(color.to_linear() * emissive_scale);

        let material = materials.add(StandardMaterial {
            base_color: color,
            emissive: emissive.into(),
            unlit: false,
            ..Default::default()
        });

        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material,
                transform: Transform::from_translation(cell.position)
                    .with_scale(Vec3::splat(radius)),
                ..Default::default()
            },
            Star {
                mass: derived.local_density,
                radius,
                temperature,
                luminosity,
            },
            Name::new("Star"),
        ));
    }
}

pub fn spawn_black_holes_from_density(
    mut commands: Commands,
    universe: Res<PruUniverse>,
    sim_state: Res<SimulationState>,
    settings: Res<FormationSettings>,
    schedule: Res<FormationSchedule>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cell_query: Query<(&PruCell, &DerivedFields)>,
    existing_bh: Query<&Transform, With<BlackHole>>,
) {
    if sim_state.tick - schedule.last_star_tick < settings.formation_interval {
        // Reuse same cadence as star formation.
        return;
    }

    let avoidance_radius = universe.spacing * 0.9;
    let bh_mesh = meshes.add(Mesh::from(Sphere { radius: 0.4 }));

    for (cell, derived) in cell_query.iter() {
        if derived.local_density < settings.black_hole_density_threshold
            || derived.curvature_proxy.abs() < settings.black_hole_curvature_threshold
        {
            continue;
        }

        let already_present = existing_bh
            .iter()
            .any(|t| (t.translation - cell.position).length() < avoidance_radius);
        if already_present {
            continue;
        }

        let mass = derived.local_density * 4.0;
        let radius = (mass * 0.05).clamp(0.2, 1.5);
        let spin = derived.curvature_proxy.abs();

        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.02, 0.02, 0.05),
            perceptual_roughness: 0.9,
            metallic: 0.7,
            ..Default::default()
        });

        commands.spawn((
            PbrBundle {
                mesh: bh_mesh.clone(),
                material,
                transform: Transform::from_translation(cell.position)
                    .with_scale(Vec3::splat(radius)),
                ..Default::default()
            },
            BlackHole { mass, radius, spin },
            Name::new("Black Hole"),
        ));
    }
}

pub fn identify_galaxies(
    mut commands: Commands,
    sim_state: Res<SimulationState>,
    universe: Res<PruUniverse>,
    settings: Res<FormationSettings>,
    mut schedule: ResMut<FormationSchedule>,
    mut id_counter: ResMut<GalaxyIdCounter>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cell_query: Query<(&PruCell, &DerivedFields)>,
    stars: Query<&Transform, With<Star>>,
    mut galaxies: Query<(Entity, &mut Galaxy, &mut Transform)>,
) {
    if sim_state.tick - schedule.last_galaxy_tick < settings.galaxy_refresh_interval {
        return;
    }
    schedule.last_galaxy_tick = sim_state.tick;

    let mut regions: HashMap<UVec3, (f32, Vec3)> = HashMap::new();
    let region_size = settings.region_size.max(1);

    for (cell, derived) in cell_query.iter() {
        if derived.local_density < settings.galaxy_density_threshold {
            continue;
        }
        let key = UVec3::new(
            cell.grid_coords.x / region_size,
            cell.grid_coords.y / region_size,
            cell.grid_coords.z / region_size,
        );
        let entry = regions.entry(key).or_insert((0.0, Vec3::ZERO));
        entry.0 += derived.local_density;
        entry.1 += cell.position * derived.local_density;
    }

    // Update existing galaxies if their region is still valid.
    for (_entity, mut galaxy, mut transform) in galaxies.iter_mut() {
        if let Some((mass, weighted_pos)) = regions.remove(&galaxy.region_key) {
            let center = weighted_pos / mass.max(1e-3);
            let radius = (mass * 0.05).clamp(universe.spacing, universe.spacing * 8.0);
            galaxy.total_mass = mass;
            galaxy.center = center;
            galaxy.radius = radius;
            galaxy.num_stars = stars
                .iter()
                .filter(|t| (t.translation - center).length() < radius)
                .count() as u32;

            transform.translation = center;
            transform.scale = Vec3::splat(radius * 0.5);
        } else {
            // Fade out gracefully by shrinking the galaxy. If it becomes tiny, despawn later.
            galaxy.total_mass *= 0.9;
            galaxy.radius *= 0.9;
            transform.scale = Vec3::splat(galaxy.radius.max(0.1) * 0.5);
        }
    }

    let halo_mesh = meshes.add(Mesh::from(Sphere { radius: 1.0 }));

    // Spawn new galaxies for remaining regions.
    for (region_key, (mass, weighted_pos)) in regions.into_iter() {
        if mass < settings.galaxy_density_threshold * 3.0 {
            continue;
        }

        let center = weighted_pos / mass.max(1e-3);
        let radius = (mass * 0.05).clamp(universe.spacing, universe.spacing * 8.0);
        let id = id_counter.next();

        let color = Color::srgb(0.6, 0.8, 1.0);
        let halo_emissive = Color::LinearRgba(color.to_linear() * 0.05);
        let material = materials.add(StandardMaterial {
            base_color: color.with_alpha(0.1),
            emissive: halo_emissive.into(),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..Default::default()
        });

        commands.spawn((
            PbrBundle {
                mesh: halo_mesh.clone(),
                material,
                transform: Transform::from_translation(center)
                    .with_scale(Vec3::splat(radius * 0.5)),
                ..Default::default()
            },
            Galaxy {
                id,
                total_mass: mass,
                radius,
                num_stars: stars
                    .iter()
                    .filter(|t| (t.translation - center).length() < radius)
                    .count() as u32,
                center,
                region_key,
            },
            Name::new(format!("Galaxy #{id}")),
        ));
    }
}
