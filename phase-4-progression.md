# Phase 4: Progression & Depth — Boons, Weapons, and Replayability

## Overview
**Duration:** Weeks 17–22 (10–15 sessions)
**Goal:** Complete roguelike run loop with boon selection, multiple weapons, meta-progression, and meaningful replay value.
**Prerequisite:** Phase 3 complete (full floor with rooms, transitions, boss, minimap)

---

## Session 4.1 — Boon Selection System

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. The floor structure works. Now implement the boon (upgrade) system — the core of roguelike replayability.

### Boon architecture

Create game/src/boons/mod.rs:

```rust
#[derive(Clone)]
pub struct BoonDef {
    pub id: BoonId,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static SpriteData,  // 8×8 pixel icon
    pub rarity: Rarity,
    pub category: BoonCategory,
    pub effect: BoonEffect,
    pub stackable: bool,   // can you pick this boon multiple times?
}

#[derive(Clone, Copy, PartialEq)]
pub enum Rarity { Common, Rare, Legendary }

#[derive(Clone, Copy, PartialEq)]
pub enum BoonCategory { Offense, Defense, Mobility, Special }

#[derive(Clone)]
pub enum BoonEffect {
    // Stat modifications
    DamageFlat(i32),          // +N damage per hit
    DamagePercent(f32),       // +N% damage
    AttackSpeedPercent(f32),  // reduce cooldown by N%
    MaxHpFlat(i32),           // +N max HP (and heal that amount)
    MoveSpeedPercent(f32),    // +N% move speed
    DashDistancePercent(f32), // +N% dash distance
    DashCooldownReduction(f32), // reduce dash cooldown by N seconds

    // Triggered effects
    OnHitHeal(f32),           // N% chance to heal 1 HP on hit
    OnKillExplode(f32),       // enemies explode for N damage on death
    OnDashDamage(i32),        // dash deals N damage to enemies passed through
    OnTakeDamageRetaliate(i32), // deal N damage to attacker when hit
    
    // Unique mechanics
    ProjectileAttack,         // attack fires a projectile in addition to melee
    ChainLightning(i32),     // hits arc to N nearby enemies for half damage
    LifeSteal(f32),          // heal for N% of damage dealt
    Shield(i32),             // absorb N hits before taking HP damage
    Thorns(i32),             // attackers take N damage
}
```

### Boon pool — define 20+ boons:

**Offense (8):**
1. Sharpened Blade — +1 damage (Common, stackable)
2. Berserker's Rage — +25% damage (Rare)
3. Swift Strikes — +30% attack speed (Rare)
4. Killing Blow — enemies explode for 2 damage on death (Rare)
5. Chain Lightning — hits arc to 2 nearby enemies (Legendary)
6. Projectile Slash — attacks send a ranged slash projectile (Legendary)
7. Critical Edge — 20% chance for double damage (Rare)
8. Fury — +5% damage per enemy killed this room (Common, stackable)

**Defense (6):**
1. Tough Skin — +1 max HP (Common, stackable)
2. Iron Shield — absorb 2 hits (Rare, shield regenerates each room)
3. Life Steal — heal 15% of damage dealt (Rare)
4. Vampiric Touch — 10% chance to heal on hit (Common)
5. Retaliation — attackers take 1 damage (Rare)
6. Second Wind — revive once per floor with 1 HP (Legendary)

**Mobility (4):**
1. Swift Feet — +20% movement speed (Common, stackable)
2. Phantom Dash — +40% dash distance (Rare)
3. Shadow Step — dash cooldown reduced 50% (Rare)
4. Dash Strike — dash deals 2 damage to enemies passed through (Legendary)

**Special (4):**
1. Gold Magnet — +50% gold drop (Common)
2. Lucky — increased rare/legendary boon chance (Rare)
3. Treasure Sense — minimap reveals treasure rooms (Common)
4. Death's Bargain — +3 damage but max HP reduced to 1 (Legendary)

### Boon selection UI

When a boon choice triggers (after certain room clears, from treasure rooms):

1. Game pauses
2. Screen darkens slightly
3. Three boon cards appear side by side in the center:
   - Each card: 24×20 pixel bordered rectangle
   - Contains: icon (8×8), name (pixel text), short description
   - Rarity border color: white=Common, blue=Rare, gold=Legendary
