# Phase 2: Combat Core — Fighting Enemies with Hades-Style Feel

## Overview
**Duration:** Weeks 7–11 (10–15 sessions)
**Goal:** Player fights enemies in an arena room with dash, attack, knockback, particles, and satisfying game feel.
**Prerequisite:** Phase 1 complete (sprites, animation, tilemap, camera, player movement)

This is the make-or-break phase. Combat feel is what separates "tech demo" from "game people want to play." Every session focuses on a different layer of juice.

---

## Session 2.1 — Hitbox System & Basic Attack

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. The engine handles sprites, animation, tiles, and camera. Now implement the combat hitbox system.

### Create engine/src/collision.rs (or expand existing)

```rust
/// Axis-Aligned Bounding Box in pixel coordinates
#[derive(Clone, Copy)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl AABB {
    pub fn overlaps(&self, other: &AABB) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }
    
    /// Offset by a position (useful for getting world-space box from local offset)
    pub fn at(&self, px: f32, py: f32) -> AABB {
        AABB { x: self.x + px, y: self.y + py, w: self.w, h: self.h }
    }
    
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }
}
```

### Combat hitbox design

Each entity has TWO boxes:
1. **Hurtbox** — the area where they can be hit. Roughly body-sized. Always active.
2. **Hitbox** — the area where their attack deals damage. Only active during specific animation frames.

For the player:
- Hurtbox: 6×8 pixels, centered on lower body (offset from sprite top-left)
- Attack hitbox: 10×8 pixels, positioned in FRONT of the player (depends on facing direction)
  - Facing right: hitbox starts at player's right edge
  - Facing left: hitbox starts at player's left edge minus hitbox width

### Modify player.rs attack logic:

```rust
pub struct Player {
    // ... existing fields
    pub hurtbox_offset: AABB,      // relative to position
    pub attack_hitbox_offset: AABB, // relative to position (right-facing)
    pub attack_active: bool,        // true only during active hitbox frames
    pub attack_cooldown: f32,       // time until can attack again
    pub invincible_timer: f32,      // i-frames from dash or hit stagger
}
```

Attack flow:
1. Press Attack → enter Attacking state, play attack animation, set cooldown
2. Attack animation frame 2 (the swing follow-through): `attack_active = true`
3. Attack animation frame 3 (recovery): `attack_active = false`
4. Attack animation finishes → return to Idle/Walk
5. Cannot attack again until cooldown expires (0.3s)

The hitbox is only checked against enemies during the 1-2 frames it's active. This creates a "window" that feels like a real swing.

### Create a basic Enemy struct

In game/src/enemy.rs:

```rust
pub struct Enemy {
    pub transform: Transform,
    pub animation: AnimationPlayer,
    pub hurtbox_offset: AABB,
    pub hp: i32,
    pub max_hp: i32,
    pub alive: bool,
    pub facing_right: bool,
    pub flash_timer: f32,   // when hit, flash white for a few frames
}
```

For now, create a static dummy enemy that:
- Stands still (idle animation)
- Has a hurtbox
- Takes damage when the player's attack hitbox overlaps its hurtbox
- Flashes white (use tinted blit) when hit
- Dies at 0 HP (play death animation)

### Test scene:
- Place 3 dummy enemies in the room
- Player can walk up and attack them
- Each hit deals 1 damage, enemies have 3 HP
- Visual feedback on hit: enemy flashes white, slight knockback (push 4px away from player)
- Enemy death: play death animation, then remove

No enemy AI yet — they just stand there and take hits.
```

### Success Criteria
- [ ] Player attack animation plays correctly
- [ ] Hitbox only registers during active frames (can't hit by standing next to enemy)
- [ ] Enemy flashes white on hit
- [ ] Enemy takes damage and dies at 0 HP
- [ ] Knockback pushes enemy away from player
- [ ] Attack cooldown prevents spam

---

## Session 2.2 — Hit Feedback & Game Feel

### Claude Code Prompt

```
You are the Game Designer for Cryptfall, focusing on GAME FEEL. The basic hit detection works. Now add the layers that make combat satisfying.

### 1. Hit Pause (Freeze Frames)
The single most impactful juice technique. When the player's attack connects:
- Freeze EVERYTHING for 3 frames (100ms at 30 FPS)
- During freeze: nothing moves, animations pause, input is still read but not applied
- After freeze: resume normally

