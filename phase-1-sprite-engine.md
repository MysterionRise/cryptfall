# Phase 1: Sprite Engine — Animated Characters in a Tiled World

## Overview
**Duration:** Weeks 4–6 (6–9 sessions)
**Goal:** An animated player character walking through a tiled room with camera follow.
**Prerequisite:** Phase 0 complete (framebuffer, diff renderer, input, game loop all working)

---

## Session 1.1 — Sprite Data Format & Blitting

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Phase 0 is complete — we have a working framebuffer with half-block rendering, diff-based output, input system, and fixed-timestep game loop.

Now build the sprite system. Create engine/src/sprite.rs:

### Sprite Data Format

Sprites are stored as compile-time constant arrays. Each sprite is a grid of pixels where None = transparent:

```rust
pub struct SpriteData {
    pub width: usize,
    pub height: usize,
    pub pixels: &'static [Option<Color>],  // row-major, length = width * height
}

impl SpriteData {
    pub const fn new(width: usize, height: usize, pixels: &'static [Option<Color>]) -> Self {
        SpriteData { width, height, pixels }
    }
    
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            None
        }
    }
}
```

### Sprite Blitting

Add a method to FrameBuffer:

```rust
impl FrameBuffer {
    /// Blit a sprite at pixel position (px, py). 
    /// Handles clipping (sprite partially off-screen).
    /// Transparent pixels (None) are skipped — they don't overwrite the background.
    pub fn blit_sprite(&mut self, sprite: &SpriteData, px: i32, py: i32) { ... }
    
    /// Blit with horizontal flip (for left-facing sprites)
    pub fn blit_sprite_flipped(&mut self, sprite: &SpriteData, px: i32, py: i32) { ... }
    
    /// Blit with a color tint (multiply each pixel color by tint)
    /// Useful for damage flash (tint red), ghost trail (tint with reduced brightness)
    pub fn blit_sprite_tinted(&mut self, sprite: &SpriteData, px: i32, py: i32, tint: Color) { ... }
}
```

IMPORTANT: px and py are SIGNED (i32) because sprites can be partially off-screen (negative positions). The blitting must handle this correctly by calculating the visible region.

### Create a test sprite

In game/src/sprites.rs, create a simple test sprite by hand — a 8×12 pixel character (roughly humanoid silhouette) using NES-style colors:

```rust
use engine::sprite::SpriteData;
use engine::color::Color;

const N: Option<Color> = None;  // transparent
const W: Option<Color> = Some([255, 255, 255]);  // white
const S: Option<Color> = Some([240, 180, 140]);  // skin
const H: Option<Color> = Some([139, 69, 19]);    // hair/brown
const B: Option<Color> = Some([30, 100, 200]);   // blue clothing

pub static PLAYER_TEST: SpriteData = SpriteData::new(8, 12, &[
    // 8 wide × 12 tall — a tiny person
    N, N, H, H, H, H, N, N,  // row 0: hair top
    N, H, H, H, H, H, H, N,  // row 1: hair 
    N, H, S, S, S, S, H, N,  // row 2: face top
    N, N, S, S, S, S, N, N,  // row 3: face
    N, N, N, S, S, N, N, N,  // row 4: neck
    N, B, B, B, B, B, B, N,  // row 5: torso
    N, B, B, B, B, B, B, N,  // row 6: torso
    N, B, B, B, B, B, B, N,  // row 7: torso
    N, N, B, B, B, B, N, N,  // row 8: waist
    N, N, B, N, N, B, N, N,  // row 9: legs
    N, N, B, N, N, B, N, N,  // row 10: legs
    N, N, H, N, N, H, N, N,  // row 11: boots
]);
```

### Update game/src/main.rs:
- Remove the test square
- Blit the PLAYER_TEST sprite at a position
- Move it with arrow keys (using the existing input system + Transform)
- Test horizontal flip: sprite faces right by default, flip when moving left
- Test tinting: press Attack to flash the sprite red for 5 frames
- Keep the gradient background to verify transparency works

