# Phase 3 Review — Dungeon Structure

**Reviewer:** QA Lead
**Date:** Session 3.7

## Summary

Phase 3 delivers a complete procedural dungeon system with room templates, floor generation, room transitions, encounter waves, a Bone King boss fight, minimap, pause screen, and pickup items. The game transforms from a single-arena combat demo into a multi-room dungeon crawl with progression across floors. All 87 tests pass. Zero compiler warnings. Zero clippy errors.

## Floor Generation

- **Algorithm**: Binding of Isaac-style grid placement. Start at (0,0), grow outward via open connections. Special rooms assigned to dead ends (furthest = boss, second = exit, then treasure, shop).
- **Config scaling**: Floor 1 targets 6-10 rooms, scaling to 12-15 by floor 5+. `enemy_count_mult` and `enemy_hp_mult` increase per floor (currently unused by encounter system but wired for Phase 4).
- **Reliability**: Retry loop (up to 20 attempts with different seeds) plus a fallback linear floor (start-combat-combat-boss-exit). Tested with 50 different seeds — all produce valid layouts.
- **Determinism**: Same seed produces identical layouts. Different seeds produce different layouts.

### Observations
- Floor layouts feel varied enough for early game. The 3 combat templates (arena, pillared_hall, l_shape) plus 2 corridors provide decent variety.
- Dead-end assignment for special rooms works well — boss is always far from start, creating a natural exploration arc.
- Occasional tight layouts with only 6 rooms create fast, intense floors. Larger 10+ room floors feel more exploratory. Good range.

### Balance Notes
- Floor 1 combat rooms have 2-3 skeletons (Easy) or 2-3 skeletons + 1 ghost (Medium). This feels appropriate for introduction.
- By floor 3, Easy rooms have 4-5 skeletons. Medium rooms add 2+ ghosts. Hard rooms with 2 waves create genuine pressure.
- Boss encounter is always a single Bone King regardless of floor number. Consider adding minion waves on higher floors (Phase 4).

## Room Transitions

- **Fade system**: 0.3s fade-out, instant room swap, 0.3s fade-in. Total transition time: ~0.6s. Feels snappy without being jarring.
- **Room swap**: Clears all per-room state (enemies, projectiles, particles, pickups, wave tracker). Repositions player at the entry point matching their arrival direction with 1.5-tile offset inward.
- **Camera snap**: Camera snaps to player position after swap, then clamps to tilemap bounds. No jitter observed in code review.

### Observations
- 0.5s room entry invincibility prevents unfair hits from enemies that spawn near doorways.
- Door collision check uses both exact tile match and 1-tile adjacency, handling 2-wide doors correctly.
- "SEALED" text flash (1.5s fade) when doors close is a nice Hades-style touch.

## Encounter System

- **Difficulty tiers**: Easy (single wave, skeletons only), Medium (2 waves, skeletons then skeletons+ghosts), Hard (2 waves, both with ghosts), Boss (single Bone King).
- **Wave triggers**: Immediate, OnPreviousWaveCleared, and OnEnemyCountBelow(n) provide good pacing variety.
- **Floor scaling**: Floor bonus adds 0-3 extra skeletons and 0-2 extra ghosts per tier per floor. Spawn count capped by available spawn points — prevents overcrowding.
- **Seed-based variety**: Same difficulty can produce slightly different counts via `seed % 3`.

### Observations
- Wave-based spawning with the WaveTracker is a clean replacement for the old WaveManager. The encounter system is more flexible and room-aware.
- Encounter completion requires all waves spawned AND all enemies dead AND at least one enemy was spawned. This prevents empty rooms from soft-locking.
- Medium rooms spawn wave 2 only after wave 1 is fully cleared. Hard rooms spawn wave 2 when fewer than 2 enemies remain. This creates good tension: Hard rooms have overlapping waves.

## Bone King Boss Fight

- **HP**: 20, with phase transition at 10 HP.
- **Phase 1 attacks**: Alternating slam (0.6s windup, 0.15s active, 0.4s recovery) and sweep (0.5s windup, 0.2s active, 0.3s recovery). Idle 0.5s between attacks.
- **Phase 2**: Speed +30%, windups shortened (slam 0.45s, sweep 0.35s), idle reduced to 0.35s. 33% chance of charge attack each cycle.
- **Charge**: 0.4s windup, then 100px/s horizontal charge until wall collision. 1.0s stun on wall hit (0.5s on timeout). Damage window: body hitbox active during charge.
- **Phase transition**: Roar state (0.8s, invulnerable). Camera shake during roar. Red tint in phase 2.
- **Death**: 1.5s dying sequence with rapid white/red flashing. Double death burst particles + 10-frame hit pause + camera shake 8.0. Boss death does NOT immediately open doors — needs to set `alive = false` which triggers room clear logic.

### Observations
- Boss deals 2 damage per hit (vs 1 for skeletons/ghosts). With 5 player HP, that's 2-3 hits to kill. Telegraphs (windup states) give player time to react.
- Slam has clear forward hitbox (15x12 pixels). Sweep has wide arc (30x10 pixels). Players need to learn the difference: dodge sideways for slam, back away for sweep.
- Charge wall-hit stun (1.0s) creates a clear damage window. This is the main intended strategy — bait charge, punish stun. Good Hades-like design.
- Reduced knockback on boss (0.3x) prevents the player from pushing the boss into walls trivially.
- Boss invulnerability during Roar and Dying states prevents cheese kills during transitions.

