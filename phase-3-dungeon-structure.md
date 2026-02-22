# Phase 3: Dungeon Structure — Procedural Floors & Room Transitions

## Overview
**Duration:** Weeks 12–16 (8–12 sessions)
**Goal:** Player explores a procedurally generated multi-room floor, fights through encounters, transitions between rooms, and descends to harder floors.
**Prerequisite:** Phase 2 complete (combat, enemies, particles, HUD all working)

---

## Session 3.1 — Room Templates & Data Format

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Combat works in a single room. Now design the room template system for procedural dungeon generation.

### Room template format

Create game/src/dungeon/room_template.rs:

```rust
/// A room template defines the shape and layout of a room
pub struct RoomTemplate {
    pub width: usize,        // in tiles
    pub height: usize,       // in tiles
    pub tiles: Vec<TileType>,  // row-major
    pub spawn_points: Vec<SpawnPoint>,  // where enemies can appear
    pub entry_points: Vec<EntryPoint>,  // where doors/connections can be
    pub player_spawn: Option<(usize, usize)>,  // default player position (tile coords)
    pub room_type: RoomType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RoomType {
    Combat,      // enemies to fight
    Treasure,    // reward chest
    Shop,        // buy items/upgrades  
    Boss,        // boss encounter
    Start,       // first room of floor
    Exit,        // stairs down
    Corridor,    // connecting passage
}

pub struct SpawnPoint {
    pub x: usize,
    pub y: usize,
    pub group: u8,  // spawn wave group (0 = immediate, 1 = wave 2, etc.)
}

pub struct EntryPoint {
    pub x: usize,
    pub y: usize, 
    pub direction: Direction,  // which edge of the room this entry faces
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { North, South, East, West }
```

### Design 8-10 hand-crafted room templates:

Rooms should be varied sizes (small: 12×10, medium: 18×14, large: 24×18 tiles).

1. **Start Room (medium)**: Open room, single entry south. No enemies. Player spawns center.

2. **Arena (large)**: Open combat room. 4 entry points (one per side). 6 spawn points spread around edges.

3. **Pillared Hall (large)**: Combat room with 4 interior pillars (2×2 wall blocks). Creates cover and tactical positioning. 4 entries, 8 spawn points between pillars.

4. **Corridor-H (small, wide)**: Horizontal connecting corridor, 18×6 tiles. Entries on east and west ends. 2 spawn points.

5. **Corridor-V (small, tall)**: Vertical corridor, 6×18. North and south entries. 2 spawn points.

6. **L-Shape (medium)**: L-shaped room with walls creating the turn. 2 entries (one per arm of the L). 4 spawn points.

7. **Treasure Vault (small)**: 10×8 room, single entry. Chest spawn point in center. Optional guard spawn points.

8. **Boss Arena (large)**: 26×20, open with decorative pillars around edges. Single entry south. Boss spawn center.

9. **Shop (medium)**: 16×12, counter-like wall structure inside. Single entry. NPC positions.

10. **Exit Room (small)**: 12×10, stairs-down tile in center. Single entry.

For each template, define the tile layout as a const string that gets parsed:

```rust
const ARENA_TEMPLATE: &str = "\
WWWWWWWWWDWWWWWWWWWW
W..................W
W..................W
W..................W
D..................D
W..................W
W..................W
W..................W
W..................W
WWWWWWWWWDWWWWWWWWWW";
// W=Wall, .=Floor, D=Door position
```

Create a parser that converts these strings + metadata into RoomTemplate structs.