`cargo run` should show a tiny pixel character walking over the gradient.
```

### Success Criteria
- [ ] Sprite renders correctly with transparency (gradient shows through None pixels)
- [ ] Horizontal flip works (character faces movement direction)
- [ ] Tinting works (red flash on attack press)
- [ ] Sprite clips correctly when partially off-screen (no crash, no garbage)
- [ ] Moving the sprite only redraws affected cells (diff rendering still works)

---

## Session 1.2 — Animation System

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Sprites render and blit correctly. Now add animation.

### Create engine/src/animation.rs

```rust
/// A sequence of sprite frames with timing
pub struct AnimationData {
    pub frames: &'static [&'static SpriteData],  // ordered frame list
    pub frame_duration: f64,                       // seconds per frame
    pub looping: bool,                             // loop or play once?
}

/// Runtime animation state
pub struct AnimationPlayer {
    current_animation: &'static AnimationData,
    current_frame: usize,
    elapsed: f64,
    finished: bool,  // true when a non-looping animation completes
    flipped: bool,   // horizontal flip
}

impl AnimationPlayer {
    pub fn new(animation: &'static AnimationData) -> Self { ... }
    
    /// Switch to a new animation. Resets frame to 0.
    /// If already playing this animation, does nothing (prevents restart stutter).
    pub fn play(&mut self, animation: &'static AnimationData) {
        if std::ptr::eq(self.current_animation, animation) { return; }
        // ... reset state
    }
    
    /// Advance animation by dt seconds
    pub fn update(&mut self, dt: f64) { ... }
    
    /// Get the current frame's sprite data
    pub fn current_sprite(&self) -> &'static SpriteData { ... }
    
    /// Is a one-shot animation finished?
    pub fn is_finished(&self) -> bool { self.finished }
    
    pub fn set_flipped(&mut self, flipped: bool) { self.flipped = flipped; }
    pub fn is_flipped(&self) -> bool { self.flipped }
}
```

### Animation timing
At 30 FPS game tick rate:
- 4-frame walk cycle at 0.15s per frame = 0.6s full cycle (feels natural)
- 3-frame attack at 0.08s per frame = 0.24s (snappy)
- 2-frame idle "breathing" at 0.5s per frame = 1.0s cycle (subtle)

### Create player animation set

In game/src/sprites.rs, create animation frames for the player:

1. **Idle** (2 frames, looping): slight up-down bob. Second frame is 1 pixel higher than first.
2. **Walk** (4 frames, looping): simple walk cycle. Legs alternate, body bobs.
3. **Dash** (1 frame, used with motion blur effect): crouched pose

Each frame is an 8×12 sprite. Keep the same color scheme as PLAYER_TEST.

You can create the walk cycle by:
- Frame 0: neutral stance (legs together)
- Frame 1: right leg forward, left leg back, body 1px lower
- Frame 2: neutral stance again  
- Frame 3: left leg forward, right leg back, body 1px lower

The 1px vertical bob during walking makes a HUGE difference in perceived quality.

### Animation state machine in game code

In game/src/player.rs, create a player struct:

```rust
pub enum PlayerState {
    Idle,
    Walking,
    Dashing,
}

pub struct Player {
    pub transform: Transform,
    pub state: PlayerState,
    pub animation: AnimationPlayer,
    pub facing_right: bool,
    pub speed: f32,
    pub dash_speed: f32,
    pub dash_timer: f32,
}
```

State transitions:
- No input → Idle (plays idle animation)
- Direction held → Walking (plays walk animation, flipped based on direction)
- Dash pressed → Dashing for 0.15s (plays dash frame, moves at dash_speed)
- Dash ends → back to Idle or Walking based on input

### Update game/src/main.rs:
- Create a Player with the animation system
- Handle state transitions based on input
- Render using animation.current_sprite() with correct flip
- Speed: 60 pixels/sec walking, 200 pixels/sec dashing

The character should visibly animate while walking and snap into a dash pose during dash.
```

### Success Criteria
- [ ] Idle animation plays subtle bob when standing still
- [ ] Walk animation cycles through 4 frames while moving
- [ ] Character faces direction of movement (sprite flips)
- [ ] Dash has a distinct visual (different pose + fast movement)
- [ ] Switching animations doesn't stutter or restart unnecessarily
- [ ] Animation timing feels natural at 30 FPS

---

## Session 1.3 — Tile Map System

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Player animation works. Now build the tile map renderer.

### Create engine/src/tilemap.rs

```rust
pub const TILE_SIZE: usize = 8;  // 8×8 pixels per tile

/// A tile in the map
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    WallTop,     // top edge of wall (different visual)
    Door,
    Pit,         // hazard/void
}

/// A room made of tiles
pub struct TileMap {
    pub width: usize,    // in tiles
    pub height: usize,   // in tiles  
    pub tiles: Vec<TileType>,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self { ... }
    pub fn get(&self, tx: usize, ty: usize) -> TileType { ... }
    pub fn set(&mut self, tx: usize, ty: usize, tile: TileType) { ... }
    pub fn is_solid(&self, tx: usize, ty: usize) -> bool { ... } // Wall and WallTop are solid
    
    /// Check if a pixel-space rectangle collides with any solid tile
    pub fn collides(&self, x: f32, y: f32, w: f32, h: f32) -> bool { ... }
    
    /// Pixel dimensions
    pub fn pixel_width(&self) -> usize { self.width * TILE_SIZE }
    pub fn pixel_height(&self) -> usize { self.height * TILE_SIZE }
}
```

### Tile sprites

In game/src/tiles.rs, create 8×8 sprite data for each tile type:

1. **Floor**: Dark gray stone pattern. Use 2-3 shades of gray with a subtle brick pattern. 
   Colors: [40,40,45], [45,45,50], [35,35,40]

2. **Wall**: Darker, more solid. Visible brick lines.
   Colors: [60,55,50], [50,45,40], [70,65,55]

3. **WallTop**: The top face of walls (player sees this as a ledge). Slightly lighter.
   Colors: [80,75,65], [70,65,55]

4. **Door**: Distinct color — warm wood tones.
   Colors: [120,80,40], [100,65,30], [140,95,50]

5. **Pit**: Very dark, almost black with subtle purple.
   Colors: [15,10,20], [10,5,15]

### Tile map rendering

Add a method to render the tilemap to the framebuffer, given a camera offset:

```rust
pub fn render_tilemap(
    fb: &mut FrameBuffer, 
    tilemap: &TileMap, 
    tile_sprites: &HashMap<TileType, &SpriteData>,
    camera_x: i32,  // pixel offset of camera top-left
    camera_y: i32,
) {
    // Only render tiles visible in the viewport
    // Calculate which tile range is visible
    // Blit each visible tile sprite at its pixel position minus camera offset
}
```

### Create a test room

In game/src/main.rs or a new game/src/room.rs:

Create a 15×12 tile room (120×96 pixels — fits in the 80×100 pixel viewport with room to spare):

```
WWWWWWWWWWWWWWW
W.............W
W.............W
W.............W
W.............W
W......W......W
W......W......W
W.............W
W.............W
W.............W
W.............W
WWWWWWWWWWWWWWW
```

W = Wall (with WallTop on the row above floor), . = Floor

The player should spawn in the center of the room.

### Tile collision

When the player moves, check collision against solid tiles:
- Player has a collision box (smaller than sprite, e.g., 6×4 pixels at the feet)
- Before applying movement, check if the new position's collision box overlaps any solid tile
- If it does, don't move in that direction (slide along walls by checking X and Y separately)

### Update game/src/main.rs:
- Render the tile map
- Render the player on top
- Player walks around the room, collides with walls
- Camera is static for now (room fits in viewport)

`cargo run` should show a tiled room with the animated player walking around, blocked by walls.
```

### Success Criteria
- [ ] Tile map renders correctly with distinct floor/wall/door tiles
- [ ] Player walks over floor tiles with proper animation
- [ ] Player cannot walk through walls (collision works)
- [ ] Wall sliding works (holding diagonal into a wall doesn't get stuck)
- [ ] Tile rendering uses diff (only changed tiles re-render when nothing moves)

---

## Session 1.4 — Camera System

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Tile maps and collision work. Now add a camera that follows the player and supports rooms larger than the viewport.

### Create engine/src/camera.rs

```rust
pub struct Camera {
    pub x: f32,           // top-left corner in world pixels
    pub y: f32,
    pub target_x: f32,    // where we're lerping toward
    pub target_y: f32,
    pub viewport_w: usize, // framebuffer pixel width
    pub viewport_h: usize, // framebuffer pixel height
    pub smoothing: f32,    // 0.0 = instant snap, 0.95 = very smooth lag
    
