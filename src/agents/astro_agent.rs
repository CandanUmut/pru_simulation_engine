use bevy::prelude::*;

use crate::astro::galaxy::Galaxy;

/// Region tracked by an agent.
#[derive(Debug, Clone)]
pub struct TrackedRegion {
    pub min: UVec3,
    pub max: UVec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstroAgentKind {
    GalaxyAgent,
    ClusterAgent,
    BlackHoleAgent,
}

/// Higher-level observer that summarizes regional behavior.
#[derive(Component, Debug, Clone)]
pub struct AstroAgent {
    pub id: u32,
    pub kind: AstroAgentKind,
    pub name: Option<String>,
    pub tracked_region: Option<TrackedRegion>,
}

impl AstroAgent {
    pub fn new(id: u32, kind: AstroAgentKind, name: impl Into<Option<String>>) -> Self {
        Self {
            id,
            kind,
            name: name.into(),
            tracked_region: None,
        }
    }
}

/// Attach agents directly to galaxies to simplify bookkeeping.
pub fn attach_agents_to_galaxies(
    mut commands: Commands,
    galaxies: Query<(Entity, &Galaxy), Without<AstroAgent>>,
) {
    for (entity, galaxy) in galaxies.iter() {
        let name = format!("Galaxy Agent {}", galaxy.id);
        commands.entity(entity).insert((
            AstroAgent::new(galaxy.id, AstroAgentKind::GalaxyAgent, Some(name)),
            AgentTelemetry::default(),
        ));
    }
}

/// Rolling telemetry values used to detect changes and emit reports.
#[derive(Component, Debug, Clone, Default)]
pub struct AgentTelemetry {
    pub last_mass: f32,
    pub last_star_count: u32,
    pub last_black_holes: u32,
}