Implementation in the game loop:
```rust
pub struct HitPause {
    pub remaining_frames: u32,
}

// In update:
if hit_pause.remaining_frames > 0 {
    hit_pause.remaining_frames -= 1;
    return;  // skip all game logic this tick
}
```

This tiny freeze creates the feeling of IMPACT. It's what makes Hades, Hollow Knight, and Celeste feel so good.

### 2. Screen Shake on Hit
Trigger camera shake when an attack connects:
- Normal hit: intensity 2.5, decay 0.8
- Kill hit: intensity 5.0, decay 0.85
- Boss hit: intensity 4.0, decay 0.82

### 3. Knockback Physics
When an enemy is hit:
- Calculate knockback direction: vector from player center to enemy center, normalized
- Apply knockback velocity: 120 px/sec in that direction
- Knockback decays over 0.2 seconds (friction)
- Enemy collides with walls during knockback (doesn't clip through)

```rust
// In enemy struct:
pub knockback_velocity: Vec2,

// In update:
self.knockback_velocity.x *= 0.85_f32.powf(dt * 30.0);  // friction
self.knockback_velocity.y *= 0.85_f32.powf(dt * 30.0);
self.transform.position.x += self.knockback_velocity.x * dt;
self.transform.position.y += self.knockback_velocity.y * dt;
// + wall collision check
```

### 4. Damage Numbers
When damage is dealt, spawn a floating number:

```rust
pub struct DamageNumber {
    pub value: i32,
    pub x: f32,
    pub y: f32,
    pub velocity_y: f32,  // floats upward
    pub lifetime: f32,
    pub color: Color,
}
```

- Spawn at the hit point (enemy center)
- Float upward at 30 px/sec
- Fade out over 0.8 seconds
- Color: white for normal damage, yellow for critical

For rendering text as pixels: create a tiny 3×5 pixel font for digits 0-9. Each digit is a const array.

### 5. Enemy Hit Stagger
When hit, the enemy enters a brief stagger state:
- Cannot act for 0.2 seconds
- Animation: play hit animation (slight recoil)
- After stagger: resume normal behavior

### 6. Player Attack "Active Frame" Visualization (debug)
Add a debug toggle (press F1) that draws hitboxes and hurtboxes as colored rectangle outlines:
- Green outline: hurtboxes
- Red outline: active hitboxes
- Blue outline: inactive hitboxes

This is invaluable for tuning and will be used extensively in later sessions.

### Test scene:
Place 5 enemies with 3 HP each. Fight them all. Every hit should feel IMPACTFUL thanks to the combined effect of hit-pause + shake + knockback + flash + damage numbers.

Record your subjective experience: does it feel good? What needs tuning?
```

### Success Criteria
- [ ] Hit pause creates a visible "punch" on every hit
- [ ] Screen shake accompanies hits
- [ ] Knockback pushes enemies visibly with wall collision
- [ ] Damage numbers float up and fade
- [ ] Debug hitbox display works (toggleable)
- [ ] Combined effect feels satisfying (the "game feel" test)

---

## Session 2.3 — Particle System

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Combat feels good. Now build the particle system to add visual flair.

### Create engine/src/particle.rs

```rust
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: u8,        // 1 = single pixel, 2 = 2x2 block
    pub gravity: f32,    // downward acceleration (0 for no gravity)
    pub friction: f32,   // velocity decay per second (0.95 = light, 0.8 = heavy)
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,  // hard cap (e.g., 500)
}

impl ParticleSystem {
    pub fn new(max: usize) -> Self { ... }
    
    pub fn spawn(&mut self, particle: Particle) {
        if self.particles.len() < self.max_particles {
            self.particles.push(particle);
        }
    }
    
    /// Spawn multiple particles with randomized parameters
    pub fn burst(&mut self, config: &BurstConfig) { ... }
    
    pub fn update(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.vy += p.gravity * dt as f32;
            p.vx *= p.friction.powf(dt as f32 * 30.0);
            p.vy *= p.friction.powf(dt as f32 * 30.0);
            p.x += p.vx * dt as f32;
            p.y += p.vy * dt as f32;
            p.lifetime -= dt as f32;
        }
        self.particles.retain(|p| p.lifetime > 0.0);
    }
    
    pub fn render(&self, fb: &mut FrameBuffer, camera_x: i32, camera_y: i32) {
        for p in &self.particles {
            let alpha = (p.lifetime / p.max_lifetime).clamp(0.0, 1.0);
            // Fade: lerp particle color toward black based on remaining lifetime
            let faded = [
                (p.color[0] as f32 * alpha) as u8,
                (p.color[1] as f32 * alpha) as u8,
                (p.color[2] as f32 * alpha) as u8,
            ];
            let sx = p.x as i32 - camera_x;
            let sy = p.y as i32 - camera_y;
            fb.set_pixel_safe(sx, sy, faded);
            if p.size >= 2 {
                fb.set_pixel_safe(sx+1, sy, faded);
                fb.set_pixel_safe(sx, sy+1, faded);
                fb.set_pixel_safe(sx+1, sy+1, faded);
            }
        }
    }
}

