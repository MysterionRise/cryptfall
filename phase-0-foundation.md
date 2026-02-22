# Phase 0: Foundation — Engine Bootstrap

## Overview
**Duration:** Weeks 1–3 (6–9 sessions at ~3hrs each)
**Goal:** A colored rectangle moving smoothly around the screen at 30 FPS with zero flicker.
**Role:** Engine Architect (all sessions)

---

## Session 0.1 — Project Scaffold & Raw Terminal

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall, a terminal game engine built on crossterm in Rust. This is session 1: project scaffolding.

Create a cargo workspace with this structure:

cryptfall/
├── Cargo.toml          (workspace)
├── crates/
│   ├── engine/
│   │   ├── Cargo.toml  (lib crate, depends on crossterm)
│   │   └── src/
│   │       └── lib.rs
│   └── game/
│       ├── Cargo.toml  (bin crate, depends on engine)
│       └── src/
│           └── main.rs

Engine dependencies: crossterm = "0.28"
Game dependencies: engine = { path = "../engine" }

In engine/src/lib.rs, implement:

1. A `Terminal` struct that:
   - On `new()`: enters raw mode, switches to alternate screen, hides cursor, enables mouse capture
   - On `drop()`: restores everything (raw mode off, main screen, show cursor, disable mouse)
   - Has a `size() -> (u16, u16)` method returning terminal columns and rows

2. A `run` function that takes a closure `FnMut() -> bool` and calls it in a loop until it returns false, with a basic 30 FPS frame timer using std::time::Instant. Print the actual FPS in the top-right corner of the terminal each frame.

In game/src/main.rs:
- Create the Terminal
- Use the run loop
- Just clear the screen and print "Cryptfall Engine - {fps} FPS" at position (0,0) each frame
- Exit on 'q' keypress (use crossterm::event::poll with zero duration + read)

Make sure the terminal is ALWAYS restored even on panic — use std::panic::set_hook to call terminal cleanup before the default panic handler.

