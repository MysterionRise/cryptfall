#![allow(dead_code)]

use engine::animation::AnimationData;
use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None; // transparent
const S: Option<Color> = Some([240, 185, 140]); // skin
const H: Option<Color> = Some([60, 45, 30]); // dark brown hair
const A: Option<Color> = Some([80, 140, 200]); // steel blue armor
const L: Option<Color> = Some([120, 180, 230]); // armor highlight
const D: Option<Color> = Some([50, 100, 150]); // armor shadow
const B: Option<Color> = Some([100, 70, 45]); // leather boots
const W: Option<Color> = Some([200, 200, 210]); // silver weapon
const X: Option<Color> = Some([240, 240, 255]); // weapon highlight

// =============================================================================
// IDLE — 2 frames, 0.5s/frame, looping
// Subtle breathing: frame 1 shifts body 1px up.
// =============================================================================

#[rustfmt::skip]
static IDLE_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, W, N,
    N, N, N, A, A, A, A, N, W, N,
    N, N, N, D, A, A, D, N, W, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

#[rustfmt::skip]
static IDLE_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, W, N,
    N, N, N, D, A, A, D, N, W, N,
    N, N, N, N, A, A, N, N, W, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static IDLE_ANIM: AnimationData = AnimationData {
    frames: &[&IDLE_0, &IDLE_1],
    frame_duration: 0.5,
    looping: true,
};

// =============================================================================
// WALK — 4 frames, 0.12s/frame, looping
// Contact (high) → Passing (low) → Contact (high) → Passing (low)
// 1px vertical bob on passing frames.
// =============================================================================

/// Walk frame 0: right stride, body high.
#[rustfmt::skip]
static WALK_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, A, N, N, N, A, N, N,
    N, N, N, A, N, N, N, A, N, N,
    N, N, N, B, N, N, N, B, N, N,
    N, N, B, B, N, N, N, B, B, N,
]);

/// Walk frame 1: passing, body 1px low, feet together.
#[rustfmt::skip]
static WALK_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, B, B, N, N, N, N,
    N, N, N, B, B, B, B, N, N, N,
]);

/// Walk frame 2: left stride, body high.
#[rustfmt::skip]
static WALK_2: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, A, N, N, N, A, N, N, N,
    N, N, A, N, N, N, A, N, N, N,
    N, N, B, N, N, N, B, N, N, N,
    N, B, B, N, N, N, B, B, N, N,
]);

pub static WALK_ANIM: AnimationData = AnimationData {
    frames: &[&WALK_0, &WALK_1, &WALK_2, &WALK_1],
    frame_duration: 0.12,
    looping: true,
};

// =============================================================================
// DASH — 2 frames, 0.07s/frame, one-shot
// Crouched forward lean → extended horizontal.
// =============================================================================

/// Dash frame 0: crouched forward lean.
#[rustfmt::skip]
static DASH_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, A, N, N, A, N, N, N,
    N, N, N, B, N, N, B, N, N, N,
    N, N, B, B, N, N, B, B, N, N,
]);

/// Dash frame 1: extended, stretched forward.
#[rustfmt::skip]
static DASH_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, H, H, H, H, N, N,
    N, N, N, H, H, H, H, H, H, N,
    N, N, N, H, S, S, S, S, H, N,
    N, N, N, N, S, S, S, S, N, N,
    N, N, N, A, A, A, A, A, A, N,
    N, N, N, D, A, L, A, A, D, N,
    N, N, N, D, A, A, A, A, D, N,
    N, N, N, N, A, A, A, N, N, N,
    N, N, N, A, N, N, N, A, N, N,
    N, N, N, B, N, N, N, B, N, N,
    N, N, B, B, N, N, N, B, B, N,
]);

pub static DASH_ANIM: AnimationData = AnimationData {
    frames: &[&DASH_0, &DASH_1],
    frame_duration: 0.07,
    looping: false,
};

// =============================================================================
// ATTACK — 4 frames, 0.06s/frame, one-shot
// Wind-up → Swing mid → Follow-through (HITBOX) → Recovery
// =============================================================================

/// Attack frame 0: wind-up, weapon pulled up/back.
#[rustfmt::skip]
static ATTACK_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, W, N, N,
    N, N, H, H, H, H, H, H, W, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

/// Attack frame 1: swing mid, weapon horizontal at shoulder.
#[rustfmt::skip]
static ATTACK_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, W, X,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

/// Attack frame 2: follow-through, weapon fully extended (ACTIVE HITBOX).
#[rustfmt::skip]
static ATTACK_2: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, W, X, X,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

/// Attack frame 3: recovery, weapon returning to side.
#[rustfmt::skip]
static ATTACK_3: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, W, N, N,
    N, N, N, N, A, A, N, W, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