## Minimap

- **Display**: Top-right corner. Each room is a 5x3 pixel colored rectangle. Connections shown as single-pixel lines between room centers.
- **Color coding**: Current room = bright white. Cleared rooms = green. Uncleared discovered = grey. Boss = red. Exit = gold. Treasure = yellow. Shop = cyan. Start = white.
- **Fog of war**: Only discovered rooms and their immediate neighbors (dimmed) are shown. Undiscovered rooms are hidden.
- **Toggle**: Tab key toggles minimap visibility. Always visible on pause screen regardless of toggle.

### Observations
- Minimap updates correctly as rooms are discovered during transitions.
- Grid-based layout with axis-aligned connections displays cleanly.
- Color coding provides useful at-a-glance information without being distracting.

## Pause Screen

- **Esc** toggles pause. During pause, Q quits the game.
- **Display**: Semi-transparent black overlay (60% opacity). Shows "PAUSED", floor number, HP, rooms explored/total, and control hints.
- **Full minimap**: Always shown on pause screen, even if toggled off during gameplay.

### Observations
- Pause correctly freezes all game logic (enemies, projectiles, particles, timers).
- Tab (minimap toggle) works even while paused.
- No issues with state corruption on unpause — all timers resume correctly.

## Pickups

- **SmallHeal**: +1 HP. 25% chance to spawn at room center after clearing a combat room.
- **Visual**: 6-pixel bobbing animation (sin wave, 3 Hz). Collection radius 8 pixels. Particle burst on collection.
- **BigHeal and Gold**: Defined but not spawned yet (Phase 4 treasure rooms and currency).

### Observations
- Drop rate of 25% per cleared room feels reasonable. With ~5 combat rooms per floor and 5 HP, the player might heal 1-2 times per floor.
- Pickups are cleared on room transition — they don't persist. This is correct since each room has its own encounter.

## Performance

- **Release build**: Compiles in ~34s. Clean build, no warnings.
- **Game loop**: Fixed 30 tick/sec with accumulator. Frame time clamped at 250ms to prevent spiral of death.
- **Rendering**: Differential renderer only redraws changed cells. Performance bars in HUD show FPS, redraw ratio, input time, and render time.
- **Boss fight particles**: Boss death spawns 2x (30-40) = 60-80 particles. Hit sparks add 8-12 per hit. Particle system has MAX_PARTICLES cap (512 default). Should handle boss fight without issue.
- **Memory**: Enemy vectors are cleared on room transition. No unbounded growth. Projectile system preallocates capacity of 32.

## Bugs Found and Fixed (Session 3.7)

1. **`waves.rs` dead code**: Entire file unused after encounter system replacement. Removed file and `mod waves;` declaration.
2. **`spawn_enemies_for_room` dead code**: Method in `world.rs` replaced by encounter system. Removed method and unused imports.
3. **14 clippy warnings**: Fixed `is_multiple_of`, unnecessary casts, `is_some_and`, struct init patterns, duplicate if-else branches.

## No Bugs Found (Code Review)

The following potential issues were investigated and found to be handled correctly:
- Door collision with 2-wide doors (adjacency check covers this)
- Room entry invincibility prevents unfair damage
- Boss invulnerability during Roar/Dying prevents state corruption
- Wave tracker prevents empty-room soft-locks
- Floor generation fallback prevents crash on failed generation
- Camera clamp prevents rendering outside tilemap bounds
- Pickup collection during room transition (pickups cleared on swap)

## Test Coverage

| Crate | Tests | Coverage |
|-------|-------|----------|
| engine | 65 | Animation (15), Collision (16), Framebuffer (16), Particles (18) |
| game | 22 | FloorGen (10), RoomTemplate (3), Templates (9) |
| **Total** | **87** | All passing |

Notable gap: No tests for encounter selection logic, wave tracker state machine, or door collision detection. These work correctly based on code review, but adding unit tests would improve confidence for Phase 4 changes.

## Phase 4 Recommendations

### Priority 1: Boon/Upgrade System
- The encounter difficulty framework is ready. Boons that modify player stats (speed, damage, HP) slot naturally into the `Player` struct.
- Room clear rewards could use the existing pickup system with new `PickupType` variants.
- Treasure rooms and shops already have templates and room types; they just need content.

### Priority 2: Weapon Variety
- Current attack is a single melee swing. The attack hitbox system (`ATTACK_HITBOX_RIGHT/LEFT`) is extensible to different weapon shapes and timings.
- Projectile system could be shared for player ranged weapons (same `ProjectileSystem`, different source).

### Priority 3: Difficulty Scaling
- `FloorConfig.enemy_count_mult` and `enemy_hp_mult` are computed but unused by encounters. Wire these into `select_encounter` for proper floor scaling.
- Boss rooms on higher floors could spawn minion waves alongside the Bone King.
- Consider new boss types for floors 3+ (Phase 4/5 scope).

### Priority 4: Meta-Progression
- Death currently restarts the same floor. A meta-progression system (persistent unlocks, starting bonuses) would add replayability.
- Floor number display in HUD is ready. Score/currency could use the Gold pickup type.

### Polish Items
- Room name announce on entry (commented placeholder in render at line 735). Would add atmosphere.
- Treasure/shop room content. Templates exist but rooms are currently empty.
- Sound design (not applicable until SSH/terminal audio is explored).