Test it: `cargo run` should show the FPS counter and quit cleanly on 'q'.
```

### Success Criteria
- [ ] `cargo run` enters alternate screen, shows FPS, exits cleanly on 'q'
- [ ] Ctrl+C doesn't leave terminal in broken state
- [ ] Panic doesn't leave terminal in broken state
- [ ] FPS counter reads ~30

### Key Technical Notes
- crossterm 0.28 is the latest stable as of early 2026. If it fails, try 0.27.
- The panic hook is CRITICAL — without it, any crash leaves the user's terminal destroyed
- Use `crossterm::event::poll(Duration::ZERO)` for non-blocking input, NOT `read()` which blocks

---

## Session 0.2 — FrameBuffer & Half-Block Rendering

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. We have a working terminal loop from session 0.1.

Now implement the core rendering system in engine/src/. Create these files:

### engine/src/color.rs
- Define `type Color = [u8; 3]` (RGB)
- Define some named constants: BLACK, WHITE, RED, GREEN, BLUE, TRANSPARENT (use Option<Color> where None = transparent)

### engine/src/framebuffer.rs
Implement `FrameBuffer`:

```rust
pub struct FrameBuffer {
    width: usize,       // pixel columns (= terminal columns)
    height: usize,      // pixel rows (= terminal rows × 2, because half-block)
    pixels: Vec<Option<Color>>,  // row-major, None = background/transparent
    background: Color,  // default background color (dark gray or black)
}
```

Key methods:
- `new(term_cols: usize, term_rows: usize)` — allocates width=term_cols, height=term_rows*2
- `clear()` — fill all pixels with None
- `set_pixel(x: usize, y: usize, color: Color)` — bounds-checked
- `get_pixel(x: usize, y: usize) -> Option<Color>`
- `fill_rect(x: usize, y: usize, w: usize, h: usize, color: Color)` — fill a rectangle

### engine/src/renderer.rs
Implement `Renderer`:

The renderer converts a FrameBuffer into terminal output using the half-block technique:
- Each terminal cell at (col, row) represents TWO vertical pixels:
  - Top pixel: framebuffer[col, row*2]
  - Bottom pixel: framebuffer[col, row*2 + 1]
- Render using '▄' (U+2584 Lower Half Block):
  - Background color of the cell = top pixel color (or background)
  - Foreground color of the cell = bottom pixel color (or background)

The renderer should:
1. Build a String buffer (pre-allocate ~128KB)
2. Start with CSI sequence to move cursor to (0,0)
3. Wrap entire output in BeginSynchronizedUpdate / EndSynchronizedUpdate
4. For each terminal row, for each column:
   - Determine top_color and bottom_color
   - Emit the minimum escape sequences (only change fg/bg when they differ from previous cell)
   - Emit '▄'
5. Write the entire buffer to stdout in ONE write call
6. Flush stdout

Important optimizations:
- Track "current_fg" and "current_bg" to avoid redundant color changes
- Use `\x1b[48;2;{r};{g};{b}m` for background, `\x1b[38;2;{r};{g};{b}m` for foreground
- Reset colors at the end with `\x1b[0m`

### Update engine/src/lib.rs
- Export the new modules
- Update the Terminal struct to hold a FrameBuffer and Renderer
- Provide a `frame(&mut self) -> &mut FrameBuffer` method to access the buffer
- After each frame's update closure, call the renderer

### Update game/src/main.rs
- Draw a test pattern: fill the screen with a gradient (vary red channel across X, green channel across Y)
- This proves true color half-block rendering works

`cargo run` should fill the terminal with a smooth color gradient. No flicker.
```

### Success Criteria
- [ ] Terminal shows a smooth RGB gradient using half-block characters
- [ ] No visible flicker (synchronized output working)
- [ ] Resizing terminal doesn't crash (handle gracefully)
- [ ] Colors look correct (test on at least 2 terminals if possible)

### Key Technical Notes
- The ▄ character is `\u{2584}` in Rust
- Pre-allocate the String buffer: `String::with_capacity(width * height * 30)` roughly
- `BeginSynchronizedUpdate` is `\x1b[?2026h`, End is `\x1b[?2026l`
- Some terminals don't support synchronized output — it's safe to always send it (unsupported terminals just ignore it)
- Use `std::io::Write` trait on stdout, NOT `println!`

---

## Session 0.3 — Double Buffering & Diff Rendering

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. We have half-block rendering working. Now add differential rendering to minimize output bandwidth.

### Modify engine/src/renderer.rs

Add a second buffer (the "front buffer") that stores the PREVIOUS frame's cell state. The cell state is:

```rust
#[derive(Clone, Copy, PartialEq)]
struct Cell {
    top: Color,     // resolved color (None becomes background color)
    bottom: Color,
}
```

The Renderer now holds:
- `front: Vec<Cell>` — what's currently displayed
- `back: Vec<Cell>` — what we're about to display
- `width: usize`, `height: usize` (in terminal cells, not pixels)

New rendering flow:
1. Convert the FrameBuffer pixels into the `back` buffer (resolve None → background color)
2. Compare `back` vs `front` cell by cell
3. Only emit escape sequences for cells that CHANGED
4. For changed cells, use cursor positioning (`\x1b[{row};{col}H`) to jump to them
5. Batch consecutive changed cells in the same row (avoid repositioning for each one)
6. After rendering, swap: front = back.clone() (or use std::mem::swap with a fresh back)

Add a `force_redraw` flag that skips the diff and renders everything (needed after terminal resize).

### Add terminal resize handling

In the game loop, check for `crossterm::event::Event::Resize(cols, rows)` events. When detected:
- Resize both buffers
- Set force_redraw = true
- Update the FrameBuffer dimensions

### Update game/src/main.rs to test:
- Draw the color gradient as before
- Also draw a small 6×6 white square at position (10, 10)
- Move it with arrow keys (1 pixel per keypress)
- Print FPS and "Cells redrawn: {n}/{total}" in top-right to prove diff rendering works

When the square moves, only ~50-100 cells should be redrawn, not the entire screen.
```

### Success Criteria
- [ ] Moving the square redraws only changed cells (visible in the counter)
- [ ] Full screen gradient + moving square, no flicker
- [ ] Terminal resize is handled (no crash, content redraws correctly)
- [ ] FPS remains stable at 30

### Key Technical Notes
- The diff is the single most important optimization. A full 80×50 redraw is ~120KB of escape codes. A typical frame with a moving character might change 200 cells = ~6KB
- Cursor positioning: `\x1b[{row+1};{col+1}H` (1-indexed!)
- Consecutive changed cells in the same row: position to the first one, then emit the rest sequentially (no repositioning needed)
- Be careful with the buffer swap — don't allocate new vecs each frame

---

## Session 0.4 — Input System

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. We need a proper input system for real-time games.

### Create engine/src/input.rs

The terminal only gives us key-press events (not key-down/key-up). For real-time games we need to infer "held" state. Design:

```rust
pub enum GameKey {
    Up, Down, Left, Right,
    Attack,     // Z or Enter
    Dash,       // X or Space
    Pause,      // Escape
    Quit,       // Q
}

pub struct InputState {
    // For each GameKey, track:
    pressed: HashSet<GameKey>,      // newly pressed THIS frame
    held: HashMap<GameKey, Instant>, // last time this key was seen
    released: HashSet<GameKey>,      // keys that timed out (inferred release)
}
```

The input system works as follows:
1. At the START of each frame, call `input.begin_frame()`
   - Clear `pressed` and `released` sets
   - Check all `held` keys: if a key hasn't been re-seen in >150ms, move it to `released` and remove from `held`
2. During input polling, for each keypress event:
   - Map raw crossterm KeyCode → GameKey
   - If key is NOT in `held`: add to `pressed` AND `held` (it's newly pressed)
   - If key IS in `held`: update the timestamp (it's being held/repeated)
3. Provide query methods:
   - `is_pressed(key)` → true only on the FIRST frame of a press
   - `is_held(key)` → true while the key is being held
   - `is_released(key)` → true on the frame the key times out
   - `direction() -> (f32, f32)` → returns normalized (dx, dy) from arrow key state. (-1,0) for left, (1,1) normalized for diagonal, (0,0) for nothing.

Also poll ALL available events each frame (drain the event queue), not just one.

### Map these crossterm keys:
- Arrow keys → Up/Down/Left/Right
- WASD → Up/Down/Left/Right (alternative)
- Z, Enter → Attack
- X, Space → Dash  
- Escape → Pause
- Q → Quit

### Kitty keyboard protocol (optional enhancement):
If `crossterm::event::PushKeyboardEnhancementFlags` is available, try enabling it. This gives actual key-release events. If it works, use real release events instead of the 150ms timeout. If the terminal doesn't support it, fall back gracefully.

### Update game/src/main.rs:
- Replace raw key handling with the new InputState
- Move the square smoothly: if direction is held, move 2 pixels per frame (60px/sec at 30fps)
- Show input debug info: which keys are pressed/held/released
- Dash test: pressing Dash makes the square jump 20 pixels in the current direction

Test that movement feels responsive. Adjust the held-timeout if needed.
```

### Success Criteria
- [ ] Square moves smoothly while holding arrow keys
- [ ] Dash jumps the square in the movement direction
- [ ] Movement stops shortly after releasing the key (~150ms delay, not noticeable)
- [ ] Diagonal movement works and is normalized (same speed as cardinal)
- [ ] Input debug display shows correct pressed/held/released states
- [ ] Kitty protocol enhancement attempted (okay if it doesn't work on current terminal)

### Key Technical Notes
- Drain ALL events per frame: `while crossterm::event::poll(Duration::ZERO)? { let event = crossterm::event::read()?; ... }`
- The 150ms release timeout is a tuning value. Too short = keys "drop" during OS key-repeat gap. Too long = movement feels sluggish on release. Start at 150ms, tune based on feel.
- Key repeat delay varies by OS (typically 250–500ms initial delay, then 30ms repeat). The 150ms timeout must be longer than the repeat interval but short enough to feel responsive.
- Normalize diagonal: if both X and Y are nonzero, multiply each by 0.707 (1/√2)

---

## Session 0.5 — Game Loop Refinement & Frame Timing

### Claude Code Prompt

```
You are the Engine Architect for Cryptfall. Refine the game loop to use fixed-timestep with interpolation.

### Modify engine/src/gameloop.rs (extract from lib.rs if needed)

Current loop is simple frame-rate limiting. Replace with a proper fixed-timestep loop:

```rust
const TICK_RATE: f64 = 1.0 / 30.0;  // 30 ticks per second
const MAX_FRAME_TIME: f64 = 0.25;    // prevent spiral of death

pub struct GameLoop {
    accumulator: f64,
    current_time: Instant,
    tick_count: u64,
    frame_count: u64,
    fps: f64,
    fps_timer: Instant,
    fps_frame_count: u64,
}
```

The loop logic:
```
loop {
    new_time = Instant::now()
    frame_time = min(new_time - current_time, MAX_FRAME_TIME)
    current_time = new_time
    accumulator += frame_time

    poll_input()  // drain all input events

    while accumulator >= TICK_RATE {
        fixed_update(TICK_RATE)   // physics/game logic at fixed rate
        accumulator -= TICK_RATE
        tick_count += 1
    }

    alpha = accumulator / TICK_RATE   // 0.0 to 1.0, interpolation factor
    render(alpha)                      // pass alpha for smooth rendering

    // FPS calculation (average over 1 second)
    fps_frame_count += 1
    if fps_timer.elapsed() >= 1 second {
        fps = fps_frame_count as f64 / fps_timer.elapsed().as_secs_f64()
        fps_frame_count = 0
        fps_timer = Instant::now()
    }
    
    // Sleep for remaining frame budget
    sleep_until(current_time + TICK_RATE - small_margin)
}
```

### Why this matters for the game:
- `fixed_update` runs at EXACTLY 30hz regardless of render speed. All physics, movement, cooldowns use fixed delta time.
- `render(alpha)` can interpolate between previous and current position for smoother visuals: `render_x = prev_x + (curr_x - prev_x) * alpha`
- If the system is slow, multiple ticks run before one render (catches up)
- MAX_FRAME_TIME prevents the "spiral of death" if the game freezes briefly

### Create engine/src/types.rs
Define position/velocity types that support interpolation:

```rust
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub struct Transform {
    pub position: Vec2,
    pub prev_position: Vec2,  // position at start of this tick
}

impl Transform {
    pub fn interpolated(&self, alpha: f32) -> Vec2 {
        Vec2 {
            x: self.prev_position.x + (self.position.x - self.prev_position.x) * alpha,
            y: self.prev_position.y + (self.position.y - self.prev_position.y) * alpha,
        }
    }
    
    pub fn commit(&mut self) {
        self.prev_position = self.position;
    }
}
```

Call `commit()` at the start of each fixed_update. Use `interpolated(alpha)` during render.

### Refactor the game loop API

The engine should provide a trait or struct that the game implements:

```rust
pub trait Game {
    fn update(&mut self, input: &InputState, dt: f64);  // fixed timestep
    fn render(&mut self, fb: &mut FrameBuffer, alpha: f32);  // interpolated
}
```

The engine's run function takes a `impl Game` and drives the loop.

### Update game/src/main.rs:
- Implement the Game trait
- Move the square using Transform (velocity-based: direction × speed × dt)
- Render using interpolated position
- Movement should look noticeably smoother than before, especially during direction changes

Also add a simple "trail" effect: when the square moves, leave fading copies behind (reduce alpha by 30% each frame for last 5 positions). This tests both the interpolation and visual feedback.
```

### Success Criteria
- [ ] Movement is visibly smoother than the previous frame-locked version
- [ ] FPS counter is stable and accurate
- [ ] Trail effect works (fading squares behind the moving one)
- [ ] Game doesn't slow down or speed up when system is under load
- [ ] Clean API: game code only implements `update()` and `render()`

---

## Session 0.6 — Integration Test & Phase 0 Checkpoint

### Claude Code Prompt

```
You are the QA Lead for Cryptfall. Review and test the Phase 0 engine.

### Review checklist:

1. **Code quality:** Read through all files in crates/engine/src/. Check for:
   - Unnecessary allocations in the hot path (render loop, input polling)
   - Proper error handling (no unwrap() in library code — use Result)
   - Consistent naming conventions
   - Missing pub visibility that game crate needs
   - Dead code or unused imports

2. **Terminal safety:** Verify:
   - Alternate screen is ALWAYS restored (normal exit, panic, Ctrl+C)
   - Test by adding `panic!("test")` in the game loop — terminal should restore
   - Test by pressing Ctrl+C — terminal should restore
   - Raw mode is properly exited in all code paths

3. **Rendering correctness:** 
   - Test the gradient: are colors smooth? Any banding or artifacts?
   - Test at different terminal sizes (resize the window during gameplay)
   - Check: does the diff renderer correctly handle the case where the terminal is resized? (force_redraw should trigger)
   - Check: are there any off-by-one errors at the right/bottom edge?

4. **Performance:**
   - Add a simple benchmark: render a fully-changing screen (every pixel different each frame). What's the actual FPS? Should be ≥30 on a modern terminal.
   - Profile: where does time go? Use `std::time::Instant` to measure input polling, update, and render separately. Print these timings.
   - Check for any allocations in the render loop (look for Vec::new, String::new, push, format! in hot paths)

5. **Create a test scene** that exercises everything:
   - Color gradient background
   - A white square moving with arrow keys
   - A trail effect behind the square
   - FPS counter in top-right
   - Frame timing breakdown (input/update/render ms) in top-left
   - Cells-redrawn counter
   - "Press Q to quit" text at bottom

6. **Write a brief report** of findings in docs/phase0-review.md:
   - What works well
   - What needs fixing before Phase 1
   - Performance numbers
   - Terminal compatibility notes

Fix any critical issues found. Phase 0 is complete when this review passes.
```

### Success Criteria
- [ ] All terminal safety tests pass (panic, Ctrl+C, normal exit)
- [ ] No allocations in the hot render path
- [ ] Full-screen redraw achieves ≥30 FPS
- [ ] Partial redraw (small moving object) achieves ≥30 FPS easily
- [ ] Code compiles with no warnings
- [ ] Phase 0 review document written

---

## Phase 0 File Manifest

After all sessions, you should have:

```
cryptfall/
├── Cargo.toml
├── crates/
│   ├── engine/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Module exports, Terminal struct
│   │       ├── color.rs        # Color type, constants
│   │       ├── framebuffer.rs  # FrameBuffer (pixel grid)
│   │       ├── renderer.rs     # Diff renderer with sync output
│   │       ├── input.rs        # InputState with held-key inference
│   │       ├── gameloop.rs     # Fixed-timestep loop
│   │       └── types.rs        # Vec2, Transform with interpolation
│   └── game/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs         # Test scene with moving square
└── docs/
    └── phase0-review.md
```
