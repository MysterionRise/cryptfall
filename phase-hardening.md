# Hardening Phase: Quality Gate Before Phase 3

## Overview
**Duration:** 1–2 focused sessions
**Goal:** Crash-proof the codebase, add test coverage, extract systems from main.rs, and polish game feel — all without changing gameplay behavior.
**Prerequisite:** Phase 2 complete (3-wave arena, skeletons, ghost mages, projectiles, player health/death, particles, hit feedback)
**Gate for:** Phase 3 (Dungeon Structure) — the architecture must be clean enough to support multi-room dungeons.

---

## Team Roles

| Role | Agent File | Model | Scope |
|------|-----------|-------|-------|
| Product Owner | `.claude/agents/product-owner.md` | opus | Read-only. Backlogs, priorities, acceptance criteria |
| Devil's Advocate | `.claude/agents/devils-advocate.md` | opus | Read-only. Critiques, risk assessments |
| QA Engineer | `.claude/agents/qa-engineer.md` | sonnet | Test files only. Writes `#[test]` functions |
| AI Engineer | `.claude/agents/ai-engineer.md` | sonnet | `enemies/` only. AI behaviors and tuning |
| Principal Architect | `.claude/agents/principal-architect.md` | opus | Full access, plan mode. Refactoring only |
| Tech Lead | `.claude/agents/tech-lead.md` | sonnet | Full access. Fixes, never features |
| UX/Game Designer | `.claude/agents/ux-game-designer.md` | sonnet | Game crate only. Feel and polish |

---

## Sprint H0 — Critical Crash Prevention

**Lead:** tech-lead
**Review:** devils-advocate
**Priority:** P0 — Must complete before any other work

### Tasks

#### H0.1: Replace `is_multiple_of()` with stable equivalent
- **File:** `crates/game/src/main.rs` (lines ~226, ~598, ~612)
- **Issue:** `is_multiple_of()` is a nightly-only API. Code won't compile on stable Rust.
- **Fix:** Replace `x.is_multiple_of(n)` with `x % n == 0`
- **Acceptance:** `cargo +stable build` compiles cleanly

#### H0.2: Add minimum terminal size guard
- **File:** `crates/engine/src/lib.rs` (around line 88)
- **Issue:** Resizing terminal to 0×0 causes panic in framebuffer allocation
- **Fix:** Clamp terminal size to minimum 20×10 (or skip frame if too small)
- **Acceptance:** Resize terminal rapidly during gameplay without panic

#### H0.3: Assert non-empty animation frames
- **File:** `crates/engine/src/animation.rs` (around line 65)
- **Issue:** Creating an animation with empty frames array causes index-out-of-bounds panic
- **Fix:** Return early / use `.get()` with fallback, or assert in constructor with clear error
- **Acceptance:** `Animation::new(vec![])` does not panic

#### H0.4: Guard `render_hearts` for negative HP
- **File:** `crates/game/src/hud.rs` (around line 78)
- **Issue:** If HP goes negative (race condition or damage > remaining HP), unsigned underflow panic
- **Fix:** Clamp HP to 0 before rendering: `let hp = hp.max(0) as usize`
- **Acceptance:** `render_hearts(-1, 5)` doesn't panic

#### H0.5: Remove unused import
- **File:** `crates/game/src/projectile.rs` (line 2)
- **Issue:** `engine::color::Color` imported but unused
- **Fix:** Remove the line
- **Acceptance:** `cargo clippy` clean for this file

---

## Sprint H1 — Test Infrastructure

**Lead:** qa-engineer
**Parallel with:** H0 (no file conflicts)
**Priority:** P1

### Tasks

#### H1.1: AABB collision tests
- **File:** `crates/engine/src/collision.rs` — add `#[cfg(test)] mod tests`
- **Tests:**
  - Two overlapping rects → true
  - Two non-overlapping rects → false
  - Edge-touching rects → define and test expected behavior
  - Zero-size rect → no panic
  - Negative coordinates → correct behavior
  - Identical rects → true
- **Acceptance:** `cargo test -p engine` passes with 6+ collision tests

#### H1.2: Animation state machine tests
- **File:** `crates/engine/src/animation.rs` — add `#[cfg(test)] mod tests`
- **Tests:**
  - Frame index advances on tick
  - Looping animation wraps to frame 0
  - Non-looping animation sets finished flag
  - Frame index never exceeds frame count
  - Empty animation handling (per H0.3 fix)
- **Acceptance:** `cargo test -p engine` passes with 5+ animation tests

#### H1.3: Particle system tests
- **File:** `crates/engine/src/particle.rs` — add `#[cfg(test)] mod tests`
- **Tests:**
  - Particle lifetime counts down to zero
  - Dead particles are removed on update
  - Burst spawns correct number of particles
  - Particle count never exceeds cap (500)
  - Velocity and gravity affect position
- **Acceptance:** `cargo test -p engine` passes with 5+ particle tests

