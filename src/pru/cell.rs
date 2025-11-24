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
    /// Discrete lattice coordinates for neighborhood lookups.
    pub grid_coords: UVec3,
    /// Inertial / mass-related information bits.
    pub ua_mass_lock: f64,
    /// Geometric adjacency information bits.
    pub ub_geom_lock: f64,
}

impl PruCell {
    /// Convenience constructor for a new PRU cell.
    pub fn new(position: Vec3, grid_coords: UVec3, ua_mass_lock: f64, ub_geom_lock: f64) -> Self {
        Self {
            position,
            grid_coords,
            ua_mass_lock,
            ub_geom_lock,
        }
    }
}

/// Dynamical properties for a PRU cell used by the macro-gravity integrator.
#[derive(Component, Debug, Clone, Copy)]
pub struct PruDynamics {
    /// Effective inertial mass derived from UA.
    pub mass: f32,
    /// Current velocity in world units per second.
    pub velocity: Vec3,
    /// Current acceleration accumulated from gravity or other rules.
    pub acceleration: Vec3,
}

impl Default for PruDynamics {
    fn default() -> Self {
        Self {
            mass: 1.0,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
        }
    }
}

/// Derived scalar fields computed from a cell's locks and local neighborhood.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct DerivedFields {
    /// Proxy for mass/density derived from UA.
    pub local_density: f32,
    /// Curvature-like proxy derived from UB relative to neighbors.
    pub curvature_proxy: f32,
}
