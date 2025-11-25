//! Astrophysical archetypes and formation logic.
//!
//! =========================
//! PHASE 4: STARS, BLACK HOLES, GALAXIES & ASTRO AGENTS
//! Status: IN PROGRESS
//! =========================

use bevy::prelude::*;

use crate::pru::universe::compute_derived_fields;

pub mod black_hole;
pub mod formation;
pub mod galaxy;
pub mod star;

pub struct AstroPlugin;

impl Plugin for AstroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<formation::FormationSettings>()
            .init_resource::<formation::FormationSchedule>()
            .init_resource::<galaxy::GalaxyIdCounter>()
            .add_systems(
                Update,
                (
                    formation::spawn_stars_from_density,
                    formation::spawn_black_holes_from_density,
                    formation::identify_galaxies,
                    star::animate_stars,
                    black_hole::animate_black_holes,
                )
                    .chain()
                    .after(compute_derived_fields),
            );
    }
}