#### H1.4: Player state machine tests
- **File:** `crates/game/src/player.rs` — add `#[cfg(test)] mod tests`
- **Tests:**
  - Taking damage transitions to Hit state
  - Hit state returns to Idle after duration
  - Death transition when HP reaches 0
  - I-frames prevent damage during active
  - HP never goes below 0
- **Acceptance:** `cargo test -p game` passes with 5+ player tests

#### H1.5: Wave progression tests
- **File:** `crates/game/src/` — test module or integration test
- **Tests:**
  - Wave starts with correct enemy count
  - Killing all enemies clears the wave
  - Wave 3 clear triggers victory
  - Wave transitions have correct timing
- **Acceptance:** `cargo test -p game` passes with 4+ wave tests

---

## Sprint H2 — Performance Optimization

**Lead:** tech-lead
**Review:** principal-architect
**Priority:** P2

### Tasks

#### H2.1: Pre-allocate projectile vectors
- **File:** `crates/game/src/projectile.rs` (lines ~106-107)
- **Issue:** Trail and impact particle vectors allocate on every projectile update
- **Fix:** Use `Vec::with_capacity(16)` for trails, reuse across frames
- **Acceptance:** No new allocations visible in hot path (verified by code review)

#### H2.2: Pre-allocate hit results
- **File:** `crates/game/src/projectile.rs` (line ~130)
- **Issue:** Hit result collection allocates per frame
- **Fix:** `Vec::with_capacity(8)` — max 8 simultaneous hits is more than enough
- **Acceptance:** Allocation moved to initialization, not per-frame

#### H2.3: Replace digit extraction allocation
- **File:** `crates/game/src/hud.rs` (lines ~61-72)
- **Issue:** `get_digits()` returns a `Vec<u8>` per frame for score/wave display
- **Fix:** Use a fixed `[u8; 5]` array with a length counter (max 5 digits = 99999)
- **Acceptance:** No heap allocation in `get_digits`

#### H2.4: Reduce redundant particle bounds checks
- **File:** `crates/engine/src/particle.rs` (lines ~82-83)
- **Issue:** Color array indexed twice with separate bounds checks
- **Fix:** Single bounds-checked access with local variable
- **Acceptance:** Code review confirms single access pattern

---

## Sprint H3 — Architecture Extraction

**Lead:** principal-architect
**Review:** product-owner, devils-advocate
**Depends on:** H0, H2 (needs clean, optimized main.rs)
**Priority:** P1

### Tasks

#### H3.1: Extract `CombatSystem`
- **Source:** `crates/game/src/main.rs` (~lines 460-570)
- **Destination:** `crates/game/src/combat.rs`
- **Public API:**
  ```rust
  pub struct CombatSystem { /* hitbox state, damage tracking */ }
  impl CombatSystem {
      pub fn new() -> Self;
      pub fn update(&mut self, player: &mut Player, enemies: &mut [Enemy], particles: &mut ParticleSystem, dt: f32);
  }
  ```
- **Acceptance:** Main.rs no longer contains hitbox/damage logic. `cargo run` plays identically.

#### H3.2: Extract `WaveManager`
- **Source:** `crates/game/src/main.rs`
- **Destination:** `crates/game/src/waves.rs`
- **Public API:**
  ```rust
  pub struct WaveManager {
      pub current_wave: u32,
      pub victory: bool,
  }
  impl WaveManager {
      pub fn new() -> Self;
      pub fn update(&mut self, enemies: &[Enemy], dt: f32);
      pub fn spawn_wave(&self) -> Vec<Enemy>;
      pub fn is_wave_clear(&self) -> bool;
  }
  ```
- **Acceptance:** Main.rs no longer contains wave state/spawning logic. Wave progression works identically.

#### H3.3: Centralize tuning constants
- **Destination:** `crates/game/src/tuning.rs`
- **Contents:** All `const` magic numbers from main.rs, player.rs, enemies/, projectile.rs
- **Organization:** Grouped by category with doc comments
  ```rust
  /// Player movement speed in pixels per second
  pub const PLAYER_SPEED: f32 = 120.0;
  ```
- **Acceptance:** All gameplay-affecting constants live in `tuning.rs`. No magic numbers in logic code.

---

## Sprint H4 — AI Polish

**Lead:** ai-engineer
**Review:** ux-game-designer
**Parallel with:** H2
**Priority:** P2

### Tasks

#### H4.1: Implement slime contact damage
- **File:** `crates/game/src/enemies/slime.rs`
- **Behavior:** When slime overlaps player hitbox, deal 1 damage with 0.5s cooldown
- **Implementation:** `ContactDamage` component or behavior in slime update
- **Acceptance:** Walking into a slime takes damage. Can't take contact damage more than 2x/sec.

#### H4.2: Add skeleton attack telegraph particles
- **File:** `crates/game/src/enemies/skeleton.rs`
- **Behavior:** During WindUp state, emit warning particles (red/orange sparks near weapon)
- **Acceptance:** Player can see the attack coming before the hitbox activates

