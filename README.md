# PRU Universe Bevy Simulation

A Bevy-based scientific visualization sandbox for exploring a **Precomputed Relational Universe (PRU)**. The PRU models space as a discrete lattice of cells, each carrying two information reservoirs (UA mass lock and UB geometry lock). A global tick updates all cells in lockstep, enabling deterministic experiments and visual overlays.

## Phase status
- **PHASE 1: Core Scaffold** — ✅ Complete (project scaffold, PRU lattice spawn, orbit camera, HUD + time controls).
- **PHASE 2: Basic Fields & Overlays** — ✅ Complete (density/curvature proxies, overlay toggles, metrics HUD).
- **PHASE 3: Macro Gravity & Large-Scale Structure** — ⏳ TODO.
- **PHASE 4: Stars, Black Holes, Galaxies** — ⏳ TODO.
- **PHASE 5: Time Control, Presets & Experiment Management** — ⏳ TODO.

## How to run
```bash
cargo run
```

## Controls
- **Camera**
  - Right-drag: orbit around the origin.
  - Middle-drag or Shift + Left-drag: pan.
  - Scroll: zoom.
- **Simulation**
  - Space: pause/resume.
  - `.` (period): single-step one tick.
  - `=` / `+`: speed up time scale.
  - `-`: slow down time scale.
  - `D`: toggle density overlay.
  - `C`: toggle curvature overlay.
- **HUD Buttons**
  - Pause/Resume, Step, Slower, Faster mirror the keyboard shortcuts.

## Current features (Phase 1)
- Initializes a configurable 3D PRU lattice with randomized UA/UB locks.
- Simple per-cell coloring seeded from lock values plus subtle animation.
- Orbit camera with lighting suitable for inspecting the lattice.
- HUD displaying tick counter, simulated time, time scale, and cell count with interactive time controls.
- UI uses Bevy's embedded default font, keeping the repository free of binary asset files.

## Phase 2 additions
- Derived per-cell scalar fields:
  - **local_density** based on UA mass lock.
  - **curvature_proxy** derived from UB lock neighbors.
- Overlay toggles to visualize density or curvature via color/emissive cues.
- Metrics HUD listing average/min/max density and average curvature.
- Tiny bar sparkline tracking average density over recent ticks.

## Extending the simulation
Future phases will add derived scalar fields (density, curvature), overlays, gravitational dynamics, astrophysical archetypes, and experiment presets. Systems are separated into `pru/` (simulation core), `render/` (camera + lighting), and `ui/` (controls) for incremental growth.