pub struct BurstConfig {
    pub x: f32,
    pub y: f32,
    pub count: usize,
    pub speed_min: f32,
    pub speed_max: f32,
    pub angle_min: f32,    // radians
    pub angle_max: f32,
    pub lifetime_min: f32,
    pub lifetime_max: f32,
    pub colors: &'static [Color],  // random color from this set
    pub size: u8,
    pub gravity: f32,
    pub friction: f32,
}
```

Use `fastrand` crate for random values (tiny, fast, no-std compatible).

### Particle effects to implement:

1. **Hit Sparks** — on weapon connecting with enemy:
   - 8-12 particles, burst from hit point
   - Direction: spread 120° centered on hit direction
   - Colors: white, yellow, orange
   - Speed: 60-120 px/sec
   - Lifetime: 0.15-0.3s
   - Size: 1px
   - No gravity, high friction (0.85)

2. **Dash Trail** — during dash:
   - Spawn 2 particles per frame at player position
   - Colors: light blue, white (matching armor)
   - Speed: 10-20 px/sec (slight random drift)
   - Lifetime: 0.2-0.4s
   - Size: 1px
   - No gravity, low friction

3. **Enemy Death Explosion** — when enemy HP reaches 0:
   - 20-30 particles, burst in all directions (360°)
   - Colors: match enemy palette + red
   - Speed: 40-100 px/sec
   - Lifetime: 0.3-0.6s
   - Size: mix of 1px and 2px
   - Slight gravity (30), medium friction

4. **Dust Puff** — when player changes direction or lands:
   - 4-6 particles at player's feet
   - Colors: brown, tan (stone dust)
   - Direction: spread 180° upward
   - Speed: 20-40 px/sec
   - Lifetime: 0.2-0.3s
   - Size: 1px
   - Light gravity (20)

5. **Blood/Impact** — when player takes damage:
   - 6-10 particles from player position
   - Colors: red, dark red
   - Spread: 180° away from damage source
   - Speed: 40-80 px/sec
   - Lifetime: 0.3-0.5s
   - Medium gravity (40)

### Integration:
- Add ParticleSystem to the game state
- Trigger appropriate effects during combat events
- Render particles AFTER sprites but BEFORE HUD
- Particles should be camera-relative (move with the world, not the screen)

The particle system should handle 200+ simultaneous particles without dropping below 30 FPS.
```

### Success Criteria
- [ ] Hit sparks fly on every weapon connect
- [ ] Dash leaves a visible trail
- [ ] Enemy death produces satisfying explosion
- [ ] Dust puffs on direction changes
- [ ] Particles fade out smoothly
- [ ] 200+ particles doesn't impact FPS
- [ ] Particles are culled correctly when off-camera

---

## Session 2.4 — Enemy AI: Melee Chaser

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Particle effects are in. Now create the first real enemy with AI.

### Enemy Type: Skeleton Warrior (Melee Chaser)

A basic melee enemy that embodies the core AI loop: patrol → detect → chase → attack → cooldown.

### Create game/src/enemies/skeleton.rs

State machine:
```rust
pub enum SkeletonState {
    Idle { timer: f32 },           // Stand still for 1-2 seconds
    Patrol { target_x: f32 },     // Walk to random nearby point
    Chase,                         // Move toward player
    WindUp { timer: f32 },        // Telegraph attack (0.4s)
    Attack { timer: f32 },        // Lunge attack (0.2s)  
    Cooldown { timer: f32 },      // Recovery after attack (0.6s)
    Stagger { timer: f32 },       // Hit stagger (0.3s)
    Dead,
}
```

### Behavior:

**Idle/Patrol (player far away, >80px):**
- Alternate between standing idle (1-2s) and walking to random nearby point (within 30px)
- Walk speed: 25 px/sec (slow shuffle)

**Detection (player within 80px line of sight):**
- Transition to Chase
- "!" visual indicator (1-pixel exclamation mark above head for 0.3s)

