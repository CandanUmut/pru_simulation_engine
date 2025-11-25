use bevy::prelude::*;

/// A black hole, created when density & curvature exceed extreme thresholds.
#[derive(Component, Debug, Clone)]
pub struct BlackHole {
    pub mass: f32,
    pub radius: f32,
    pub spin: f32,
}

/// Simple visual hint for accretion disks.
pub fn animate_black_holes(time: Res<Time>, mut query: Query<(&BlackHole, &mut Transform)>) {
    let phase = time.elapsed_seconds();
    for (bh, mut transform) in query.iter_mut() {
        let wobble = (phase * 1.3 + bh.spin).sin() * 0.08;
        let scale = 1.0 + wobble;
        transform.scale = Vec3::new(
            bh.radius * scale,
            bh.radius * 0.5 * scale,
            bh.radius * scale,
        );
    }
}