pub static ATTACK_ANIM: AnimationData = AnimationData {
    frames: &[&ATTACK_0, &ATTACK_1, &ATTACK_2, &ATTACK_3],
    frame_duration: 0.06,
    looping: false,
};

// =============================================================================
// HIT — 2 frames, 0.1s/frame, one-shot
// Recoil → Recovery
// =============================================================================

/// Hit frame 0: recoil, body tilts back (shifted left).
#[rustfmt::skip]
static HIT_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, H, H, H, H, N, N, N, N,
    N, H, H, H, H, H, H, N, N, N,
    N, H, S, S, S, S, H, N, N, N,
    N, N, S, S, S, S, N, N, N, N,
    N, A, A, A, A, A, A, N, N, N,
    N, D, A, L, A, A, D, N, N, N,
    N, D, A, A, A, A, D, N, N, N,
    N, N, A, A, A, A, N, N, N, N,
    N, N, D, A, A, D, N, N, N, N,
    N, N, N, A, A, N, N, N, N, N,
    N, N, N, A, N, A, N, N, N, N,
    N, N, N, A, N, A, N, N, N, N,
    N, N, N, B, N, B, N, N, N, N,
    N, N, B, B, N, B, B, N, N, N,
]);

/// Hit frame 1: recovery, returning to neutral.
#[rustfmt::skip]
static HIT_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, H, H, H, H, N, N, N,
    N, N, H, H, H, H, H, H, N, N,
    N, N, H, S, S, S, S, H, N, N,
    N, N, N, S, S, S, S, N, N, N,
    N, N, A, A, A, A, A, A, N, N,
    N, N, D, A, L, A, A, D, N, N,
    N, N, D, A, A, A, A, D, N, N,
    N, N, N, A, A, A, A, N, N, N,
    N, N, N, D, A, A, D, N, N, N,
    N, N, N, N, A, A, N, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

pub static HIT_ANIM: AnimationData = AnimationData {
    frames: &[&HIT_0, &HIT_1],
    frame_duration: 0.1,
    looping: false,
};

// =============================================================================
// DEATH — 4 frames, 0.15s/frame, one-shot
// Stagger → Falling → On ground → White flash
// =============================================================================

/// Death frame 0: stagger, leaning to the side.
#[rustfmt::skip]
static DEATH_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, H, H, H, H, N, N,
    N, N, N, H, H, H, H, H, H, N,
    N, N, N, H, S, S, S, S, H, N,
    N, N, N, N, S, S, S, S, N, N,
    N, N, N, A, A, A, A, A, N, N,
    N, N, N, D, A, L, A, A, N, N,
    N, N, N, D, A, A, A, A, N, N,
    N, N, N, N, A, A, A, N, N, N,
    N, N, N, N, D, A, A, N, N, N,
    N, N, N, N, N, A, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

/// Death frame 1: falling, body tilted further.
#[rustfmt::skip]
static DEATH_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, H, H, H, H, N,
    N, N, N, N, H, H, H, H, H, N,
    N, N, N, N, H, S, S, S, H, N,
    N, N, N, N, N, S, S, S, N, N,
    N, N, N, N, A, A, A, A, N, N,
    N, N, N, N, D, A, L, A, N, N,
    N, N, N, N, D, A, A, A, N, N,
    N, N, N, N, N, A, A, A, N, N,
    N, N, N, N, N, D, A, N, N, N,
    N, N, N, N, N, A, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, A, N, A, N, N, N,
    N, N, N, N, B, N, B, N, N, N,
    N, N, N, B, B, N, B, B, N, N,
]);

/// Death frame 2: collapsed on ground.
#[rustfmt::skip]
static DEATH_2: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, H, H, H, N, N, N, N,
    N, N, H, S, S, H, N, N, N, N,
    N, N, A, A, A, A, A, N, N, N,
    N, N, D, A, A, A, D, N, N, N,
    N, N, N, B, A, B, N, N, N, N,
    N, N, N, B, B, B, N, N, N, N,
]);

/// Death frame 3: white flash (same silhouette as collapsed, all white).
#[rustfmt::skip]
static DEATH_3: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, X, X, X, N, N, N, N,
    N, N, X, X, X, X, N, N, N, N,
    N, N, X, X, X, X, X, N, N, N,
    N, N, X, X, X, X, X, N, N, N,
    N, N, N, X, X, X, N, N, N, N,
    N, N, N, X, X, X, N, N, N, N,
]);

pub static DEATH_ANIM: AnimationData = AnimationData {
    frames: &[&DEATH_0, &DEATH_1, &DEATH_2, &DEATH_3],
    frame_duration: 0.15,
    looping: false,
};
