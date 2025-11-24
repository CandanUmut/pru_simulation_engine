//! Rendering layer: cameras, lighting, and PRU cell visuals.

use bevy::prelude::*;

use crate::render::camera::OrbitCameraPlugin;
use crate::render::visuals::SceneVisualsPlugin;

pub mod camera;
pub mod visuals;

/// Bundles all rendering-related plugins for the simulation.
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OrbitCameraPlugin, SceneVisualsPlugin));
    }
}
