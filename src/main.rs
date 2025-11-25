//! Entry point for the PRU Universe Bevy simulation.
//!
//! This binary orchestrates the Bevy application and wires together the
//! core PRU simulation logic, rendering, and user interface layers.

// =========================
// PHASE 1: CORE SCAFFOLD
// Status: COMPLETE
// =========================
// PHASE 2: BASIC FIELDS & OVERLAYS
// Status: COMPLETE
// =========================
// PHASE 3: MACRO GRAVITY & LARGE-SCALE STRUCTURE
// Status: IN PROGRESS
// =========================
// PHASE 4: STARS, BLACK HOLES, GALAXIES & ASTRO AGENTS
// Status: IN PROGRESS
// =========================
// PHASE 5: TIME CONTROL, PRESETS & EXPERIMENT MANAGEMENT
// Status: TODO

mod agents;
mod app;
mod astro;
mod pru;
mod render;
mod ui;

fn main() {
    app::run_app();
}
