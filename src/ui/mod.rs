//! Minimal user interface for simulation control and status readout.

use bevy::prelude::*;

use crate::ui::controls::{
    keyboard_controls, setup_ui, update_density_history_bars, update_energy_text,
    update_gravity_labels, update_metrics_text, update_overlay_labels, update_status_text,
    update_ui_buttons,
};

pub mod controls;

/// Plugin encapsulating UI setup and interactions.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                keyboard_controls,
                update_ui_buttons,
                update_status_text,
                update_metrics_text,
                update_energy_text,
                update_density_history_bars,
                update_overlay_labels,
                update_gravity_labels,
            ),
        );
    }
}
