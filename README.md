# PRU Universe Bevy Simulation

A Bevy-based scientific visualization sandbox for exploring a **Precomputed Relational Universe (PRU)**. The PRU models space as a discrete lattice of cells, each carrying two information reservoirs (UA mass lock and UB geometry lock). A global tick updates all cells in lockstep, enabling deterministic experiments and visual overlays.

## Phase status
- **PHASE 1: Core Scaffold** — ✅ Complete (project scaffold, PRU lattice spawn, orbit camera, HUD + time controls).
- **PHASE 2: Basic Fields & Overlays** — ✅ Complete (density/curvature proxies, overlay toggles, metrics HUD).
- **PHASE 3: Macro Gravity & Large-Scale Structure** — ⏳ In progress (naive gravity, motion, energy HUD).
- **PHASE 4: Stars, Black Holes, Galaxies & Astro Agents** — ⏳ In progress (formation rules, galaxy agents, agent panel).
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
- `G`: toggle gravity on/off.
- `M`: toggle between the naive N-body baseline and the relational lattice gravity mode.
- `[` / `]`: decrease/increase effective gravity.
  - `,` / `/`: decrease/increase damping.
  - `;` / `'`: decrease/increase softening length.
- **HUD Buttons**
  - Pause/Resume, Step, Slower, Faster mirror the keyboard shortcuts.
  - Gravity toggle plus adjustments for G, damping, and softening.

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

## Phase 3 additions
- Naive pairwise macro-gravity baseline and a relational lattice gravity mode driven by a precomputed neighbor kernel.
- Dynamic positions feed back into density/curvature overlays so clustering and voids become visible in real time.
- HUD energy diagnostics (kinetic, potential, total, ΔE/E0) for monitoring numerical drift.
- Interactive gravity controls via keyboard or HUD buttons (toggle, adjust G, damping, and softening).
- Known limitation: the naive baseline remains O(N²) for direct comparisons; the relational lattice mode uses a fixed stencil for better scaling but is still a first-pass approximation.

## Phase 4 additions
- Field-driven formation of **stars**, **black holes**, and coarse **galaxy halos** derived from density/curvature thresholds.
- **Astro Agents** that track galaxies, count nearby stars/black holes, and emit concise reports when mass or membership changes.
- An **Agents Panel** in the UI listing active agents alongside the latest high-level reports, so you can monitor the simulation without scanning every cell.

## Extending the simulation
Future phases will add derived scalar fields (density, curvature), overlays, gravitational dynamics, astrophysical archetypes, and experiment presets. Systems are separated into `pru/` (simulation core), `render/` (camera + lighting), and `ui/` (controls) for incremental growth.
