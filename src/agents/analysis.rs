use bevy::prelude::*;

use crate::app::SimulationState;
use crate::astro::black_hole::BlackHole;
use crate::astro::formation::FormationSettings;
use crate::astro::galaxy::Galaxy;
use crate::astro::star::Star;

use super::astro_agent::{AgentTelemetry, AstroAgent};
use super::events::{AstroReport, AstroReportLog};

#[derive(Resource, Default)]
pub struct AnalysisSchedule {
    pub last_agent_tick: u64,
    pub agent_interval: u64,
}

pub fn analyze_agents(
    sim_state: Res<SimulationState>,
    settings: Res<FormationSettings>,
    mut schedule: ResMut<AnalysisSchedule>,
    mut reports: ResMut<AstroReportLog>,
    mut agents: Query<(&mut AstroAgent, &mut AgentTelemetry, &Galaxy)>,
    black_holes: Query<&Transform, With<BlackHole>>,
    stars: Query<&Transform, With<Star>>,
) {
    if schedule.agent_interval == 0 {
        schedule.agent_interval = settings.galaxy_refresh_interval.max(4);
    }
    if sim_state.tick - schedule.last_agent_tick < schedule.agent_interval {
        return;
    }
    schedule.last_agent_tick = sim_state.tick;

    for (mut agent, mut telemetry, galaxy) in agents.iter_mut() {
        let region_radius = galaxy.radius.max(0.1);
        let bh_count = black_holes
            .iter()
            .filter(|t| (t.translation - galaxy.center).length() < region_radius)
            .count() as u32;
        let star_count = stars
            .iter()
            .filter(|t| (t.translation - galaxy.center).length() < region_radius)
            .count() as u32;

        let mass_change = (galaxy.total_mass - telemetry.last_mass).abs();
        let star_change = star_count.abs_diff(telemetry.last_star_count);
        let bh_change = bh_count.abs_diff(telemetry.last_black_holes);

        if telemetry.last_mass == 0.0 {
            telemetry.last_mass = galaxy.total_mass;
        }

        if mass_change > galaxy.total_mass * 0.05 || star_change > 0 || bh_change > 0 {
            let summary = format!(
                "Galaxy {} mass {:.2} (Δ{:.2}), stars {}, black holes {}",
                galaxy.id, galaxy.total_mass, mass_change, star_count, bh_count
            );
            reports.push(AstroReport {
                tick: sim_state.tick,
                agent_id: agent.id,
                agent_kind: agent.kind,
                summary,
            });
        }

        telemetry.last_mass = galaxy.total_mass;
        telemetry.last_star_count = star_count;
        telemetry.last_black_holes = bh_count;
        agent.tracked_region = Some(crate::agents::astro_agent::TrackedRegion {
            min: UVec3::ZERO,
            max: UVec3::splat(settings.region_size * 3),
        });
    }
}
