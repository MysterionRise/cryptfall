# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Cryptfall is a Hades-inspired terminal roguelike built in Rust. It renders real-time action gameplay using half-block Unicode (▄) with true RGB colors via ANSI escape codes, targeting 30 FPS with differential rendering. The game is playable locally and over SSH.

**Current state:** Phase 2 (Combat Core) is complete — 3-wave arena with skeletons, ghost mages, projectiles, player health/death, particles, and hit feedback. Currently in the **Hardening Phase** (quality gate before Phase 3). Source code is built session-by-session following these plans.

## Build Commands

```bash
cargo build                # Debug build
cargo build --release      # Optimized build
cargo run                  # Run game locally
cargo test                 # Run all tests
cargo test -p engine       # Run engine crate tests only
cargo test -p game         # Run game crate tests only
cargo check                # Type-check without building
```

## Architecture

Cargo workspace with three crates:

- **`crates/engine/`** — Library crate. Terminal rendering (framebuffer, diff renderer), input handling (held-key inference via 150ms timeout), fixed-timestep game loop, sprite/animation system, tilemap, camera, collision (AABB), particles. Zero-allocation hot paths. This is the framework — no game logic here.
- **`crates/game/`** — Binary crate. Player, enemies, combat (hitboxes, knockback, i-frames), procedural dungeon generation, room templates, boon/upgrade system, weapons, meta-progression. Uses engine APIs only.
- **`crates/server/`** — Binary crate (Phase 5). SSH server via `russh`, per-connection game instances, ANSI input parsing, session management (max 50 concurrent).

Key dependencies: `crossterm` (terminal control), `russh`/`tokio` (SSH server).

## Session-Based Development

Development follows 32 numbered sessions across 5 phases. Each session has a prompt, success criteria, and technical notes in the phase markdown files:

- `00-quick-reference.md` — Session index, role preambles, git workflow
- `phase-0-foundation.md` — Engine bootstrap (framebuffer, renderer, input, game loop)
- `phase-1-sprite-engine.md` — Sprites, animation, tilemap, camera
- `phase-2-combat-core.md` — Combat, enemies, particles, hit feedback
- `phase-3-dungeon-structure.md` — Procedural dungeons, rooms, minimap, boss
- `phase-4-progression.md` — Boons, weapons, meta-progression, title screen
- `phase-5-ssh-distribution.md` — SSH server, CI/CD, deployment

## Role System

Each session specifies a role that constrains scope:

- **Engine Architect** — Framework, rendering, input. Write zero-allocation hot paths. Design APIs. Never write game logic.
- **Game Designer** — Combat, AI, rooms, progression. Think in game feel: hit-pause, screen shake, i-frames. Use engine APIs only. Never modify engine directly.
- **Visual Designer** — Sprites as Rust `const` RGB arrays, animations, particles, UI. NES aesthetic: chunky silhouettes, 3-4 colors, high contrast.
- **Infrastructure Engineer** — SSH server, CI/CD, Docker, cross-platform builds.
- **QA Lead** — Playtesting, edge cases, terminal compatibility, performance, balance.

## Cross-Role Coordination

The `docs/` directory contains coordination files between roles:
- `engine-requests.md`, `art-requests.md`, `known-bugs.md`, `design-decisions.md`, `tuning-values.md`, `playtest-notes.md`

## Git Workflow

Feature branches per session: `phase0/session-0.1-scaffold`. Tag at phase completion: `v0.0.1-phase0`.

## Technical Notes

- **Rendering**: Each terminal cell displays two pixels vertically using the half-block character (▄ U+2584) with foreground=bottom pixel, background=top pixel
- **Game loop**: Fixed 30 tick/sec timestep with accumulator pattern and interpolation for smooth rendering
- **Input**: Crossterm event polling with held-key inference (150ms release timeout) since terminals don't send key-up events
- **Differential rendering**: Only redraws changed cells by diffing current vs previous framebuffer
- **Terminal recovery after crash**: `reset`, `stty sane`, or `tput reset`

## Hardening Phase

Between Phase 2 and Phase 3, a structured quality gate addresses crash prevention, test coverage, architecture cleanup, and game-feel polish. See `phase-hardening.md` for the full sprint plan.

### Hardening Sprint Index

| Sprint | Focus | Lead Agent |
|--------|-------|------------|
| H0 | Critical crash prevention | tech-lead |
| H1 | Test infrastructure (20+ tests) | qa-engineer |
| H2 | Performance optimization (zero-alloc hot paths) | tech-lead |
| H3 | Architecture extraction (CombatSystem, WaveManager, tuning.rs) | principal-architect |
| H4 | AI polish (slime contact damage, telegraphs, constants) | ai-engineer |
| H5 | Game feel polish (transitions, animations, HUD feedback) | ux-game-designer |
| H6 | Code quality & CI (clippy pedantic, GitHub Actions, docs) | tech-lead |

### Team Roles (Agent Definitions)

Specialized agent definitions live in `.claude/agents/`:

| Agent | Role | Access |
|-------|------|--------|
| `product-owner.md` | Backlog, priorities, acceptance criteria | Read-only |
| `devils-advocate.md` | Challenge decisions, find flaws | Read-only |
| `qa-engineer.md` | Test coverage, bug detection | All (test files only) |
| `ai-engineer.md` | Enemy AI behaviors, state machines | All (enemies/ only) |
| `principal-architect.md` | System design, refactoring | All (plan mode) |
| `tech-lead.md` | Code quality, perf, CI, standards | All |
| `ux-game-designer.md` | Game feel, visual polish | All (game crate only) |
