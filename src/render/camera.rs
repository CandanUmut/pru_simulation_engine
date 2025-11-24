use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

/// Resource containing orbit camera parameters.
#[derive(Resource)]
pub struct OrbitCameraSettings {
    pub focus: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub rotate_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for OrbitCameraSettings {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 25.0,
            yaw: 0.6,
            pitch: 0.8,
            rotate_sensitivity: 0.005,
            pan_sensitivity: 0.015,
            zoom_sensitivity: 1.2,
        }
    }
}

/// Marker component for the orbiting camera.
#[derive(Component)]
pub struct OrbitCamera;

/// Plugin configuring camera spawning and input handling.
pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OrbitCameraSettings>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (camera_input, apply_camera_transform.after(camera_input)),
            );
    }
}

fn setup_camera(mut commands: Commands, settings: Res<OrbitCameraSettings>) {
    let mut transform = Transform::default();
    transform.translation = settings.focus + Vec3::new(0.0, settings.radius * 0.4, settings.radius);
    transform.look_at(settings.focus, Vec3::Y);

    commands.spawn((
        Camera3dBundle {
            transform,
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::FRAC_PI_4,
                near: 0.1,
                far: 5000.0,
                ..Default::default()
            }),
            ..Default::default()
        },
        OrbitCamera,
    ));
}

fn camera_input(
    time: Res<Time>,
    mut settings: ResMut<OrbitCameraSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let delta_time = time.delta_seconds();

    for ev in mouse_motion_events.read() {
        if mouse_buttons.pressed(MouseButton::Right) {
            settings.yaw -= ev.delta.x * settings.rotate_sensitivity;
            settings.pitch += ev.delta.y * settings.rotate_sensitivity;
            settings.pitch = settings.pitch.clamp(-1.5, 1.5);
        }

        let panning = mouse_buttons.pressed(MouseButton::Middle)
            || (keyboard.pressed(KeyCode::ShiftLeft) && mouse_buttons.pressed(MouseButton::Left));
        if panning {
            let yaw_rotation = Quat::from_rotation_y(settings.yaw);
            let right = yaw_rotation * Vec3::X;
            let up = Vec3::Y;
            let pan_multiplier = settings.radius * settings.pan_sensitivity * delta_time * 60.0;
            settings.focus -= right * ev.delta.x * pan_multiplier;
            settings.focus += up * ev.delta.y * pan_multiplier;
        }
    }

    for ev in mouse_wheel_events.read() {
        let scroll_amount = ev.y + ev.x;
        settings.radius -= scroll_amount * settings.zoom_sensitivity;
        settings.radius = settings.radius.clamp(2.0, 200.0);
    }
}

fn apply_camera_transform(
    settings: Res<OrbitCameraSettings>,
    mut query: Query<&mut Transform, With<OrbitCamera>>,
) {
    if settings.is_changed() {
        let rot = Quat::from_euler(EulerRot::YXZ, settings.yaw, settings.pitch, 0.0);
        let dir = rot * Vec3::new(0.0, 0.0, 1.0);
        let focus = settings.focus;
        for mut transform in query.iter_mut() {
            transform.translation = focus + dir * settings.radius;
            transform.look_at(focus, Vec3::Y);
        }
    }
}
