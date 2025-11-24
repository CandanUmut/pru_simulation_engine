use bevy::prelude::*;

/// Component representing a single PRU cell in the simulation lattice.
///
/// UA (mass_lock) and UB (geometry_lock) are simplified scalar placeholders for
/// the underlying information reservoirs described in the PRU thesis. Future
/// phases will derive additional fields (density, curvature) from these values.
#[derive(Component, Debug, Clone, Copy)]
pub struct PruCell {
    /// World-space position of the cell center.
    pub position: Vec3,
    /// Inertial / mass-related information bits.
    pub ua_mass_lock: f64,
    /// Geometric adjacency information bits.
    pub ub_geom_lock: f64,
}

impl PruCell {
    /// Convenience constructor for a new PRU cell.
    pub fn new(position: Vec3, ua_mass_lock: f64, ub_geom_lock: f64) -> Self {
        Self {
            position,
            ua_mass_lock,
            ub_geom_lock,
        }
    }
}