4. Player navigates with Left/Right arrows
5. Selected card is highlighted (brighter, slightly raised)
6. Press Attack/Enter to choose
7. Brief "power up" animation (screen flash, particles from player)
8. Apply boon effects to player stats

### Boon card rendering:

Create game/src/hud/boon_select.rs:

```rust
pub struct BoonSelectScreen {
    pub options: [BoonDef; 3],
    pub selected: usize,  // 0, 1, or 2
    pub animation_timer: f32,
    pub active: bool,
}
```

For the card sprites, create a system that renders text using the tiny pixel font from Phase 2. Each card is composed of:
- Border (1px, colored by rarity)
- Dark background fill
- Icon centered at top
- Name in white (may need to abbreviate to fit ~8-10 chars)
- 1-line description in gray below

### Boon effect application:

Create a `PlayerBoons` struct that tracks active boons and modifies player stats:

```rust
pub struct PlayerBoons {
    pub active: Vec<BoonId>,
    
    // Cached computed stat modifiers (recalculate when boons change)
    pub damage_flat_bonus: i32,
    pub damage_mult: f32,
    pub attack_speed_mult: f32,
    pub max_hp_bonus: i32,
    pub move_speed_mult: f32,
    pub dash_distance_mult: f32,
    pub dash_cooldown_reduction: f32,
    
    // Triggered effect flags
    pub on_hit_heal_chance: f32,
    pub on_kill_explode_damage: i32,
    pub on_dash_damage: i32,
    pub shield_charges: i32,
    pub shield_max: i32,
    pub has_projectile_attack: bool,
    pub chain_lightning_targets: i32,
    pub life_steal_percent: f32,
}

impl PlayerBoons {
    pub fn add(&mut self, boon: &BoonDef) { ... }
    pub fn recalculate(&mut self) { ... }
}
```

### Boon selection algorithm:
When generating 3 options:
1. Weight by rarity: Common 60%, Rare 30%, Legendary 10%
2. Don't offer duplicate non-stackable boons the player already has
3. Try to offer at least 2 different categories
4. Lucky boon shifts weights: Common 40%, Rare 40%, Legendary 20%

### Test:
- Add boon selection after every 2nd combat room clear
- Verify all 3 cards render with correct icons, names, descriptions
- Select each boon and verify its effect:
  - Damage boons: enemies die faster
  - HP boons: hearts increase
  - Speed boons: visible speed change
  - Shield: hits absorbed before HP loss
- Play a full run collecting 3-4 boons, verify they stack correctly
```

### Success Criteria
- [ ] 20+ boons defined across 4 categories
- [ ] Selection UI renders cleanly with 3 cards
- [ ] Navigation and selection feels responsive
- [ ] Boon effects are immediately noticeable in gameplay
- [ ] Stackable boons compound correctly
- [ ] Rarity weighting produces good distribution

---

## Session 4.2 — Weapon System

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Boons work. Now add weapon variety.

### Create game/src/weapons.rs

Start with 3 weapons that play fundamentally differently:

```rust
pub struct WeaponDef {
    pub id: WeaponId,
    pub name: &'static str,
    pub base_damage: i32,
    pub attack_cooldown: f32,  // seconds between attacks
    pub hitbox: AABB,           // attack hitbox (facing right)
    pub active_frames: (usize, usize),  // animation frames where hitbox is active
    pub animations: WeaponAnimations,
    pub knockback_force: f32,
}

