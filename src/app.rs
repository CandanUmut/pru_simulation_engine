use bevy::prelude::*;

use crate::pru::universe::setup_universe;
use crate::render::RenderPlugin;
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
        app.add_systems(Startup, setup_universe)
            .add_systems(Update, (advance_simulation_time, animate_cells));
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
fn animate_cells(time: Res<Time>, mut query: Query<(&crate::pru::cell::PruCell, &mut Transform)>) {
    let elapsed = time.elapsed_seconds();
    for (cell, mut transform) in query.iter_mut() {
        let base_scale = 0.15 + (cell.ua_mass_lock as f32).abs() * 0.05;
        let pulse = (elapsed * 0.7 + cell.ub_geom_lock as f32).sin() * 0.025;
        transform.scale = Vec3::splat((base_scale + pulse).max(0.02));
    }
}

/// Build and run the Bevy application with simulation, rendering, and UI layers.
pub fn run_app() {
    App::new()
        .insert_resource(SimulationState::default())
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
