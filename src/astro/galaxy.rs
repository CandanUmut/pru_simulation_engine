use bevy::prelude::*;

/// A galaxy as a higher-level structure, linked to a region of the PRU lattice.
#[derive(Component, Debug, Clone)]
pub struct Galaxy {
    pub id: u32,
    pub total_mass: f32,
    pub radius: f32,
    pub num_stars: u32,
    /// Barycenter in world coordinates.
    pub center: Vec3,
    pub region_key: UVec3,
}

#[derive(Resource, Default)]
pub struct GalaxyIdCounter {
    pub next_id: u32,
}

impl GalaxyIdCounter {
    pub fn next(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}