### Add door tiles:
Add a DoorClosed and DoorOpen tile type. Door tiles:
- DoorClosed: solid (blocks movement), distinct visual (wooden door sprite)
- DoorOpen: passable, different visual (open doorway)
- Doors close when combat starts, open when all enemies are dead
```

### Success Criteria
- [ ] 8+ room templates defined with varied sizes and layouts
- [ ] Templates parse correctly from string format
- [ ] Each template has appropriate entry points, spawn points, and room type
- [ ] Door tiles render distinctly and can toggle open/closed

---

## Session 3.2 — Procedural Floor Generator

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Room templates are ready. Now generate a floor layout from them.

### Create game/src/dungeon/floor_gen.rs

A floor is a graph of rooms connected by corridors. The generation algorithm:

### Step 1: Generate room graph
```rust
pub struct FloorLayout {
    pub rooms: Vec<PlacedRoom>,
    pub connections: Vec<(usize, usize, Direction)>,  // room_a, room_b, direction from a to b
}

pub struct PlacedRoom {
    pub template_id: usize,
    pub grid_x: i32,       // position on abstract grid
    pub grid_y: i32,
    pub world_x: i32,      // pixel position in world (for rendering)
    pub world_y: i32,
    pub room_type: RoomType,
    pub cleared: bool,
    pub discovered: bool,
}
```

### Generation algorithm (inspired by Binding of Isaac):

1. Place Start room at grid (0, 0)
2. Maintain a list of "open connections" (room + direction that needs a neighbor)
3. For 6-10 iterations:
   a. Pick a random open connection
   b. Calculate the grid position the new room would occupy
   c. Check that position isn't already occupied
   d. Select a compatible room template (has an entry point facing the connecting direction)
   e. Place the room
   f. Add its other open entry points to the open connections list
4. Ensure at least one Boss room and one Exit room exist (place them at the ends of branches)
5. If floor is too small (<6 rooms), retry
6. Place treasure and shop rooms in remaining dead-end positions

### Step 2: Calculate world positions

Each room gets a world position based on its grid position:
- Rooms are separated by a fixed gap (room max width + corridor length)
- Corridors connect adjacent rooms (use corridor templates)

### Step 3: Generate corridor connections

Between each pair of connected rooms:
- Find the matching entry points (Room A's east entry connects to Room B's west entry)
- Place a corridor template between them
- Corridors are short (6-8 tiles long) horizontal or vertical passages

### Floor difficulty scaling

```rust
pub struct FloorConfig {
    pub floor_number: u32,
    pub min_rooms: usize,      // 6 at floor 1, +1 per floor
    pub max_rooms: usize,      // 10 at floor 1, +1 per floor
    pub enemy_count_mult: f32, // 1.0 at floor 1, +0.15 per floor
    pub enemy_hp_mult: f32,    // 1.0 at floor 1, +0.1 per floor
    pub enemy_speed_mult: f32, // 1.0 at floor 1, +0.05 per floor
}
```

### Test:
- Generate 10 random floors at floor_number=1 and print their room graph to console
- Verify: all floors have 6-10 rooms, exactly 1 start, 1 boss, 1 exit
- Verify: all rooms are reachable (connected graph)
- Verify: no rooms overlap in grid space
- Debug render: show the floor graph as a minimap (rectangles + lines)
```

### Success Criteria
- [ ] Floor generation produces valid connected graphs
- [ ] Every floor has start, boss, and exit rooms
- [ ] Room variety (different templates are used)
- [ ] No overlapping rooms
- [ ] Reproducible with a seed (for debugging)
- [ ] Floor difficulty scales with floor number

---

## Session 3.3 — Room Transitions

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Floors generate correctly. Now implement the system for transitioning between rooms.

### Room transition system

Create game/src/dungeon/world.rs:

```rust
pub struct DungeonWorld {
    pub floor: FloorLayout,
    pub current_room_index: usize,
    pub active_tilemap: TileMap,          // current room's tilemap
    pub active_enemies: Vec<Enemy>,        // current room's enemies
    pub active_projectiles: Vec<Projectile>,
    pub transition: Option<RoomTransition>,
}

pub struct RoomTransition {
    pub from_room: usize,
    pub to_room: usize,
    pub direction: Direction,
    pub phase: TransitionPhase,
    pub timer: f32,
}

pub enum TransitionPhase {
    FadeOut { duration: f32 },      // darken current room
    Load,                            // swap room data
    FadeIn { duration: f32 },       // brighten new room
}
```

