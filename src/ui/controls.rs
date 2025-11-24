use bevy::prelude::*;

use crate::app::SimulationState;
use crate::pru::gravity::{GravityParams, SimulationEnergy};
use crate::pru::universe::{FieldMetrics, PruUniverse};

pub const DENSITY_BAR_COUNT: usize = 40;

#[derive(Component)]
pub(crate) struct StatusText;

#[derive(Component)]
pub(crate) struct MetricsText;

#[derive(Component)]
pub(crate) struct EnergyText;

#[derive(Component)]
pub(crate) struct PauseButton;

#[derive(Component)]
pub(crate) struct PauseLabel;

#[derive(Component)]
pub(crate) struct StepButton;

#[derive(Component)]
pub(crate) struct SpeedButton {
    delta: f32,
}

#[derive(Component)]
pub(crate) struct DensityToggle;

#[derive(Component)]
pub(crate) struct DensityLabel;

#[derive(Component)]
pub(crate) struct CurvatureToggle;

#[derive(Component)]
pub(crate) struct CurvatureLabel;

#[derive(Component)]
pub(crate) struct GravityToggle;

#[derive(Component)]
pub(crate) struct GravityLabel;

#[derive(Component)]
pub(crate) struct GravityParamsText;

#[derive(Component)]
pub(crate) struct GravityAdjustButton {
    delta: f32,
}

#[derive(Component)]
pub(crate) struct DampingAdjustButton {
    delta: f32,
}

#[derive(Component)]
pub(crate) struct SofteningAdjustButton {
    delta: f32,
}

#[derive(Component)]
pub(crate) struct DensityBar {
    pub index: usize,
}

#[derive(Resource, Clone)]
pub(crate) struct UiColorScheme {
    normal: Color,
    hovered: Color,
    pressed: Color,
}

/// Visualization toggles for scalar overlays.
#[derive(Resource, Clone, Copy)]
pub struct VisualModeSettings {
    pub show_density_coloring: bool,
    pub show_curvature_coloring: bool,
}

impl Default for VisualModeSettings {
    fn default() -> Self {
        Self {
            show_density_coloring: true,
            show_curvature_coloring: false,
        }
    }
}

impl VisualModeSettings {
    pub fn toggle_density(&mut self) {
        self.show_density_coloring = !self.show_density_coloring;
        if self.show_density_coloring {
            self.show_curvature_coloring = false;
        }
    }

    pub fn toggle_curvature(&mut self) {
        self.show_curvature_coloring = !self.show_curvature_coloring;
        if self.show_curvature_coloring {
            self.show_density_coloring = false;
        }
    }
}

