//! Astro agents provide higher-level summaries of emergent structures.
//!
//! =========================
//! PHASE 4: STARS, BLACK HOLES, GALAXIES & ASTRO AGENTS
//! Status: IN PROGRESS
//! =========================

use bevy::prelude::*;

use crate::astro::formation::identify_galaxies;

pub mod analysis;
pub mod astro_agent;
pub mod events;

pub struct AgentsPlugin;

impl Plugin for AgentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<events::AstroReportLog>()
            .init_resource::<analysis::AnalysisSchedule>()
            .add_event::<events::GalaxyMergerEvent>()
            .add_systems(
                Update,
                (
                    astro_agent::attach_agents_to_galaxies.after(identify_galaxies),
                    analysis::analyze_agents.after(astro_agent::attach_agents_to_galaxies),
                ),
            );
    }
}