### Transition flow:

1. Player walks into a door tile (while doors are open / room is cleared):
   - Determine which connection this door leads to
   - Start transition: FadeOut over 0.3 seconds
   
2. FadeOut:
   - Apply darkening overlay to framebuffer (0.0 → 1.0 over 0.3s)
   - Player input is locked
   - At completion: switch to Load

3. Load:
   - Swap active_tilemap to the new room's tilemap
   - Spawn enemies for the new room (if not already cleared)
   - Position player at the new room's matching entry point
   - Clear projectiles and particles
   - Mark new room as discovered
   - Instant — no delay

4. FadeIn:
   - Darken overlay goes 1.0 → 0.0 over 0.3s
   - Unlock player input at completion

Total transition: ~0.6 seconds. Fast enough to not be annoying, slow enough to feel intentional.

### Room state persistence:
- When leaving a room: save its cleared status and enemy states
- When re-entering a cleared room: no enemies spawn
- When re-entering an uncleared room: enemies are restored (controversial — could also just respawn fresh)
- For simplicity in v1: uncleared rooms respawn enemies. Cleared rooms stay clear.

### Door behavior during combat:
- When player enters a combat room with enemies:
  - All doors close (DoorClosed tile, solid)
  - "SEALED" text flashes briefly
  - Enemies spawn (possibly in waves)
- When all enemies are dead:
  - All doors open (DoorOpen tile, passable)
  - Satisfying "doors open" sound effect placeholder (visual: doors flash bright then open)
  - Brief camera shake

### Player positioning on room entry:
- Player enters from the door tile corresponding to the connection direction
- Facing: into the room (e.g., entering from west door → player faces east)
- Brief invincibility (0.5s) to prevent cheap hits

### Test:
- Generate a 3-room floor (start → combat → exit)
- Walk through doors, verify transitions
- Clear the combat room, verify doors open
- Walk to exit room
- Verify minimap updates with discovered rooms
```

### Success Criteria
- [ ] Room transitions are smooth (fade out → swap → fade in)
- [ ] Player position is correct on room entry
- [ ] Doors lock during combat and unlock when cleared
- [ ] Cleared rooms stay clear on re-entry
- [ ] No visual artifacts during transition
- [ ] Minimap reflects discovered rooms

---

## Session 3.4 — Wave Spawning & Combat Encounters

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Room transitions work. Now design the encounter spawning system.

### Create game/src/dungeon/encounters.rs

```rust
pub struct EncounterDef {
    pub waves: Vec<WaveDef>,
}

pub struct WaveDef {
    pub enemies: Vec<(EnemyType, SpawnPoint)>,
    pub trigger: WaveTrigger,
}