/// Build the UI tree: status text + control buttons.
pub fn setup_ui(mut commands: Commands) {
    let colors = UiColorScheme {
        normal: Color::srgba(0.13, 0.15, 0.18, 0.8),
        hovered: Color::srgba(0.2, 0.22, 0.25, 0.9),
        pressed: Color::srgba(0.35, 0.35, 0.4, 0.95),
    };
    commands.insert_resource(colors.clone());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(12.0)),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..Default::default()
                    },
                    background_color: Color::srgba(0.05, 0.05, 0.08, 0.5).into(),
                    ..Default::default()
                })
                .with_children(|column| {
                    column.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                "PRU Universe Simulation\n",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.95, 1.0),
                                    ..Default::default()
                                },
                            ),
                            TextSection::new(
                                "Status text",
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::srgb(0.8, 0.9, 1.0),
                                    ..Default::default()
                                },
                            ),
                        ]),
                        StatusText,
                    ));

                    column.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                "Derived Fields\n",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::srgb(0.9, 0.95, 1.0),
                                    ..Default::default()
                                },
                            ),
                            TextSection::new(
                                "Metrics",
                                TextStyle {
                                    font_size: 14.0,
                                    color: Color::srgb(0.8, 0.9, 1.0),
                                    ..Default::default()
                                },
                            ),
                        ]),
                        MetricsText,
                    ));

                    column.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                "Energy Diagnostics\n",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::srgb(0.9, 0.95, 1.0),
                                    ..Default::default()
                                },
                            ),
                            TextSection::new(
                                "Energy values",
                                TextStyle {
                                    font_size: 14.0,
                                    color: Color::srgb(0.8, 0.9, 1.0),
                                    ..Default::default()
                                },
                            ),
                        ]),
                        EnergyText,
                    ));

                    column
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|row| {
                            spawn_button(row, "Pause", PauseButton, PauseLabel, &colors);
                            spawn_button(row, "Step", StepButton, (), &colors);
                            spawn_button(row, "Slower", SpeedButton { delta: -0.1 }, (), &colors);
                            spawn_button(row, "Faster", SpeedButton { delta: 0.1 }, (), &colors);
                        });

                    column
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|row| {
                            spawn_button(
                                row,
                                "Density Overlay",
                                DensityToggle,
                                DensityLabel,
                                &colors,
                            );
                            spawn_button(
                                row,
                                "Curvature Overlay",
                                CurvatureToggle,
                                CurvatureLabel,
                                &colors,
                            );
                        });

                    column
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(8.0),
                                ..Default::default()
                            },
                            background_color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|row| {
                            spawn_button(row, "Gravity", GravityToggle, GravityLabel, &colors);
                            spawn_button(
                                row,
                                "G -",
                                GravityAdjustButton { delta: -0.05 },
                                (),
                                &colors,
                            );
                            spawn_button(
                                row,
                                "G +",
                                GravityAdjustButton { delta: 0.05 },
                                (),
                                &colors,
                            );
                            spawn_button(
                                row,
                                "Damp -",
                                DampingAdjustButton { delta: -0.002 },
                                (),
                                &colors,
                            );
                            spawn_button(
                                row,
                                "Damp +",
                                DampingAdjustButton { delta: 0.002 },
                                (),
                                &colors,
                            );
                            spawn_button(
                                row,
                                "Soft -",
                                SofteningAdjustButton { delta: -0.02 },
                                (),
                                &colors,
                            );
                            spawn_button(
                                row,
                                "Soft +",
                                SofteningAdjustButton { delta: 0.02 },
                                (),
                                &colors,
                            );
                        });

                    column.spawn((
                        TextBundle::from_section(
                            "Gravity Params",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgb(0.8, 0.9, 1.0),
                                ..Default::default()
                            },
                        ),
                        GravityParamsText,
                    ));

                    column
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(260.0),
                                height: Val::Px(80.0),
                                align_items: AlignItems::FlexEnd,
                                column_gap: Val::Px(2.0),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                ..Default::default()
                            },
                            background_color: Color::srgba(0.02, 0.03, 0.05, 0.6).into(),
                            ..Default::default()
                        })
                        .with_children(|graph| {
                            for i in 0..DENSITY_BAR_COUNT {
                                graph.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(4.0),
                                            height: Val::Px(6.0),
                                            margin: UiRect::horizontal(Val::Px(1.0)),
                                            ..Default::default()
                                        },
                                        background_color: Color::srgb(0.3, 0.5, 0.9).into(),
                                        ..Default::default()
                                    },
                                    DensityBar { index: i },
                                ));
                            }
                        });
                });
        });
}

fn spawn_button<C1: Component, C2: Bundle>(
    parent: &mut ChildBuilder,
    label: &str,
    component: C1,
    label_component: C2,
    colors: &UiColorScheme,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..Default::default()
                },
                background_color: colors.normal.into(),
                border_color: BorderColor(Color::srgba(0.5, 0.6, 0.7, 0.6)),
                ..Default::default()
            },
            component,
        ))
        .with_children(|button| {
            button.spawn((
                TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.9, 0.95, 1.0),
                        ..Default::default()
                    },
                ),
                label_component,
            ));
        })
        .id()
}