    // Screen shake
    shake_intensity: f32,
    shake_decay: f32,      // how fast shake dies down (e.g., 0.9 = fast)
    shake_offset_x: f32,
    shake_offset_y: f32,
}

impl Camera {
    pub fn new(viewport_w: usize, viewport_h: usize) -> Self { ... }
    
    /// Set the target to follow (usually player center)
    pub fn follow(&mut self, world_x: f32, world_y: f32) {
        self.target_x = world_x - self.viewport_w as f32 / 2.0;
        self.target_y = world_y - self.viewport_h as f32 / 2.0;
    }
    
    /// Update camera position (call in fixed_update)
    pub fn update(&mut self, dt: f64) {
        // Smooth lerp toward target
        let t = 1.0 - self.smoothing.powf(dt as f32 * 30.0); // frame-rate independent smoothing
        self.x += (self.target_x - self.x) * t;
        self.y += (self.target_y - self.y) * t;
        
        // Update shake
        if self.shake_intensity > 0.1 {
            self.shake_offset_x = (rand() - 0.5) * self.shake_intensity;
            self.shake_offset_y = (rand() - 0.5) * self.shake_intensity;
            self.shake_intensity *= self.shake_decay;
        } else {
            self.shake_offset_x = 0.0;
            self.shake_offset_y = 0.0;
            self.shake_intensity = 0.0;
        }
    }
    