**Chase (player within 80px but >20px):**
- Walk toward player at 40 px/sec
- Face the player
- Play walk animation

**Wind-up (player within 20px, attack range):**
- Stop moving
- Play telegraph animation (weapon raised, body leans back)
- Duration: 0.4 seconds — this is the player's window to dodge
- Visual telegraph: the enemy flashes slightly brighter
- CRITICAL: the telegraph must be CLEARLY READABLE. Players need time to react.

**Attack (after wind-up):**
- Lunge forward 15px in player direction
- Attack hitbox active for 0.15 seconds
- If hits player: deal 1 damage
- Total attack duration: 0.2 seconds

**Cooldown (after attack):**
- Cannot act for 0.6 seconds
- Slightly vulnerable (lower defense if you add that later)
- Returns to Chase if player still in range, else Idle

**Stagger (when hit by player):**
- Interrupts any state except Dead
- Cannot act for 0.3 seconds
- Receives knockback
- After stagger: returns to Chase

**Death:**
- Play death animation
- Spawn death particles
- After animation: mark for removal

### Enemy stats:
- HP: 3
- Damage: 1
- Walk speed: 25 (patrol), 40 (chase)
- Detection range: 80px
- Attack range: 20px

### Visual Designer subtask — Skeleton sprites:

Create skeleton enemy sprites (10×14, same size as player):
- Palette: bone white [220,210,190], dark [60,50,40], red eyes [200,40,40], weapon gray [150,150,155]
- Animations: idle (2 frames), walk (4 frames), wind-up (2 frames), attack (2 frames), stagger (1 frame), death (3 frames)

### Create game/src/enemies/mod.rs:
- Define an EnemyType enum
- Use a common Enemy struct with type-specific behavior via a trait or match on type
- Support spawning enemies at positions

### Test scene:
- Large room with 5 skeleton warriors at different positions
- Player fights them
- Verify: enemies detect, chase, telegraph, attack, take hits, stagger, die
- Verify: player can dodge telegraphed attacks with dash
- Verify: enemies collide with walls and each other (basic separation)
```

### Success Criteria
- [ ] Skeletons patrol when player is far away
- [ ] Skeletons detect and chase the player
- [ ] Attack telegraph is clearly visible (player has time to react)
- [ ] Dash i-frames let the player dodge through attacks
- [ ] Hitting a skeleton interrupts its attack (stagger)
- [ ] Skeletons die with proper animation + particles
- [ ] Multiple skeletons don't stack perfectly on each other (basic separation)

---

## Session 2.5 — Enemy AI: Ranged Shooter

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Melee enemies work. Now add a ranged enemy type to create combat variety.

### Enemy Type: Ghost Mage (Ranged Shooter)

### Create game/src/enemies/ghost.rs

State machine:
```rust
pub enum GhostState {
    Float { timer: f32 },          // Hover in place
    Reposition { target: Vec2 },   // Move to new vantage point
    Aim { timer: f32 },            // Line up shot (0.6s telegraph)
    Shoot,                          // Fire projectile
    Cooldown { timer: f32 },       // 1.0s between shots
    Stagger { timer: f32 },
    Dead,
}
```

### Behavior:

**Float/Reposition:**
- Ghost prefers to keep 50-70px distance from player
- If player gets closer than 40px: reposition away
- If player is farther than 80px: reposition closer
- Movement: 30 px/sec, slightly floaty (sine-wave vertical bob while moving)
- Ghost CAN float over pits (ignores pit tiles, but NOT walls)

**Aim (player in range, 30-100px):**
- Stop moving
- Telegraph: ghost glows brighter, a small indicator dot appears between ghost and player showing the aim line
- Duration: 0.6 seconds
- The aim direction is set at START of aim phase (player can dodge after telegraph)

**Shoot:**
- Spawn a projectile in the aimed direction
- Projectile speed: 80 px/sec
- Projectile is a 3×3 pixel bright orb (purple/pink)
- Projectile has a small particle trail

**Projectile behavior:**
```rust
pub struct Projectile {
    pub transform: Transform,
    pub velocity: Vec2,
    pub hitbox: AABB,      // 3×3
    pub damage: i32,
    pub lifetime: f32,      // despawn after 2 seconds
    pub owner: ProjectileOwner,  // Enemy or Player (for future player projectiles)
    pub color: Color,
    pub trail_timer: f32,
}
```

Projectiles:
- Move in a straight line at constant speed
- Despawn on hitting a wall (spawn small impact particles)
- Despawn on hitting the player (deal damage, spawn hit particles)
- Leave a particle trail (1 particle every 0.05s)
- Do NOT hit other enemies (pass through)

### Create game/src/projectile.rs:
- Manage a Vec<Projectile>
- Update positions each tick
- Check collision with player hurtbox and with tilemap walls
- Render as colored pixels with glow effect (center pixel bright, surrounding pixels dimmer)

### Ghost sprites (Visual Designer subtask):
- 10×12 pixels (slightly smaller than player — ghosts float)
- Palette: purple [120,60,160], light purple [180,140,220], white glow [240,230,255], dark [40,20,60]
- Semi-transparent look: some interior pixels are darker background blended
- Animations: float (2 frames, slow bob), aim (2 frames, glow increasing), shoot (1 frame, recoil), stagger (1 frame), death (3 frames, dissolve)

### Test scene:
- Room with 3 skeletons and 2 ghost mages
- Ghosts stay at range and shoot
- Projectiles are visible and dodgeable
- Mix of melee and ranged creates interesting combat dynamics
- Player must manage positioning: dash through melee, dodge projectiles
```