/// Keyboard shortcuts mirroring the UI controls.
pub fn keyboard_controls(
    mut sim_state: ResMut<SimulationState>,
    mut modes: ResMut<VisualModeSettings>,
    mut gravity: ResMut<GravityParams>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        sim_state.toggle();
    }
    if keys.just_pressed(KeyCode::Period) {
        sim_state.step_once();
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::NumpadSubtract) {
        sim_state.adjust_speed(-0.1);
    }
    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::NumpadAdd) {
        sim_state.adjust_speed(0.1);
    }
    if keys.just_pressed(KeyCode::KeyD) {
        modes.toggle_density();
    }
    if keys.just_pressed(KeyCode::KeyC) {
        modes.toggle_curvature();
    }
    if keys.just_pressed(KeyCode::KeyG) {
        gravity.enabled = !gravity.enabled;
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        gravity.g_effective = (gravity.g_effective - 0.05).max(0.0);
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        gravity.g_effective = (gravity.g_effective + 0.05).clamp(0.0, 5.0);
    }
    if keys.just_pressed(KeyCode::Comma) {
        gravity.damping = (gravity.damping - 0.002).max(0.0);
    }
    if keys.just_pressed(KeyCode::Slash) {
        gravity.damping = (gravity.damping + 0.002).min(1.0);
    }
    if keys.just_pressed(KeyCode::Semicolon) {
        gravity.softening_length = (gravity.softening_length - 0.02).max(0.01);
    }
    if keys.just_pressed(KeyCode::Quote) {
        gravity.softening_length = (gravity.softening_length + 0.02).min(2.0);
    }
}

/// React to UI button interactions and update button visuals.
pub fn update_ui_buttons(
    mut sim_state: ResMut<SimulationState>,
    mut modes: ResMut<VisualModeSettings>,
    mut gravity: ResMut<GravityParams>,
    colors: Res<UiColorScheme>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SpeedButton>,
            Option<&PauseButton>,
            Option<&StepButton>,
            Option<&DensityToggle>,
            Option<&CurvatureToggle>,
            Option<&GravityToggle>,
            Option<&GravityAdjustButton>,
            Option<&DampingAdjustButton>,
            Option<&SofteningAdjustButton>,
        ),
        Changed<Interaction>,
    >,
    mut pause_label: Query<&mut Text, With<PauseLabel>>,
) {
    for (
        interaction,
        mut color,
        speed_button,
        pause_button,
        step_button,
        density_toggle,
        curvature_toggle,
        gravity_toggle,
        gravity_adjust,
        damping_adjust,
        softening_adjust,
    ) in interaction_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.pressed.into();

                if pause_button.is_some() {
                    sim_state.toggle();
                } else if let Some(speed_button) = speed_button {
                    sim_state.adjust_speed(speed_button.delta);
                } else if step_button.is_some() {
                    sim_state.step_once();
                } else if density_toggle.is_some() {
                    modes.toggle_density();
                } else if curvature_toggle.is_some() {
                    modes.toggle_curvature();
                } else if gravity_toggle.is_some() {
                    gravity.enabled = !gravity.enabled;
                } else if let Some(adj) = gravity_adjust {
                    gravity.g_effective = (gravity.g_effective + adj.delta).clamp(0.0, 5.0);
                } else if let Some(adj) = damping_adjust {
                    gravity.damping = (gravity.damping + adj.delta).clamp(0.0, 1.0);
                } else if let Some(adj) = softening_adjust {
                    gravity.softening_length =
                        (gravity.softening_length + adj.delta).clamp(0.01, 3.0);
                }
            }
            Interaction::Hovered => {
                *color = colors.hovered.into();
            }
            Interaction::None => {
                *color = colors.normal.into();
            }
        }
    }

    if let Ok(mut text) = pause_label.get_single_mut() {
        text.sections[0].value = if sim_state.running {
            "Pause".to_string()
        } else {
            "Resume".to_string()
        };
    }
}

