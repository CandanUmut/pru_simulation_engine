use bevy::prelude::*;

use crate::agents::astro_agent::{AstroAgent, AstroAgentKind};
use crate::agents::events::AstroReportLog;
use crate::astro::galaxy::Galaxy;

#[derive(Component)]
pub struct AgentListText;

#[derive(Component)]
pub struct AgentReportText;

pub fn setup_agent_panel(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(16.0),
                top: Val::Px(12.0),
                width: Val::Px(320.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                ..Default::default()
            },
            background_color: Color::srgba(0.04, 0.04, 0.08, 0.7).into(),
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "Astro Agents",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.85, 0.9, 1.0),
                    ..Default::default()
                },
            ));

            root.spawn((
                TextBundle::from_sections([TextSection::new(
                    "Agents loading...",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.8, 0.85, 0.95),
                        ..Default::default()
                    },
                )]),
                AgentListText,
            ));

            root.spawn((
                TextBundle::from_sections([TextSection::new(
                    "Recent Events",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.85, 0.9, 1.0),
                        ..Default::default()
                    },
                )]),
                AgentReportText,
            ));
        });
}

pub fn update_agent_panel(
    agents: Query<(&AstroAgent, Option<&Galaxy>)>,
    reports: Res<AstroReportLog>,
    mut list_text: Query<&mut Text, With<AgentListText>>,
    mut report_text: Query<&mut Text, With<AgentReportText>>,
) {
    if let Ok(mut text) = list_text.get_single_mut() {
        let mut lines = Vec::new();
        for (agent, galaxy) in agents.iter() {
            let summary = match agent.kind {
                AstroAgentKind::GalaxyAgent => {
                    if let Some(galaxy) = galaxy {
                        format!(
                            "#{} Galaxy mass {:.1}, stars {}, r={:.1}",
                            galaxy.id, galaxy.total_mass, galaxy.num_stars, galaxy.radius
                        )
                    } else {
                        format!("#{} Galaxy agent", agent.id)
                    }
                }
                AstroAgentKind::ClusterAgent => format!("#{} Cluster agent", agent.id),
                AstroAgentKind::BlackHoleAgent => format!("#{} Black hole agent", agent.id),
            };
            lines.push(summary);
        }

        if lines.is_empty() {
            lines.push("No agents yet".to_string());
        }

        text.sections = vec![TextSection::new(
            lines.join("\n"),
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.78, 0.84, 0.95),
                ..Default::default()
            },
        )];
    }

    if let Ok(mut text) = report_text.get_single_mut() {
        let mut lines = vec!["Recent Events".to_string()];
        for report in reports.reports.iter().rev().take(5) {
            lines.push(format!("[{}] {}", report.tick, report.summary));
        }
        text.sections = vec![TextSection::new(
            lines.join("\n"),
            TextStyle {
                font_size: 13.0,
                color: Color::srgb(0.85, 0.9, 1.0),
                ..Default::default()
            },
        )];
    }
}