### Success Criteria
- [ ] Ghosts maintain distance from player
- [ ] Aim telegraph is clearly visible
- [ ] Projectiles travel in a straight line and despawn on walls
- [ ] Projectile trail particles look good
- [ ] Player can dodge projectiles with dash
- [ ] Mixed melee + ranged combat feels dynamic
- [ ] Projectiles don't hit other enemies

---

## Session 2.6 — Player Health & Damage

### Claude Code Prompt

```
You are the Game Designer for Cryptfall. Enemies can now attack. Implement player health and damage.

### Player damage system:

Add to player.rs:
```rust
pub hp: i32,           // current health (start at 5)
pub max_hp: i32,       // maximum health (5)
pub invincible_timer: f32,  // i-frames after taking damage (1.0s)
pub dash_invincible: bool,  // true during dash
```

### When player takes damage:
1. Check: if invincible_timer > 0 OR dash_invincible → ignore damage
2. Reduce HP by damage amount
3. Set invincible_timer = 1.0 seconds (generous i-frames)
4. Knockback player away from damage source
5. Hit pause: 4 frames (slightly longer than enemy hit pause)
6. Camera shake: intensity 4.0
7. Spawn blood/impact particles
8. Flash player sprite: alternate between normal and white every 2 frames during i-frames
9. Play hit animation
10. If HP ≤ 0: trigger death sequence

### Dash i-frames:
- During the dash state, set dash_invincible = true
- After dash ends, set dash_invincible = false
- This is the primary defensive mechanic

### Player death sequence:
1. Hit pause: 8 frames (long dramatic pause)
2. Camera shake: intensity 8.0
3. Play death animation
4. Particles: big burst of player-colored particles
5. Screen slowly fades to dark (over 1.5 seconds)
6. Show death screen (for now, just "YOU DIED - Press R to restart")
7. On R: reset room (all enemies respawn, player returns to start position with full HP)

### HUD — Health display:

Create game/src/hud.rs:

Render player health as pixel-art hearts in the top-left corner:
- Full heart: 5×5 pixel red heart sprite
- Empty heart: 5×5 pixel dark gray heart outline
- Space between hearts: 1px
- Position: (2, 2) in screen space (not affected by camera)

When taking damage, the lost heart should:
- Flash white → red → disappear over 0.3 seconds
- Slight bounce animation (heart jumps up 2px then settles)

### Screen fade effect:

Add to renderer: ability to apply a color overlay to the entire framebuffer with variable opacity.

```rust
impl FrameBuffer {
    /// Apply a color overlay with given opacity (0.0 = no effect, 1.0 = solid color)
    pub fn overlay(&mut self, color: Color, opacity: f32) {
        for pixel in &mut self.pixels {
            if let Some(c) = pixel {
                *c = [
                    lerp(c[0], color[0], opacity),
                    lerp(c[1], color[1], opacity),
                    lerp(c[2], color[2], opacity),
                ];
            }
        }
    }
}
```

Use this for: death screen fade, room transitions, pause menu darkening.

### Test:
- Fight enemies, take damage, see health decrease
- Verify i-frames (flashing sprite, can't be hit again for 1 second)
- Dash through enemy attacks to test dodge
- Die and see the death sequence
- Restart and verify everything resets
- Make sure HUD hearts are visible and don't overlap with gameplay
```

