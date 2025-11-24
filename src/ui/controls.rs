use bevy::prelude::*;

use crate::app::SimulationState;
use crate::pru::universe::PruUniverse;

#[derive(Component)]
pub(crate) struct StatusText;

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

#[derive(Resource)]
pub(crate) struct UiColorScheme {
    normal: Color,
    hovered: Color,
    pressed: Color,
}

/// Build the UI tree: status text + control buttons.
pub fn setup_ui(mut commands: Commands) {
    commands.insert_resource(UiColorScheme {
        normal: Color::srgba(0.13, 0.15, 0.18, 0.8),
        hovered: Color::srgba(0.2, 0.22, 0.25, 0.9),
        pressed: Color::srgba(0.35, 0.35, 0.4, 0.95),
    });

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
                            spawn_button(row, "Pause", PauseButton, PauseLabel);
                            spawn_button(row, "Step", StepButton, ());
                            spawn_button(row, "Slower", SpeedButton { delta: -0.1 }, ());
                            spawn_button(row, "Faster", SpeedButton { delta: 0.1 }, ());
                        });
                });
        });
}

fn spawn_button<C1: Component, C2: Bundle>(
    parent: &mut ChildBuilder,
    label: &str,
    component: C1,
    label_component: C2,
) -> Entity {
    let colors = UiColorScheme {
        normal: Color::srgba(0.13, 0.15, 0.18, 0.8),
        hovered: Color::srgba(0.2, 0.22, 0.25, 0.9),
        pressed: Color::srgba(0.35, 0.35, 0.4, 0.95),
    };

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
pub fn keyboard_controls(mut sim_state: ResMut<SimulationState>, keys: Res<ButtonInput<KeyCode>>) {
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
}

/// React to UI button interactions and update button visuals.
pub fn update_ui_buttons(
    mut sim_state: ResMut<SimulationState>,
    colors: Res<UiColorScheme>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SpeedButton>,
            Option<&PauseButton>,
            Option<&StepButton>,
        ),
        Changed<Interaction>,
    >,
    mut pause_label: Query<&mut Text, With<PauseLabel>>,
) {
    for (interaction, mut color, speed_button, pause_button, step_button) in
        interaction_query.iter_mut()
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
