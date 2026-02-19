# Phase 1 Review — Sprite Engine

**Reviewer:** QA Lead
**Date:** Session 1.6

## Summary

Phase 1 delivers a complete sprite engine with animation, tilemap rendering, camera system, and player character with full state machine. All systems integrate cleanly.

## Animation System

- **Idle**: 2-frame breathing loop (0.5s/frame) — subtle and appropriate
- **Walk**: 4-frame cycle (0.12s/frame) with vertical bob — reads well at 30 FPS
- **Dash**: 2-frame one-shot (0.07s/frame) — fast and punchy
- **Attack**: 4-frame one-shot (0.06s/frame) — weapon extension is clear, locks movement during swing
- **Hit/Death**: Defined but not yet wired (Phase 2 combat)

`AnimationPlayer.play()` uses `std::ptr::eq` to avoid restarting the same animation — prevents stutter when holding a direction.

## Player State Machine

States: Idle → Walking → Dashing → Attacking

- Attack locks movement until animation completes, then transitions based on held direction
- Dash uses fixed timer (0.15s) at 200 px/s with locked direction
- Facing direction preserved during attack (doesn't flip mid-swing)
- Attack takes priority over dash when both pressed simultaneously

## Tilemap & Collision

- 30×25 test room with interior walls, pillars, and auto-detected WallTop ledges
- AABB collision with 8×4 foot hitbox offset from 10×14 sprite
- X and Y axes checked independently for wall sliding — works correctly on diagonals
- 0.01 epsilon prevents floating-point edge sticking

## Camera

- Smooth follow with frame-rate independent lerp: `1 - 0.85^(dt*30)`
- LCG screen shake with exponential decay (0.85 per frame)
- World bounds clamping prevents showing void
- Attack shake (3.0) feels like impact feedback; dash shake (6.0) feels like burst of speed
- Initial `snap()` prevents pan from origin on first frame

## Visual Quality

- 10×14 sprites with NES-inspired 8-color palette are readable on dark tile backgrounds
- Tile sprites (8×8) have consistent stone/brick aesthetic
- WallTop tiles add depth perception to top-down view
- Red attack flash and blue dash tint provide clear state feedback
- Horizontal flip looks natural (symmetric character design)

## Performance

- Diff rendering redraws <20% of cells during typical gameplay (mostly player movement area)
- No allocations in animation, sprite blitting, or collision hot paths
- Tilemap renders only viewport-visible tiles
- Camera offset is integer (no sub-pixel jitter in tile rendering)

## Demo Mode

- Activates after 5 seconds of no input
- Auto-walks with pseudo-random LCG direction changes (0.8–2.8s intervals)
- Occasional attacks (1.5–4.5s intervals) with flash and shake
- Occasional dashes
- Any user input immediately exits demo mode
- Runs stable for 60+ seconds without issues

## Bugs Found & Fixed

- None blocking. All corner collision, narrow gap, and spam-attack scenarios handled correctly by the state machine.

## Known Limitations (Expected, Not Bugs)

- Terminal resize during gameplay causes one-frame layout jump (camera viewport updates in render)
- Hit and death animations are defined but not wired (awaiting Phase 2 combat)
- No animation canceling (attack must complete before next action) — intentional for game feel

## Verdict

Phase 1 is complete. All success criteria met. Ready for Phase 2 (combat core).