pub enum WaveTrigger {
    Immediate,                    // spawn when room activates
    OnPreviousWaveCleared,        // spawn after previous wave dies
    OnTimer(f32),                 // spawn after N seconds into the room
    OnEnemyCountBelow(usize),    // spawn when fewer than N enemies remain
}
```

### Encounter templates based on floor difficulty:

Floor 1 combat rooms:
- Easy: 1 wave, 3 skeletons
- Medium: 2 waves (3 skeletons → 2 skeletons + 1 ghost)
- Hard: 2 waves (4 skeletons + 1 ghost → 3 skeletons + 2 ghosts)

Floor 2 (scale up):
- Easy encounters get medium, medium gets hard, hard gets harder
- Introduce new timing: wave 2 triggers when ≤2 enemies remain (creates pressure)

### Wave spawn animation:
When a wave spawns:
- Each enemy appears with a visual effect:
  1. A warning circle on the ground at the spawn point (red-orange, 0.5s duration)
  2. Enemy materializes over 0.3s (fade in from transparent to solid)
  3. Enemy starts in Idle state with 0.5s grace period (doesn't immediately aggro)
- Enemies don't all spawn simultaneously — stagger by 0.1-0.2s for visual clarity

### Room completion rewards:
When the LAST enemy of the LAST wave dies:
1. All remaining projectiles despawn (safety)
2. Doors open with fanfare (screen shake, flash)
3. A reward spawns in the room center:
   - Combat rooms: small health pickup (25% chance) or nothing
   - Special: every 3rd combat room guarantees a boon choice (Phase 4)

### Encounter selection for a floor:
When generating a floor, assign encounter difficulty to each combat room:
```rust
fn assign_encounters(floor: &mut FloorLayout, floor_number: u32) {
    let combat_rooms: Vec<usize> = floor.rooms.iter()
        .enumerate()
        .filter(|(_, r)| r.room_type == RoomType::Combat)
        .map(|(i, _)| i)
        .collect();
    
    // First combat room: always Easy
    // Middle rooms: Medium
    // Room before boss: Hard
    // Boss room: Boss encounter
}
```

### Health pickup:
```rust
pub struct Pickup {
    pub x: f32,
    pub y: f32,
    pub pickup_type: PickupType,
    pub animation: AnimationPlayer,
    pub bob_offset: f32,  // sine wave vertical bob
}

pub enum PickupType {
    SmallHeal,   // +1 HP
    BigHeal,     // +3 HP  
    Gold(u32),   // currency for meta-progression
}
```

Pickup sprites: small (5×5) bouncing sprites. Heart for heal, coin for gold. Bob up and down with a sine wave. Glow effect (slightly brighter pixels around it).

### Test:
- Generate a full floor (6-8 rooms)
- Play through from start to boss room
- Verify wave spawning, encounter difficulty progression
- Verify pickups spawn and can be collected
- Verify each room's encounter feels appropriate for its position in the floor
```

### Success Criteria
- [ ] Wave spawning has proper visual telegraphing
- [ ] Multi-wave encounters create dynamic pressure
- [ ] Encounter difficulty scales room-to-room
- [ ] Health pickups spawn, bob, and heal on contact
- [ ] Room completion feels rewarding (door opening fanfare)
- [ ] Full floor is playable start to finish

---

## Session 3.5 — Minimap & Floor Navigation

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Build the minimap and floor navigation UI.

### Create game/src/hud/minimap.rs

The minimap sits in the top-right corner of the screen. It shows:
- Discovered rooms as small rectangles (proportional to actual room size)
- Connections between rooms as lines
- Current room highlighted (bright outline)
- Room colors by type: combat=gray, treasure=yellow, shop=green, boss=red, exit=blue, start=white
- Undiscovered adjacent rooms shown as dim outlines (player knows they exist from seeing doors)
- Fully undiscovered rooms are invisible

### Minimap rendering:

```rust
pub struct Minimap {
    pub x: usize,        // screen position (top-right corner)
    pub y: usize,
    pub scale: f32,      // pixels per grid unit
    pub visible: bool,
}

impl Minimap {
    pub fn render(&self, fb: &mut FrameBuffer, floor: &FloorLayout, current_room: usize) {
        // Calculate minimap bounds
        // For each discovered room:
        //   Draw a small filled rectangle (3×2 to 6×4 pixels depending on room size)
        //   Color by room type
        //   Bright outline if current room
        // For each connection between discovered rooms:
        //   Draw a 1px line between room centers
        // For undiscovered rooms adjacent to discovered:
        //   Draw dim outline only (mystery!)
    }
}
```

Minimap should be:
- Small enough not to obstruct gameplay (~20×15 pixels in top-right)
- Clear enough to navigate by
- Toggle-able with Tab key (some players want max visibility)

### Floor completion UI:

When player reaches the Exit room:
- Show "FLOOR CLEARED" text
- Display stats: enemies killed, damage taken, rooms explored, time
- "Press Enter to descend"  
- Generate next floor (floor_number + 1, harder)
- Transition to new floor's Start room