#### H4.3: Improve ghost aim cancel behavior
- **File:** `crates/game/src/enemies/ghost.rs`
- **Behavior:** Ghost cancels aim state if player dashes behind it or moves beyond max range
- **Acceptance:** Ghost doesn't fire at thin air after player dodges

#### H4.4: Name magic distance constants
- **File:** `crates/game/src/enemies/ghost.rs` (line ~112)
- **Issue:** `TOO_CLOSE * 0.7` is unclear
- **Fix:** `const AIM_CANCEL_DISTANCE: f32 = TOO_CLOSE * 0.7;` with doc comment
- **Acceptance:** No raw multiplied constants in distance checks

---

## Sprint H5 — Game Feel Polish

**Lead:** ux-game-designer
**Review:** product-owner
**Depends on:** H3 (wave system extracted)
**Priority:** P2

### Tasks

#### H5.1: Wave transition feedback
- **Behavior:** On wave clear, display "WAVE CLEAR" text for 1.5s with screen flash
- **Implementation:** Overlay text in HUD, brief white flash on framebuffer
- **Acceptance:** Killing the last enemy produces a visible celebration moment

#### H5.2: Enemy spawn animation
- **Behavior:** New wave enemies fade in over 0.3s (alpha ramp from 0% to 100%)
- **Implementation:** Spawn with `fade_timer: 0.3`, reduce per frame, multiply sprite colors
- **Acceptance:** Enemies don't pop in instantly. Visible fade-in on wave start.

#### H5.3: Tune i-frame flash frequency
- **Current:** I-frame visual flash may be too fast to read
- **Test:** Compare 10Hz vs 15Hz toggle rate
- **Acceptance:** Player can clearly see when they're invulnerable vs vulnerable

#### H5.4: Dash cooldown indicator
- **Behavior:** Brief HUD or visual cue when dash becomes available again
- **Implementation:** Flash player sprite or show small UI element when dash cooldown resets
- **Acceptance:** Player knows when they can dash again without counting frames

#### H5.5: Heart loss animation
- **Current:** Hearts swap instantly from full to empty on damage
- **Fix:** Hearts flash white then shrink/bounce on damage
- **Acceptance:** Taking damage has visible HUD feedback beyond the sprite flash

---

## Sprint H6 — Code Quality & CI

**Lead:** tech-lead
**Review:** qa-engineer
**Parallel with:** H5
**Priority:** P2

### Tasks

#### H6.1: Add clippy pedantic to engine crate
- **File:** `crates/engine/src/lib.rs`
- **Add:** `#![warn(clippy::pedantic)]`
- **Fix:** All resulting warnings, or add justified `#[allow]` with comments
- **Acceptance:** `cargo clippy -p engine` is clean

#### H6.2: Suppress justified warnings
- **Files:** Various, especially enemy AI files
- **Add:** `#[allow(clippy::too_many_arguments)]` where functions genuinely need many params
- **Requirement:** Each `#[allow]` has a `// Reason: ...` comment
- **Acceptance:** `cargo clippy` produces zero warnings

#### H6.3: Create GitHub Actions CI
- **File:** `.github/workflows/ci.yml`
- **Pipeline:** `cargo build` → `cargo test` → `cargo clippy -- -D warnings`
- **Triggers:** Push to any branch, PR to main
- **Acceptance:** CI config exists and is valid YAML

#### H6.4: Document public engine APIs
- **Files:** `crates/engine/src/**`
- **Add:** `/// ` doc comments to all `pub` items (structs, functions, types)
- **Acceptance:** `cargo doc -p engine --no-deps` generates without warnings

#### H6.5: Write tuning values reference
- **File:** `docs/tuning-values.md`
- **Contents:** Table of all gameplay constants, their values, what they affect, and suggested ranges
- **Acceptance:** Document exists with all constants from `tuning.rs`

---

## Parallelism Strategy

```
Batch 1 (parallel):  H0 (crash fixes) + H1 (tests)
                          │                    │
Batch 2 (parallel):  H2 (perf) ───────── H4 (AI polish)
                          │                    │
Sequential:          H3 (architecture) ◄───────┘
                          │
Batch 3 (parallel):  H5 (game feel) + H6 (CI/quality)
```

## Verification Criteria (All Sprints Complete)

- [ ] `cargo build` — clean compilation on stable Rust
- [ ] `cargo test` — 20+ meaningful tests passing
- [ ] `cargo clippy` — zero warnings
- [ ] `cargo run` — complete 3-wave arena (start → victory)
- [ ] `cargo run` — die and restart (death sequence works)
- [ ] Terminal resize during gameplay — no panic
- [ ] `main.rs` reduced to ~500 LOC (from ~930)
- [ ] All tuning constants in `tuning.rs` with doc comments
- [ ] CI pipeline configuration exists
- [ ] No nightly-only API usage