pub struct WeaponAnimations {
    pub idle: &'static AnimationData,
    pub attack: &'static AnimationData,
}
```

### Weapon 1: Sword (balanced, default)
- Damage: 2
- Cooldown: 0.35s
- Hitbox: 12×8 pixels (medium range, medium width)
- Active frames: 2-3 (of 4-frame attack animation)
- Knockback: medium
- Feel: reliable, good for learning

### Weapon 2: Spear (slow, long range)
- Damage: 3
- Cooldown: 0.5s
- Hitbox: 18×4 pixels (long range, narrow)
- Active frames: 2-3 (of 5-frame attack with longer wind-up)
- Knockback: strong (long thrust pushes far)
- Feel: precision weapon, rewards positioning. Punishes at range.

### Weapon 3: Daggers (fast, short range)
- Damage: 1
- Cooldown: 0.15s
- Hitbox: 6×10 pixels (short range, wide)
- Active frames: 1-2 (of 3-frame rapid slash)
- Knockback: weak
- Feel: aggressive, get in close and shred. High DPS but risky.

### Weapon sprites (Visual Designer subtask):

Each weapon needs attack animation frames. The weapon swing should be visually distinct:
- Sword: horizontal arc
- Spear: forward thrust
- Daggers: rapid alternating slashes

The weapon sprites modify the player sprite during attack — either overlay a weapon shape or use completely separate attack frames that include both body and weapon.

Approach: Create attack animation frames PER WEAPON that replace the player's attack animation. The player's idle/walk/dash frames stay the same (weapon is at their side, small enough to be implied).

### Weapon selection:
- Player starts each run choosing a weapon (press 1/2/3 on a selection screen)
- Or: player starts with Sword, finds other weapons in treasure rooms
- For v1: weapon select at run start. Treasure rooms can offer a weapon swap.

### Weapon-boon synergies (these emerge naturally):
- Daggers + Life Steal = rapid healing (many hits per second)
- Spear + Chain Lightning = safe ranged crowd control
- Daggers + On Kill Explode = chain reaction potential
- Sword + Projectile Slash = versatile all-range option

### Create a weapon select screen:

Before the run starts (after title screen, before floor 1):
- Show 3 weapons side by side (similar UI to boon selection but larger)
- Each shows: weapon sprite, name, damage, speed, range description
- Navigate with arrows, select with Enter
- Selected weapon equipped for the entire run

### Test:
- Play through floor 1 with each weapon
- Verify damage, cooldown, and hitbox feel correct for each
- Verify weapon animations look good
- Test weapon-boon synergies (daggers + life steal especially)
- Compare: does each weapon feel meaningfully different?
```

### Success Criteria
- [ ] 3 weapons implemented with distinct gameplay feel
- [ ] Attack animations are unique per weapon
- [ ] Damage/cooldown values feel balanced
- [ ] Weapon selection screen works
- [ ] Boon effects apply correctly to all weapons
- [ ] Each weapon is viable (no weapon is obviously best)

---

## Session 4.3 — Meta-Progression (Between-Run Unlocks)

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Add persistent progression between runs.

