use bevy::prelude::*;

/// A luminous star, emerging from high-density regions.
#[derive(Component, Debug, Clone)]
pub struct Star {
    pub mass: f32,
    pub radius: f32,
    pub temperature: f32,
    pub luminosity: f32,
}

/// Simple flicker animation to keep stars visually alive.
pub fn animate_stars(time: Res<Time>, mut query: Query<(&Star, &mut Transform)>) {
    let phase = time.elapsed_seconds();
    for (star, mut transform) in query.iter_mut() {
        let jitter = (phase * 2.3 + star.radius).sin() * 0.02;
        transform.scale = Vec3::splat((star.radius + jitter).max(0.05));
    }
}

pub fn star_color_from_temperature(temp: f32) -> Color {
    // Map temperature to a blue-white-yellow-red ramp.
    let normalized = (temp / 8000.0).clamp(0.0, 1.0);
    if normalized > 0.75 {
        Color::srgb(0.8, 0.9, 1.0)
    } else if normalized > 0.5 {
        Color::srgb(0.95, 0.95, 0.8)
    } else if normalized > 0.25 {
        Color::srgb(1.0, 0.85, 0.55)
    } else {
        Color::srgb(0.9, 0.45, 0.35)
    }
}
