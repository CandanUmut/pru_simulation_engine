use bevy::prelude::*;

use crate::pru::cell::DerivedFields;
use crate::pru::universe::{compute_derived_fields, setup_universe, FieldMetrics};
use crate::render::RenderPlugin;
use crate::ui::controls::VisualModeSettings;
use crate::ui::UiPlugin;

/// Global simulation state controlling the PRU tick loop and time scaling.
#[derive(Resource, Clone, Copy)]
pub struct SimulationState {
    /// Whether the simulation is currently advancing.
    pub running: bool,
    /// Multiplier applied to real time to speed up or slow down ticks.
    pub time_scale: f32,
    /// Current discrete tick counter.
    pub tick: u64,
    /// Fixed simulation delta time (seconds per tick).
    pub dt: f32,
    /// Accumulated (scaled) time used to trigger ticks.
    pub accumulated_time: f32,
    /// Total simulated time in seconds.
    pub simulation_time: f32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            running: true,
            time_scale: 1.0,
            tick: 0,
            dt: 1.0 / 60.0,
            accumulated_time: 0.0,
            simulation_time: 0.0,
        }
    }
}

impl SimulationState {
    /// Toggle the running flag.
    pub fn toggle(&mut self) {
        self.running = !self.running;
    }

    /// Advance by a single tick even while paused.
    pub fn step_once(&mut self) {
        self.tick += 1;
        self.simulation_time += self.dt;
    }

    /// Adjust time scale while keeping it within a reasonable range.
    pub fn adjust_speed(&mut self, delta: f32) {
        self.time_scale = (self.time_scale + delta).clamp(0.1, 10.0);
    }
}

/// Plugin responsible for initializing the PRU universe and advancing ticks.
pub struct PruSimulationPlugin;

impl Plugin for PruSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_universe).add_systems(
            Update,
            (
                advance_simulation_time,
                compute_derived_fields,
                update_cell_materials.after(compute_derived_fields),
                animate_cells.after(update_cell_materials),
            ),
        );
    }
}

/// Drive the fixed-timestep tick counter using real time scaled by the simulation speed.
fn advance_simulation_time(time: Res<Time>, mut sim_state: ResMut<SimulationState>) {
    if !sim_state.running {
        return;
    }

    sim_state.accumulated_time += time.delta_seconds() * sim_state.time_scale;
    while sim_state.accumulated_time >= sim_state.dt {
        sim_state.accumulated_time -= sim_state.dt;
        sim_state.tick += 1;
        sim_state.simulation_time += sim_state.dt;
    }
}

/// Animate cell visuals slightly using their lock values to hint at PRU activity.
fn animate_cells(
    time: Res<Time>,
    mut query: Query<(&crate::pru::cell::PruCell, &DerivedFields, &mut Transform)>,
) {
    let elapsed = time.elapsed_seconds();
    for (cell, derived, mut transform) in query.iter_mut() {
        let base_scale = 0.12 + derived.local_density * 0.04;
        let curvature_amp = (derived.curvature_proxy.abs() * 0.2).min(0.08);
        let pulse = (elapsed * 0.7 + cell.ub_geom_lock as f32).sin() * 0.025;
        transform.scale = Vec3::splat((base_scale + curvature_amp + pulse).max(0.02));
    }
}

/// Adjust materials based on derived fields and visualization toggles.
fn update_cell_materials(
    modes: Res<VisualModeSettings>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &crate::pru::cell::PruCell,
        &DerivedFields,
        &Handle<StandardMaterial>,
    )>,
) {
    for (cell, derived, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(material_handle) {
            let (base_color, emissive) = if modes.show_density_coloring {
                (density_color(derived.local_density), Color::BLACK)
            } else if modes.show_curvature_coloring {
                let intensity = (derived.curvature_proxy.abs() * 0.6).min(1.2);
                (
                    curvature_color(derived.curvature_proxy),
                    Color::srgb(intensity * 0.4, intensity * 0.2, intensity * 0.9),
                )
            } else {
                (
                    seed_color_from_locks(cell.ua_mass_lock, cell.ub_geom_lock),
                    Color::BLACK,
                )
            };

            material.base_color = base_color;
            material.emissive = emissive.into();
        }
    }
}

fn density_color(density: f32) -> Color {
    let norm = (density / 1.8).clamp(0.0, 1.0);
    let cold = Color::srgb(0.2, 0.4, 0.9);
    let warm = Color::srgb(1.0, 0.9, 0.2);
    lerp_color(cold, warm, norm)
}

fn curvature_color(curvature: f32) -> Color {
    let norm = (curvature * 0.8).clamp(-1.0, 1.0);
    if norm >= 0.0 {
        Color::srgb(0.3 + 0.5 * norm, 0.25, 0.85)
    } else {
        Color::srgb(0.15, 0.65 + norm * -0.5, 0.3 + -norm * 0.4)
    }
}

fn seed_color_from_locks(ua: f64, ub: f64) -> Color {
    let mass = (ua as f32).clamp(0.0, 2.0);
    let geom = ((ub as f32) + 1.0) * 0.5; // map -1..1 to 0..1

    let r = 0.2 + 0.6 * geom;
    let g = 0.2 + 0.6 * (1.0 - geom);
    let b = 0.4 + 0.5 * (1.0 - mass * 0.5);
    Color::srgb(r.min(1.0), g.min(1.0), b.min(1.0))
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let a_lin = a.to_linear();
    let b_lin = b.to_linear();
    let mixed = a_lin * (1.0 - t) + b_lin * t;
    Color::LinearRgba(mixed)
}

/// Build and run the Bevy application with simulation, rendering, and UI layers.
pub fn run_app() {
    App::new()
        .insert_resource(SimulationState::default())
        .init_resource::<FieldMetrics>()
        .init_resource::<VisualModeSettings>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)))
        .insert_resource(AmbientLight {
            color: Color::srgb(0.4, 0.45, 0.5),
            brightness: 0.35,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PRU Universe Bevy Simulation".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((RenderPlugin, UiPlugin, PruSimulationPlugin))
        .run();
}