### Success Criteria
- [ ] Player takes damage from enemy attacks and projectiles
- [ ] I-frames prevent damage stacking
- [ ] Dash provides reliable dodge window
- [ ] Health hearts display correctly
- [ ] Heart loss animation is satisfying
- [ ] Death sequence is dramatic (long hit-pause, shake, fade)
- [ ] Restart works cleanly

---

## Session 2.7 — Phase 2 Integration & Combat Polish

### Claude Code Prompt

```
You are the QA Lead for Cryptfall. This is the critical Phase 2 review. Combat must feel good.

### The Arena Test

Create a dedicated test room: 20×15 tile arena with no interior walls (open space). Spawn waves:
- Wave 1: 3 skeletons
- Wave 2: 2 skeletons + 1 ghost (spawns after wave 1 cleared)
- Wave 3: 4 skeletons + 2 ghosts (spawns after wave 2 cleared)

Play through all 3 waves. Evaluate:

### Game Feel Checklist:
- [ ] Does attacking feel powerful? (hit pause + shake + sparks + knockback)
- [ ] Is the attack window satisfying? (not too wide, not too narrow)
- [ ] Can you reliably dodge with dash? (i-frames generous enough?)
- [ ] Do enemy telegraphs give enough reaction time?
- [ ] Is enemy chase speed fair? (can you kite without it feeling trivial?)
- [ ] Do mixed melee+ranged encounters force interesting positioning?
- [ ] Does the death feel dramatic enough to motivate a retry?
- [ ] Does the HUD stay readable during intense combat?

### Tuning Values Review:
Document the current values and suggest adjustments:
- Player speed, dash speed, dash duration, dash cooldown
- Attack hitbox size, active frames, cooldown
- Player HP, i-frame duration
- Skeleton chase speed, attack range, telegraph time, attack damage
- Ghost reposition range, aim time, projectile speed
- Hit pause durations (player hit, enemy hit, kill)
- Screen shake intensities
- Particle counts and lifetimes

### Bug Checklist:
- [ ] Kill an enemy during its attack — does the attack still damage you?
- [ ] Dash into a wall at high speed — any clipping?
- [ ] Get hit by a projectile and melee simultaneously — double damage?
- [ ] Kill all enemies — does the game handle empty enemy list?
- [ ] Resize terminal during combat — does HUD adjust?
- [ ] Die during dash — any state machine issues?
- [ ] Stagger an enemy during wind-up — does it properly cancel?

### Performance Check:
- 5 enemies + 30 projectiles + 200 particles: steady 30 FPS?
- Profile the hot path: what's the most expensive operation?
- Any frame drops during heavy particle bursts?

### Visual Polish:
- Are enemy sprites distinct from each other and from the player?
- Is the attack hitbox visually aligned with the weapon swing?
- Do particles enhance readability or create noise?
- Is the color contrast sufficient? (especially ghost projectiles against dark floors)

Write a detailed report in docs/phase2-review.md with tuning recommendations.
```

### Success Criteria
- [ ] All 3 waves completable by a skilled player
- [ ] No game-breaking bugs found
- [ ] Performance stable at 30 FPS under load
- [ ] Combat feels satisfying (subjective but critical)
- [ ] Tuning recommendations documented
- [ ] Phase 2 review written

---

## Phase 2 File Manifest

```
crates/engine/src/
├── collision.rs     # AABB, overlap detection
├── particle.rs      # ParticleSystem, BurstConfig
└── (framebuffer.rs) # + overlay method, set_pixel_safe

crates/game/src/
├── player.rs        # + combat, health, i-frames, death
├── enemy.rs         # Common enemy struct & trait
├── enemies/
│   ├── mod.rs
│   ├── skeleton.rs  # Melee chaser AI
│   └── ghost.rs     # Ranged shooter AI
├── projectile.rs    # Projectile management
├── combat.rs        # Hitbox resolution, damage events
├── hud.rs           # Health hearts, debug info
├── sprites/
│   ├── player.rs    # + attack, hit, death frames
│   ├── skeleton.rs  # Skeleton sprite sheet
│   ├── ghost.rs     # Ghost sprite sheet
│   ├── effects.rs   # Projectile sprites, heart sprites
│   └── font.rs      # Tiny pixel font for damage numbers
└── main.rs          # Arena test scene

docs/
├── phase2-review.md
└── tuning-values.md  # All combat numbers in one place
```