### "Pause" screen:
Press Escape to pause:
- Darken the screen (overlay at 0.6 opacity)
- Show centered: full floor minimap (larger scale)
- Show: current HP, floor number, enemies killed
- Options: Resume (Escape), Quit (Q)
- Game loop still runs but game logic is frozen

### Test:
- Play through a full floor
- Verify minimap updates as rooms are discovered  
- Verify room types are color-coded correctly
- Verify pause screen shows correct info
- Complete the floor, descend to floor 2
- Verify floor 2 is harder (more enemies, higher stats)
```

### Success Criteria
- [ ] Minimap renders correctly in top-right
- [ ] Rooms appear as they're discovered
- [ ] Current room is highlighted
- [ ] Room type colors are intuitive
- [ ] Toggle with Tab works
- [ ] Pause screen shows full map
- [ ] Floor descent works and generates a harder floor

---

## Session 3.6 — First Boss Encounter

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Design the first floor boss.

### Boss: The Bone King (large skeleton boss)

### Visual (Visual Designer subtask):
- 20×24 pixels (double the size of normal enemies)
- Palette: bone white, dark maroon, gold crown, glowing red eyes
- Distinct crown or helmet to mark as boss
- Weapon: massive bone club or greatsword

### Boss health:
- 20 HP (vs player's ~3-hit damage, so ~7 hits to kill)
- Health bar at TOP of screen (not just hearts — a pixel-art bar showing remaining HP)
- Boss name displayed above health bar: "THE BONE KING"

### Phase 1 (100-50% HP): Basic attacks

State machine:
```rust
pub enum BoneKingPhase1 {
    Idle { timer: f32 },          // brief pause between attacks (0.5s)
    Slam { phase: SlamPhase },    // overhead slam
    Sweep { phase: SweepPhase },  // wide horizontal sweep
    Chase { timer: f32 },         // walk toward player if too far
}
```

**Slam Attack:**
1. Telegraph (0.6s): Boss raises weapon overhead, body flashes. Danger zone appears on ground (small rectangle in front of boss, colored red with 0.3 opacity)
2. Slam (0.15s): Weapon comes down. Hitbox active in the danger zone. Screen shake intensity 5.0
3. Recovery (0.4s): Weapon stuck in ground. Boss is vulnerable. Dust particles from impact.

**Sweep Attack:**
1. Telegraph (0.5s): Boss pulls weapon to one side, wide stance
2. Sweep (0.2s): Large horizontal hitbox (wider than slam). Covers ~30px wide arc in front.
3. Recovery (0.3s): Follow-through animation

**Pattern:** Alternates slam and sweep. Chases if player is >60px away.

### Phase 2 (50-0% HP): Enraged

Triggered when HP drops below 50%:
- Visual change: boss glows red, particles emanate constantly
- Movement speed +30%
- New attack: Charge
- Attack cooldowns reduced by 0.2s

**Charge Attack:**
1. Telegraph (0.4s): Boss stomps, screen shake, faces player
2. Charge (0.3s): Rush across the room toward player's position (100 px/sec). Hitbox covers boss body.
3. Hits wall: stunned for 1.0s (big vulnerability window). Dust/debris particles.

### Phase transition:
When crossing 50% HP:
- Hit pause: 6 frames
- Boss roars (animation: head tilts back)  
- Screen shake: intensity 6.0
- Color shift: boss sprites tint redder
- Brief invulnerability (0.5s) during roar

### Arena:
Use the Boss Arena template (26×20). No interior walls (clean arena). Some decorative floor pattern (different tile color in corners).

### Boss death:
1. Long hit pause (10 frames)
2. Boss flashes rapidly
3. Massive particle explosion (60+ particles)
4. Boss sprite disintegrates (render with increasing transparency over 1.5s)
5. Screen fade to white (instead of black — victory, not death)
6. "BONE KING DEFEATED" text
7. Reward drops: guaranteed boon choice + big heal + gold

### Test:
- Fight the boss multiple times
- Verify phase transition feels dramatic
- Verify each attack has clear telegraphs
- Verify the boss is challenging but fair (beatable without taking damage by a skilled player)
- Verify charge-into-wall stun creates satisfying punish window
- Tune HP and damage values
```

