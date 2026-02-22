#![allow(dead_code)]

use engine::animation::AnimationData;
use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;
const B: Option<Color> = Some([220, 210, 190]); // bone white
const D: Option<Color> = Some([160, 150, 130]); // bone shadow
const E: Option<Color> = Some([100, 90, 80]); // dark bone
const R: Option<Color> = Some([200, 40, 40]); // red eyes
const W: Option<Color> = Some([180, 180, 190]); // weapon steel
const X: Option<Color> = Some([220, 220, 230]); // weapon highlight
const K: Option<Color> = Some([255, 200, 50]); // telegraph glow

// =============================================================================
// IDLE — 2 frames, 0.5s/frame, looping. Subtle sway.
// 10x14 to match player proportions.
// =============================================================================

#[rustfmt::skip]
static SKEL_IDLE_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, B, B, B, B, N, N, N,
    N, N, B, B, B, B, B, B, N, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, N, N,
    N, N, D, B, B, B, B, D, W, N,
    N, N, N, B, B, B, B, N, W, N,
    N, N, N, D, B, B, D, N, W, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
]);

#[rustfmt::skip]
static SKEL_IDLE_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, B, B, B, B, B, B, N, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, N, W, N,
    N, N, N, D, B, B, D, N, W, N,
    N, N, N, N, D, D, N, N, W, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static SKEL_IDLE_ANIM: AnimationData = AnimationData {
    frames: &[&SKEL_IDLE_0, &SKEL_IDLE_1],
    frame_duration: 0.5,
    looping: true,
};

// =============================================================================
// WALK — 4 frames, 0.12s/frame, looping
// =============================================================================

#[rustfmt::skip]
static SKEL_WALK_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, B, B, B, B, N, N, N,
    N, N, B, B, B, B, B, B, N, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, N, N, N,
    N, N, N, D, B, B, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, D, N, N, N, D, N, N,
    N, N, N, D, N, N, N, D, N, N,
    N, N, N, E, N, N, N, E, N, N,
    N, N, E, E, N, N, N, E, E, N,
]);

#[rustfmt::skip]
static SKEL_WALK_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, B, B, B, B, N, N, N,
    N, N, B, B, B, B, B, B, N, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, N, N, N,
    N, N, N, D, B, B, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, E, E, N, N, N, N,
    N, N, N, E, E, E, E, N, N, N,
]);

pub static SKEL_WALK_ANIM: AnimationData = AnimationData {
    frames: &[&SKEL_WALK_0, &SKEL_WALK_1, &SKEL_WALK_0, &SKEL_WALK_1],
    frame_duration: 0.12,
    looping: true,
};

// =============================================================================
// WIND-UP — 2 frames, 0.2s/frame, one-shot. Weapon raised, telegraph glow.
// =============================================================================

#[rustfmt::skip]
static SKEL_WINDUP_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, B, B, B, B, W, N, N,
    N, N, B, B, B, B, B, B, X, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, N, N, N,
    N, N, N, D, B, B, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
]);

#[rustfmt::skip]
static SKEL_WINDUP_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, B, B, B, B, X, K, N,
    N, N, B, B, B, B, B, B, W, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, K, B, B, D, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, N, N, N,
    N, N, N, D, B, B, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
]);

pub static SKEL_WINDUP_ANIM: AnimationData = AnimationData {
    frames: &[&SKEL_WINDUP_0, &SKEL_WINDUP_1],
    frame_duration: 0.2,
    looping: false,
};

// =============================================================================
// ATTACK — 2 frames, 0.075s/frame, one-shot. Lunge with weapon extended.
// =============================================================================

#[rustfmt::skip]
static SKEL_ATTACK_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, B, B, B, B, N, N, N,
    N, N, B, B, B, B, B, B, N, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, W, X, X,
    N, N, N, D, B, B, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
]);

#[rustfmt::skip]
static SKEL_ATTACK_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, B, B, B, B, N, N, N,
    N, N, B, B, B, B, B, B, N, N,
    N, N, B, R, B, B, R, B, N, N,
    N, N, N, B, D, D, B, N, N, N,
    N, N, D, B, B, B, B, D, N, N,
    N, N, D, B, D, B, B, D, W, X,
    N, N, D, B, B, B, B, D, N, N,
    N, N, N, B, B, B, B, N, N, N,
    N, N, N, D, B, B, D, N, N, N,
    N, N, N, N, D, D, N, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
]);

pub static SKEL_ATTACK_ANIM: AnimationData = AnimationData {
    frames: &[&SKEL_ATTACK_0, &SKEL_ATTACK_1],
    frame_duration: 0.075,
    looping: false,
};

// =============================================================================
// STAGGER — 1 frame (reuse idle 1 tinted white), 0.3s, one-shot
// =============================================================================

pub static SKEL_STAGGER_ANIM: AnimationData = AnimationData {
    frames: &[&SKEL_IDLE_1],
    frame_duration: 0.3,
    looping: false,
};

// =============================================================================
// DEATH — 3 frames, 0.15s/frame, one-shot
// =============================================================================

#[rustfmt::skip]
static SKEL_DEATH_0: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, B, B, B, B, N, N,
    N, N, N, B, B, B, B, B, B, N,
    N, N, N, B, R, B, B, R, B, N,
    N, N, N, N, B, D, D, B, N, N,
    N, N, N, D, B, B, B, B, N, N,
    N, N, N, D, B, B, B, D, N, N,
    N, N, N, N, B, B, B, N, N, N,
    N, N, N, N, D, B, D, N, N, N,
    N, N, N, N, N, D, N, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, D, N, D, N, N, N,
    N, N, N, N, E, N, E, N, N, N,
    N, N, N, E, E, N, E, E, N, N,
]);

#[rustfmt::skip]
static SKEL_DEATH_1: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, B, B, B, N, N, N, N,
    N, N, B, D, D, B, N, N, N, N,
    N, N, D, B, B, B, D, N, N, N,
    N, N, D, B, B, B, D, N, N, N,
    N, N, N, E, D, E, N, N, N, N,
    N, N, N, E, E, E, N, N, N, N,
]);

#[rustfmt::skip]
static SKEL_DEATH_2: SpriteData = SpriteData::new(10, 14, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, B, N, B, D, N, B, N, N,
    N, N, N, D, N, N, E, N, N, N,
    N, N, N, N, E, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static SKEL_DEATH_ANIM: AnimationData = AnimationData {
    frames: &[&SKEL_DEATH_0, &SKEL_DEATH_1, &SKEL_DEATH_2],
    frame_duration: 0.15,
    looping: false,
};
