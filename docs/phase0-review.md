# Phase 0 Review — Engine Foundation

## What Works Well

- **Terminal safety**: Panic hook + Drop ensures terminal is always restored on panic and normal exit. All cleanup operations use `let _ =` to ignore errors during teardown.
- **Half-block rendering**: Clean implementation of ▄ character with true RGB colors via ANSI escape codes. Synchronized output (`CSI ?2026h/l`) prevents flicker.
- **Diff rendering**: Front/back buffer comparison minimizes escape code output. Cursor positioning batches consecutive changed cells. Typical partial redraws emit only a few KB instead of the full ~120KB for 80×50.
- **Fixed-timestep loop**: Accumulator pattern with MAX_FRAME_TIME cap prevents spiral of death. Interpolation factor (alpha) enables smooth rendering between ticks.
- **Input system**: Held-key inference via 150ms timeout works well for terminal environments that lack key-up events. Diagonal normalization (×1/√2) gives correct movement speed.
- **Zero-allocation hot paths**: Renderer reuses String and Vec buffers. Input begin_frame uses a stack-allocated array instead of Vec for timeout collection. Events Vec is pre-allocated and reused.

## Issues Fixed in This Review

1. **input.rs:begin_frame() heap allocation**: Was allocating a `Vec<GameKey>` every frame to collect timed-out keys. Replaced with a fixed-size `[Option<GameKey>; 8]` stack buffer — zero allocations in the hot path.
2. **Trail container**: `Vec::remove(0)` is O(n). Replaced with `VecDeque` for O(1) front removal.
3. **Missing timing instrumentation**: Added input_us, update_us, render_us to FrameInfo so the game can display per-phase timing breakdowns.

## Known Limitations

- **Ctrl+C / SIGINT**: In raw mode, Ctrl+C generates a key event (handled normally). External SIGINT (kill -2) terminates without running destructors — terminal may not be restored. Acceptable for development; SSH server (Phase 5) will handle this with proper signal handlers.
- **FPS slightly below 30**: Sleep margin (1ms) and OS scheduling jitter mean actual FPS is typically 27–29. This is acceptable — the fixed-timestep loop decouples physics from render rate.
- **No text rendering**: HUD uses colored pixel bars instead of text. A bitmap font system (Phase 1+) will enable proper text display.
- **Full-screen gradient is expensive**: Redrawing every pixel each frame (gradient background) forces a full-screen diff. In actual gameplay, static backgrounds will be tilemap-based and mostly unchanged between frames.

## Performance Observations

- **Partial redraw** (moving 6×6 square on static background): ~50–100 cells redrawn out of ~4000 total. Minimal output bandwidth.
- **Full redraw** (first frame or resize): Entire buffer is written. At 80×50 this is ~120KB of escape codes, well within terminal throughput for 30 FPS.
- **Input polling**: < 50μs per frame (essentially free).
- **Update tick**: < 10μs per frame (trivial game logic).
- **Render (buffer build + write)**: Typically 200–2000μs depending on how many cells changed.

## Terminal Compatibility

- **Synchronized output**: Uses `CSI ?2026h/l` which is widely supported (iTerm2, kitty, WezTerm, Windows Terminal, ghostty). Unsupported terminals silently ignore it.
- **True color**: Uses `CSI 38;2;r;g;b m` / `CSI 48;2;r;g;b m`. Requires a terminal with 24-bit color support. Most modern terminals support this; legacy 256-color terminals will display incorrect colors.
- **Half-block character**: ▄ (U+2584) requires Unicode support. All modern terminals handle this.

## File Manifest

```
crates/engine/src/
├── lib.rs          — Module exports, Terminal struct, cleanup, panic hook
├── color.rs        — Color type alias, named constants
├── framebuffer.rs  — FrameBuffer pixel grid (2× vertical resolution)
├── renderer.rs     — Diff renderer with sync output, front/back buffers
├── input.rs        — InputState with held-key inference (150ms timeout)
├── gameloop.rs     — Fixed-timestep loop (30Hz) with timing instrumentation
└── types.rs        — Vec2, Transform with interpolation

crates/game/src/
└── main.rs         — Test scene: gradient, moving square, trail, HUD
```

## Ready for Phase 1

The engine foundation is solid. Phase 1 (sprite engine) can build on:
- FrameBuffer's set_pixel/fill_rect for sprite blitting
- Transform/Vec2 for sprite positioning and interpolation
- InputState for player control
- Game trait for clean update/render separation
- Diff renderer for efficient partial screen updates