### Success Criteria
- [ ] Boss health bar renders at top of screen
- [ ] Phase 1 attacks have clear telegraphs
- [ ] Phase 2 is noticeably harder (faster, new attack)
- [ ] Phase transition is dramatic
- [ ] Boss is beatable without damage by a skilled player
- [ ] Charge-into-wall stun creates a skill-rewarding moment
- [ ] Death animation is spectacular

---

## Session 3.7 — Phase 3 Integration & Full Run Test

### Claude Code Prompt

```
You are the QA Lead for Cryptfall. Test a complete floor 1 run: start → explore → combat → boss → floor 2.

### Full playthrough test:

1. Start on floor 1, start room
2. Explore rooms, discover the map via minimap
3. Fight through 4-6 combat encounters with wave spawning
4. Find and use health pickups
5. Optionally visit treasure room
6. Fight The Bone King boss
7. Defeat boss, descend to floor 2
8. Verify floor 2 is harder
9. Play 2-3 rooms on floor 2
10. Die (or complete)

### Checklist:
- [ ] Floor generation produces good layouts (no awkward dead-ends, boss reachable)
- [ ] Room transitions are seamless (no flash, no position glitches)
- [ ] Wave spawning timing feels right (not too rushed, not too slow)
- [ ] Difficulty curve within a floor (early rooms easy, later hard)
- [ ] Boss fight is dramatic and challenging
- [ ] Floor descent works, floor 2 is noticeably harder
- [ ] Minimap is helpful for navigation
- [ ] Pause menu works mid-run
- [ ] Death anywhere in the floor restarts correctly (back to floor 1 start, for now)
- [ ] Total floor 1 run time: aim for 5-8 minutes

### Performance under full game load:
- Profile during boss fight (many particles, large sprite, hitboxes)
- Profile during heavy wave encounter (5+ enemies, projectiles, particles)
- Check memory: any growth over a 15-minute play session?

### Write docs/phase3-review.md with:
- Full playthrough notes
- Bugs found
- Balance observations (too easy? too hard? boring rooms?)
- Flow observations (does the pacing feel right?)
- Recommendations for Phase 4 (what the game needs next to feel complete)
```

### Success Criteria
- [ ] Complete run is possible from start to floor 2
- [ ] Run takes 5-8 minutes (good pacing)
- [ ] No crashes during full playthrough
- [ ] Performance stable throughout
- [ ] Game feels like a coherent experience, not a tech demo
- [ ] Phase 3 review written with specific observations

---

## Phase 3 File Manifest

```
crates/game/src/
├── dungeon/
│   ├── mod.rs
│   ├── room_template.rs    # Template structs + parser
│   ├── templates.rs         # All hand-crafted room templates
│   ├── floor_gen.rs         # Procedural floor layout generator
│   ├── encounters.rs        # Wave spawning, encounter definitions
│   └── world.rs             # Active dungeon state, room transitions
├── enemies/
│   └── bone_king.rs         # Boss AI, phases, attacks
├── hud/
│   ├── mod.rs
│   ├── minimap.rs           # Floor map display
│   ├── boss_bar.rs          # Boss health bar
│   └── health.rs            # Player hearts (moved from hud.rs)
├── pickup.rs                # Health/gold pickups
├── sprites/
│   ├── boss.rs              # Bone King sprite sheet
│   ├── pickups.rs           # Heart, coin sprites
│   └── doors.rs             # Door open/closed sprites
└── main.rs                  # Full game loop with floor progression

docs/
└── phase3-review.md
```