    /// Clamp camera to world bounds (don't show outside the map)
    pub fn clamp_to_bounds(&mut self, world_w: f32, world_h: f32) {
        self.x = self.x.clamp(0.0, (world_w - self.viewport_w as f32).max(0.0));
        self.y = self.y.clamp(0.0, (world_h - self.viewport_h as f32).max(0.0));
    }
    
    /// Get the final camera offset (position + shake)
    pub fn offset(&self) -> (i32, i32) {
        ((self.x + self.shake_offset_x) as i32, (self.y + self.shake_offset_y) as i32)
    }
    
    /// Trigger screen shake
    pub fn shake(&mut self, intensity: f32) {
        self.shake_intensity = intensity;
    }
    
    /// Convert world coordinates to screen coordinates
    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (i32, i32) {
        let (ox, oy) = self.offset();
        (wx as i32 - ox, wy as i32 - oy)
    }
}
```

For random shake offsets, use a simple LCG or add the `fastrand` crate (tiny, no-std compatible).

### Create a larger test room

Make a 30×25 tile room (240×200 pixels — much larger than viewport). Add interior walls, a few pillars, and open spaces to make camera movement interesting:

```
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
W............................W
W............................W
W....WWWW......WWWW..........W
W....W..........W............W
W....W..........W............W
W................W...........W
W............................W
W............................W
W........WW..................W
W........WW..................W
W............................W
W............................W
W...........WWWWWW...........W
W............................W
W............................W
W..WW....................WW..W
W..WW....................WW..W
W............................W
W............................W
W............................W
W............................W
W............................W
W............................W
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
```

### Integrate camera into rendering

Update the render pipeline:
1. Camera follows player center
2. Tile map renders with camera offset
3. Player sprite renders with camera offset  
4. Camera clamps to room bounds (no showing void beyond walls)

### Test screen shake

Press Attack to trigger a small screen shake (intensity 3.0, decay 0.85). Press Dash to trigger a bigger shake (intensity 6.0). The screen should jitter briefly then settle.

### Update game rendering order:
1. Clear framebuffer
2. Render tilemap (with camera offset — only visible tiles)
3. Render player sprite (with camera offset)
4. Render HUD on top (FPS, position — NOT affected by camera/shake)

`cargo run` should show the player exploring a large room with the camera smoothly following, clamped to bounds, with screen shake on input.
```

### Success Criteria
- [ ] Camera smoothly follows the player (visible lag, not instant snap)
- [ ] Camera clamps to room bounds (no void visible at edges)
- [ ] Screen shake works and decays naturally
- [ ] HUD elements don't shake with the camera
- [ ] Only visible tiles are rendered (performance stays good in large rooms)
- [ ] Player can explore the entire large room

---

## Session 1.5 — Visual Designer: Player Sprite Sheet

### Claude Code Prompt

```
You are the Visual Designer for Cryptfall. The engine supports sprites, animation, and tiling. Now create a proper player character sprite sheet in NES-inspired style.

The character is a warrior/adventurer for a Hades-style roguelike dungeon crawler. Think NES Castlevania or early Zelda — chunky, readable, high contrast.

### Constraints:
- Each frame is 10×14 pixels (wide enough for weapons, tall enough for detail)
- NES-style palette: use 4-5 colors max per sprite
- Must read clearly against dark dungeon backgrounds
- Must have a strong, distinct silhouette

### Color palette for the player:
```rust
const SKIN: Color = [240, 185, 140];    // warm skin tone
const HAIR: Color = [60, 45, 30];       // dark brown hair  
const ARMOR: Color = [80, 140, 200];    // steel blue armor
const ARMOR_HI: Color = [120, 180, 230]; // armor highlight
const ARMOR_LO: Color = [50, 100, 150];  // armor shadow
const BOOTS: Color = [100, 70, 45];      // leather brown
const WEAPON: Color = [200, 200, 210];   // silver weapon
const WEAPON_HI: Color = [240, 240, 255]; // weapon highlight
```

### Animations needed (all facing RIGHT — engine handles flip):

1. **IDLE** (2 frames, loop, 0.5s/frame):
   - Frame 0: Standing neutral, sword at side
   - Frame 1: Slight shift — sword arm moves slightly, body shifts 1px. Subtle "breathing"

2. **WALK** (4 frames, loop, 0.12s/frame):
   - Classic walk cycle with vertical bob
   - Frame 0: Contact — right foot forward, body high
   - Frame 1: Passing — feet together, body low (1px drop)
   - Frame 2: Contact — left foot forward, body high
   - Frame 3: Passing — feet together, body low

3. **DASH** (2 frames, one-shot, 0.07s/frame):
   - Frame 0: Crouched forward lean, speed lines implied by stretched pose
   - Frame 1: Extended, almost horizontal

4. **ATTACK** (4 frames, one-shot, 0.06s/frame):
   - Frame 0: Wind-up — weapon pulled back
   - Frame 1: Swing mid — weapon horizontal, body rotated
   - Frame 2: Swing follow-through — weapon extended forward (this is the active hitbox frame)
   - Frame 3: Recovery — returning to idle pose

5. **HIT** (2 frames, one-shot, 0.1s/frame):
   - Frame 0: Recoil — body tilts back
   - Frame 1: Recovery

6. **DEATH** (4 frames, one-shot, 0.15s/frame):
   - Frame 0: Stagger
   - Frame 1: Falling
   - Frame 2: On ground
   - Frame 3: Fade/flash (just the sprite in white — engine will handle actual fading)

### Output format:
Create game/src/sprites/player.rs with each frame as a static SpriteData const. Also create AnimationData consts for each animation set.

### Design principles:
- The weapon should extend BEYOND the body in attack frames (this is what creates the hitbox)
- Silhouette must be distinct at this small size — use bright armor against dark backgrounds
- The 1-pixel vertical bob in walk cycle is essential for perceived quality
- Feet should be distinct from legs (different color boots)
- Head should be ~3px wide, body ~5-6px wide

Generate the actual pixel arrays. Every pixel should be intentional.
```

### Success Criteria
- [ ] Character is recognizable as a humanoid warrior at 10×14 pixels
- [ ] All 6 animations have correct frame counts
- [ ] Walk cycle has visible vertical bob
- [ ] Attack frames show clear weapon extension
- [ ] Sprite reads clearly on a dark ([30,30,35]) background
- [ ] Horizontal flip looks natural (no asymmetry issues)

---

## Session 1.6 — Phase 1 Integration & Polish

### Claude Code Prompt

```
You are the QA Lead for Cryptfall. Review Phase 1: sprite engine, animation, tilemap, camera.

### Integration test:

Verify the complete scene works together:
1. Large tiled room (30×25 tiles) with interior walls
2. Player character with all animations:
   - Idle when standing still
   - Walk when moving (with correct facing)
   - Dash on Dash key (fast movement, dash sprite, i-frames indicated by slight transparency)
   - Attack on Attack key (attack animation plays, weapon extends)
3. Camera smoothly follows player, clamps to bounds
4. Screen shake on Attack press (small shake)
5. Tile collision prevents walking through walls
6. Wall sliding works on diagonals

### Visual quality check:
- Are the tile sprites visually cohesive? (consistent style/palette)
- Is the player sprite clearly visible against all tile types?
- Do animations feel right at 30 FPS? Any need for timing adjustments?
- Is the camera smoothing value comfortable? (not too laggy, not too snappy)

### Performance check:
- Profile the render loop with the tilemap + player + particles
- Check that diff rendering is effective (most frames should redraw <20% of cells)
- Verify no memory growth over time (animation system shouldn't allocate)

### Bug hunt:
- Move into corners rapidly — any collision issues?
- Dash through narrow gaps — does the collision box handle it?
- Resize terminal during gameplay — does everything adjust?
- Spam attack during movement — any animation state machine bugs?

### Create a "demo mode":
Add a simple attract mode: if no input for 5 seconds, the player auto-walks around the room, attacking occasionally. This serves as:
- A visual demo for README GIFs later
- A stress test for the animation/movement systems
- An idle attract screen for the eventual title menu

Write findings in docs/phase1-review.md.
```

### Success Criteria
- [ ] All animations play correctly in context
- [ ] No collision glitches in corners or narrow passages
- [ ] Performance is good (≥30 FPS with full scene)
- [ ] Demo mode runs without issues for 60+ seconds
- [ ] Phase 1 review document completed

---

## Phase 1 File Manifest

New/modified files:

```
crates/engine/src/
├── sprite.rs        # SpriteData, blitting with clip/flip/tint
├── animation.rs     # AnimationData, AnimationPlayer
├── tilemap.rs       # TileMap, tile collision, tile rendering
├── camera.rs        # Camera follow, smoothing, screen shake, bounds clamping
└── lib.rs           # Updated exports

crates/game/src/
├── main.rs          # Updated with full scene
├── player.rs        # Player struct, state machine, movement
├── sprites/
│   ├── mod.rs
│   ├── player.rs    # Player sprite frames + animation data
│   └── tiles.rs     # Tile sprite data
└── room.rs          # Room definition, test room layout

docs/
└── phase1-review.md
```