### Create game/src/save.rs

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub total_gold: u32,
    pub total_runs: u32,
    pub best_floor: u32,
    pub total_kills: u32,
    pub total_deaths: u32,
    pub total_playtime_secs: u64,
    pub unlocks: Unlocks,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Unlocks {
    pub weapons: Vec<WeaponId>,       // unlocked weapon options
    pub starting_boons: Vec<BoonId>,  // boons available at run start
    pub upgrades: PermanentUpgrades,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PermanentUpgrades {
    pub max_hp_bonus: i32,      // +1 per purchase, max +3
    pub damage_bonus: i32,       // +1 per purchase, max +2
    pub dash_charges: i32,       // +1 per purchase, max +1 (2 dashes)
    pub starting_gold: u32,      // gold you start each run with
    pub boon_reroll_charges: u32, // times you can reroll boon selection per run
}
```

### Save file location:
- Local play: `~/.cryptfall/save.json`
- SSH play: server-side, keyed by SSH public key fingerprint (Phase 5)
- Use serde_json for serialization

### Gold economy:
- Gold drops from enemies (1-3 per kill based on enemy type)
- Bonus gold from room clear (5 per combat room)
- Boss kill: 25 gold
- Gold persists between runs (saved on death)

### Upgrade shop (between-run, title screen):
Accessible from the title screen menu:

```
UPGRADES (Gold: 147)

[1] Vitality I    → +1 Max HP     (Cost: 30g)  [PURCHASED]
[2] Vitality II   → +1 Max HP     (Cost: 60g)  [LOCKED - requires Vitality I]
[3] Vitality III  → +1 Max HP     (Cost: 120g) [LOCKED]
[4] Strength I    → +1 Damage     (Cost: 50g)
[5] Strength II   → +1 Damage     (Cost: 100g) [LOCKED]
[6] Twin Dash     → 2 Dash Charges (Cost: 80g)
[7] Boon Reroll   → 1 Reroll/run  (Cost: 40g)  [PURCHASED]
[8] Boon Reroll+  → 2 Rerolls/run (Cost: 80g)
```

### Run statistics screen:

After death or run completion, show:
```
╔══════════════════════════╗
║     RUN COMPLETE         ║
║                          ║
║  Floor Reached: 3        ║
║  Enemies Killed: 47      ║
║  Damage Dealt: 142       ║
║  Damage Taken: 8         ║
║  Boons Collected: 4      ║
║  Time: 7:23              ║
║                          ║
║  Gold Earned: 38         ║
║  Total Gold: 185         ║
║                          ║
║  [Enter] Continue        ║
╚══════════════════════════╝
```

Render this as pixel text in a bordered box. The stats should be satisfying to read — make big numbers feel earned.

### Run tracking:

Create game/src/run_state.rs:

```rust
pub struct RunState {
    pub floor_number: u32,
    pub kills: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub gold_earned: u32,
    pub boons_collected: u32,
    pub rooms_cleared: u32,
    pub start_time: Instant,
    pub weapon: WeaponId,
    pub active_boons: Vec<BoonId>,
}
```

Track these stats during gameplay and display them on the death/completion screen.

### Game flow with meta-progression:

Title Screen → Upgrade Shop (optional) → Weapon Select → Floor 1 → ... → Death/Victory → Stats Screen → Title Screen

Each loop through this cycle should feel like progress even on death (gold earned, upgrades purchased, new personal best floor).

### Save/load:
- Save after every run (on death screen)
- Load on game start
- Handle missing save file (create default)
- Handle corrupted save file (reset to default, warn player)

### Test:
- Complete 3 runs, verify gold accumulates
- Purchase upgrades, verify they apply to next run
- Verify save file persists after quitting and relaunching
- Verify stats screen shows correct numbers
- Verify the "one more run" motivation loop works
```

### Success Criteria
- [ ] Gold drops from enemies and persists between runs
- [ ] Upgrade shop allows purchasing permanent buffs
- [ ] Upgrades apply to subsequent runs correctly
- [ ] Stats screen shows meaningful end-of-run data
- [ ] Save file persists across launches
- [ ] Corrupted save file handled gracefully
- [ ] Meta-progression feels motivating

---

## Session 4.4 — Title Screen & Game Flow

### Claude Code Prompt

```
You are the Visual Designer and Game Designer for Cryptfall. Create the title screen and polish the full game flow.

### Title screen

Create game/src/screens/title.rs:

The title screen should be STRIKING — this is what people see in the README GIF.

1. **ASCII art logo** at the top:
   Using the half-block pixel renderer, create a large "CRYPTFALL" logo:
   - ~60px wide × 16px tall
   - NES-style pixel font (each letter ~6-7px wide)
   - Color: gradient from blue to white (cold dungeon feel)
   - Optional: subtle animation (letters shimmer, torch-light flicker effect)

2. **Animated background:**
   - Slowly scrolling dungeon tilemap behind the menu (parallax-like)
   - Or: particle effect (falling dust/embers)
   - Dark enough that menu text is readable

3. **Menu options:**
   - NEW RUN (→ Weapon Select → Game)
   - UPGRADES (→ Upgrade Shop)
   - STATS (→ Career Stats: total runs, best floor, total kills, playtime)
   - QUIT
   - Navigate with Up/Down, select with Enter
   - Selected item: bright white with indicator arrow
   - Unselected: dim gray

4. **Bottom info:**
   - "Gold: {amount}" in bottom-left
   - "Best Floor: {number}" in bottom-right
   - Version number (v0.1.0)

### Game state machine

Create game/src/screens/mod.rs:

```rust
pub enum GameScreen {
    Title(TitleScreen),
    Upgrades(UpgradeScreen),
    WeaponSelect(WeaponSelectScreen),
    Playing(GameplayState),
    Paused(PauseScreen),
    BoonSelect(BoonSelectScreen),
    DeathScreen(DeathScreen),
    VictoryScreen(VictoryScreen),  // beat final boss
    Stats(StatsScreen),
}
```

Each screen implements update() and render(). The main game loop dispatches to the current screen. Transitions between screens use the fade effect.

### Screen transition table:
- Title → NEW RUN → WeaponSelect → Playing
- Title → UPGRADES → Upgrades → (Back) → Title
- Title → STATS → Stats → (Back) → Title  
- Title → QUIT → exit
- Playing → Pause (Escape) → Paused → Resume/Quit
- Playing → Room Clear → BoonSelect → Playing
- Playing → Death → DeathScreen → Title
- Playing → Beat Final Boss → VictoryScreen → Title

### Polish the game loop:
- Ensure every state transition has a brief fade (0.2-0.3s)
- Ensure input doesn't bleed between screens (clear input buffer on screen change)
- Ensure the game loop handles screen stack properly (pause overlays gameplay)

### Test:
- Navigate all menu options
- Complete a full cycle: Title → Play → Die → Stats → Title → Upgrades → Title → Play again
- Verify no state leaks between screens
- Verify the title screen looks impressive (this is GIF-worthy)
```

### Success Criteria
- [ ] Title screen is visually striking
- [ ] All menu navigation works
- [ ] Screen transitions are smooth
- [ ] No input bleeding between screens
- [ ] Full game cycle works: title → play → death → title → play
- [ ] Game state is properly cleaned up between runs

---

## Session 4.5 — Balance Pass & Phase 4 Review

### Claude Code Prompt

```
You are the QA Lead for Cryptfall. Conduct a comprehensive balance review of the complete game.

### Play 5+ full runs with different weapon/boon combinations:

Run 1: Sword, focus on offense boons
Run 2: Daggers, focus on defense/lifesteal
Run 3: Spear, focus on mobility
Run 4: Random choices (let RNG decide)
Run 5: Intentionally pick "bad" combinations — is anything unplayable?

### For each run, document:
- Weapon choice + boons collected
- Floors reached
- Deaths and what killed you
- Which rooms felt too easy or too hard
- Which boons felt impactful vs. useless
- Time to complete each floor

### Balance evaluation:

**Difficulty curve:**
- Floor 1 should be clearable by most players
- Floor 2 should require some skill + decent boons
- Floor 3 should be challenging — maybe 30% of runs reach here
- Floor 4+ should feel like an achievement
- Does the difficulty scale smoothly or are there spikes?

**Weapon balance:**
- Is one weapon clearly dominant?
- Does each weapon feel viable for at least 2 floors?
- Do boons feel equally useful across weapons?

**Boon balance:**
- Are any boons must-picks? (too strong)
- Are any boons never-picks? (too weak or boring)
- Do synergies emerge naturally?
- Is Legendary rarity appropriately powerful?

**Economy:**
- How many runs to afford the first upgrade? (should be 2-3)
- How many runs to max all upgrades? (should be 15-20+)
- Does gold income feel fair?

**Pacing:**
- Floor 1 length? (target: 3-5 minutes)
- Boss fight duration? (target: 1-2 minutes)
- Total run to floor 3? (target: 12-20 minutes)
- Does any part feel like a slog?

### Output:
Create docs/balance-pass.md with:
- All run logs
- Specific number changes recommended (e.g., "Daggers damage 1→1.5", "Ghost projectile speed 80→70")
- Boons to buff/nerf/rework
- Encounter adjustments
- Economy adjustments

Also create docs/tuning-values.md — a single file listing EVERY tunable constant in the game with current values and recommended changes. This becomes the authoritative reference for future tuning.
```

### Success Criteria
- [ ] 5+ runs completed with documented results
- [ ] No weapon is dominant or useless
- [ ] No boon is auto-pick or never-pick
- [ ] Difficulty curve is smooth
- [ ] Economy feels fair (2-3 runs for first upgrade)
- [ ] Balance document written with specific recommendations

---

## Phase 4 File Manifest

```
crates/game/src/
├── boons/
│   ├── mod.rs           # BoonDef, BoonEffect, boon pool
│   ├── effects.rs       # Boon effect application logic
│   └── selection.rs     # Weighted random selection algorithm
├── weapons.rs           # WeaponDef, 3 weapon definitions
├── save.rs              # SaveData, load/save, upgrades
├── run_state.rs         # Per-run statistics tracking
├── screens/
│   ├── mod.rs           # GameScreen enum, dispatch
│   ├── title.rs         # Title screen with ASCII logo
│   ├── weapon_select.rs # Weapon choice screen
│   ├── upgrade_shop.rs  # Between-run upgrade shop
│   ├── death.rs         # Death + stats screen
│   ├── victory.rs       # Boss defeat screen
│   ├── stats.rs         # Career statistics
│   └── pause.rs         # Pause overlay
├── hud/
│   └── boon_select.rs   # In-run boon selection cards
├── sprites/
│   ├── weapons.rs       # Weapon attack frames (per weapon)
│   ├── boon_icons.rs    # 8×8 icons for each boon
│   └── title.rs         # Title screen logo pixels
└── main.rs              # Updated with full screen state machine

docs/
├── balance-pass.md
└── tuning-values.md
```
