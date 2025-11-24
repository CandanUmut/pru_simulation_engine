//! Core PRU (Precomputed Relational Universe) data structures and systems.
//!
//! The PRU models the universe as a discrete lattice of information-carrying
//! cells. Each cell owns two "locks" (UA and UB) that store inertial/mass and
//! geometric adjacency information. A global tick updates all cells in lockstep,
//! enabling deterministic, reproducible simulations.

pub mod cell;
pub mod rules;
pub mod universe;
