#![allow(dead_code)]

use engine::animation::AnimationData;
use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;
const P: Option<Color> = Some([120, 60, 180]); // purple body
const L: Option<Color> = Some([160, 100, 220]); // light purple
const D: Option<Color> = Some([80, 30, 130]); // dark purple
const E: Option<Color> = Some([200, 100, 255]); // bright eyes/magic
const G: Option<Color> = Some([60, 200, 255]); // cyan glow
const W: Option<Color> = Some([255, 255, 255]); // white

// =============================================================================
// IDLE / FLOAT — 2 frames, 0.6s/frame, looping. Gentle bob.
// 10x12 sprite (slightly shorter than player/skeleton)
// =============================================================================

#[rustfmt::skip]
static GHOST_IDLE_0: SpriteData = SpriteData::new(10, 12, &[
    N, N, N, L, L, L, L, N, N, N,
    N, N, L, P, P, P, P, L, N, N,
    N, N, P, E, P, P, E, P, N, N,
    N, N, P, P, P, P, P, P, N, N,
    N, N, D, P, P, P, P, D, N, N,
    N, N, D, P, G, P, P, D, N, N,
    N, N, N, P, P, P, P, N, N, N,
    N, N, N, D, P, P, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, D, N, N, D, N, N, N,
    N, N, D, N, N, N, N, D, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static GHOST_IDLE_1: SpriteData = SpriteData::new(10, 12, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, L, L, L, L, N, N, N,
    N, N, L, P, P, P, P, L, N, N,
    N, N, P, E, P, P, E, P, N, N,
    N, N, P, P, P, P, P, P, N, N,
    N, N, D, P, P, P, P, D, N, N,
    N, N, D, P, G, P, P, D, N, N,
    N, N, N, P, P, P, P, N, N, N,
    N, N, N, D, P, P, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, D, N, N, N, N, D, N, N,
    N, N, N, D, N, N, D, N, N, N,
]);

pub static GHOST_IDLE_ANIM: AnimationData = AnimationData {
    frames: &[&GHOST_IDLE_0, &GHOST_IDLE_1],
    frame_duration: 0.6,
    looping: true,
};

// =============================================================================
// AIM — 2 frames, 0.3s/frame, looping while aiming. Magic glow intensifies.
// =============================================================================

#[rustfmt::skip]
static GHOST_AIM_0: SpriteData = SpriteData::new(10, 12, &[
    N, N, N, L, L, L, L, N, N, N,
    N, N, L, P, P, P, P, L, N, N,
    N, N, P, G, P, P, G, P, N, N,
    N, N, P, P, P, P, P, P, N, N,
    N, N, D, P, P, P, P, D, N, N,
    N, N, D, P, G, G, P, D, N, N,
    N, N, N, P, G, G, P, N, N, N,
    N, N, N, D, P, P, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, D, N, N, D, N, N, N,
    N, N, D, N, N, N, N, D, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static GHOST_AIM_1: SpriteData = SpriteData::new(10, 12, &[
    N, N, N, L, L, L, L, N, N, N,
    N, N, L, P, P, P, P, L, N, N,
    N, N, P, E, P, P, E, P, N, N,
    N, N, P, P, P, P, P, P, N, N,
    N, N, D, P, P, P, P, D, N, N,
    N, N, D, G, G, G, G, D, N, N,
    N, N, N, P, G, G, P, N, N, N,
    N, N, N, D, P, P, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, D, N, N, D, N, N, N,
    N, N, D, N, N, N, N, D, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static GHOST_AIM_ANIM: AnimationData = AnimationData {
    frames: &[&GHOST_AIM_0, &GHOST_AIM_1],
    frame_duration: 0.3,
    looping: true,
};

// =============================================================================
// STAGGER — 1 frame, one-shot
// =============================================================================

pub static GHOST_STAGGER_ANIM: AnimationData = AnimationData {
    frames: &[&GHOST_IDLE_1],
    frame_duration: 0.3,
    looping: false,
};

// =============================================================================
// DEATH — 3 frames, 0.15s/frame, one-shot
// =============================================================================

#[rustfmt::skip]
static GHOST_DEATH_0: SpriteData = SpriteData::new(10, 12, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, L, L, L, L, N, N, N,
    N, N, L, P, P, P, P, L, N, N,
    N, N, P, P, P, P, P, P, N, N,
    N, N, D, P, P, P, P, D, N, N,
    N, N, N, D, P, P, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static GHOST_DEATH_1: SpriteData = SpriteData::new(10, 12, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, L, N, L, P, N, L, N, N,
    N, N, N, P, N, N, D, N, N, N,
    N, N, N, N, D, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static GHOST_DEATH_ANIM: AnimationData = AnimationData {
    frames: &[&GHOST_DEATH_0, &GHOST_DEATH_1],
    frame_duration: 0.2,
    looping: false,
};
