use bevy::prelude::*;

/// Plugin that spawns default lighting and reference helpers for the scene.
pub struct SceneVisualsPlugin;

impl Plugin for SceneVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(mut commands: Commands) {
    // Soft directional light to give depth to the PRU lattice.
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 15000.0,
                shadows_enabled: false,
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.35, 0.5, 0.0)),
            ..Default::default()
        },
        Name::new("Main Light"),
    ));

    // A second, dimmer light to soften silhouettes.
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 6000.0,
                shadows_enabled: false,
                ..Default::default()
            },
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.8, -1.2, 0.2)),
            ..Default::default()
        },
        Name::new("Fill Light"),
    ));
}