/// Refresh the HUD text showing simulation counters.
pub fn update_status_text(
    sim_state: Res<SimulationState>,
    universe: Option<Res<PruUniverse>>,
    mut query: Query<&mut Text, With<StatusText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let cell_count = universe.as_ref().map(|u| u.total_cells).unwrap_or(0);
        text.sections[1].value = format!(
            "State: {}\nTick: {}\nSim time: {:.2} s\nTime scale: {:.2}x\nCells: {}",
            if sim_state.running {
                "Running"
            } else {
                "Paused"
            },
            sim_state.tick,
            sim_state.simulation_time,
            sim_state.time_scale,
            cell_count
        );
    }
}

/// Show density/curvature metrics and a tiny sparkline style bar chart.
pub fn update_metrics_text(
    metrics: Res<FieldMetrics>,
    mut text_query: Query<&mut Text, With<MetricsText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[1].value = format!(
            "Avg density: {:.3}\nMin/Max density: {:.3} / {:.3}\nAvg curvature: {:.3}",
            metrics.avg_density, metrics.min_density, metrics.max_density, metrics.avg_curvature,
        );
    }
}

pub fn update_density_history_bars(
    metrics: Res<FieldMetrics>,
    mut bar_query: Query<(&mut Style, &mut BackgroundColor, &DensityBar)>,
) {
    if !metrics.is_changed() {
        return;
    }

    let mut samples: Vec<f32> = metrics.density_history.iter().cloned().collect();
    while samples.len() < DENSITY_BAR_COUNT {
        samples.insert(0, 0.0);
    }
    let max_sample = samples
        .iter()
        .cloned()
        .fold(0.0001f32, |a, b| a.max(b.abs()));

    for (mut style, mut color, bar) in bar_query.iter_mut() {
        if let Some(sample) = samples.iter().rev().nth(bar.index) {
            let normalized = (sample / max_sample).clamp(0.0, 1.0);
            style.height = Val::Px(6.0 + normalized * 60.0);
            *color = Color::srgb(0.25 + normalized * 0.5, 0.6, 0.95).into();
        }
    }
}

pub fn update_overlay_labels(
    modes: Res<VisualModeSettings>,
    mut density_label: Query<&mut Text, With<DensityLabel>>,
    mut curvature_label: Query<&mut Text, With<CurvatureLabel>>,
) {
    if let Ok(mut text) = density_label.get_single_mut() {
        text.sections[0].value = if modes.show_density_coloring {
            "Density Overlay (On)".to_string()
        } else {
            "Density Overlay (Off)".to_string()
        };
    }

    if let Ok(mut text) = curvature_label.get_single_mut() {
        text.sections[0].value = if modes.show_curvature_coloring {
            "Curvature Overlay (On)".to_string()
        } else {
            "Curvature Overlay (Off)".to_string()
        };
    }
}

/// Update on-screen gravity toggles and parameter readout.
pub fn update_gravity_labels(
    params: Res<GravityParams>,
    mut gravity_label: Query<&mut Text, With<GravityLabel>>,
    mut params_text: Query<&mut Text, With<GravityParamsText>>,
) {
    if let Ok(mut text) = gravity_label.get_single_mut() {
        text.sections[0].value = if params.enabled {
            "Gravity (On)".to_string()
        } else {
            "Gravity (Off)".to_string()
        };
    }

    if let Ok(mut text) = params_text.get_single_mut() {
        text.sections[0].value = format!(
            "G_eff: {:.2}\nSoftening: {:.3}\nDamping: {:.4}\nMax Accel: {:.0}",
            params.g_effective, params.softening_length, params.damping, params.max_acceleration
        );
    }
}

/// Show kinetic/potential/total energy and relative drift.
pub fn update_energy_text(
    energy: Res<SimulationEnergy>,
    mut text_query: Query<&mut Text, With<EnergyText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let drift_str = energy
            .relative_drift
            .map(|d| format!("{:.2e}", d))
            .unwrap_or_else(|| "n/a".to_string());

        text.sections[1].value = format!(
            "Kinetic: {:>10.4}\nPotential: {:>10.4}\nTotal: {:>10.4}\nΔE/E0: {}",
            energy.kinetic, energy.potential, energy.total, drift_str
        );
    }
}
