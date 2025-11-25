use bevy::prelude::*;

use super::astro_agent::AstroAgentKind;

#[derive(Resource, Default)]
pub struct AstroReportLog {
    pub reports: Vec<AstroReport>,
    pub max_reports: usize,
}

impl AstroReportLog {
    pub fn push(&mut self, report: AstroReport) {
        if self.max_reports == 0 {
            self.max_reports = 128;
        }
        self.reports.push(report);
        if self.reports.len() > self.max_reports {
            let overflow = self.reports.len() - self.max_reports;
            self.reports.drain(0..overflow);
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstroReport {
    pub tick: u64,
    pub agent_id: u32,
    pub agent_kind: AstroAgentKind,
    pub summary: String,
}

#[derive(Event)]
pub struct GalaxyMergerEvent {
    pub a: u32,
    pub b: u32,
}
