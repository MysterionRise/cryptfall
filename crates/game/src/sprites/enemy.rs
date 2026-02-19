#![allow(dead_code)]

use engine::animation::AnimationData;
use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None; // transparent
const G: Option<Color> = Some([60, 160, 60]); // green body
const D: Option<Color> = Some([30, 100, 30]); // dark shadow
const L: Option<Color> = Some([120, 210, 120]); // light highlight
const R: Option<Color> = Some([200, 40, 40]); // red eyes
const W: Option<Color> = Some([255, 255, 255]); // white (death flash)

// =============================================================================
// IDLE — 2 frames, 0.5s/frame, looping
// Subtle squish: frame 1 is slightly wider and shorter.
// =============================================================================

#[rustfmt::skip]
static ENEMY_IDLE_0: SpriteData = SpriteData::new(10, 10, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, L, L, L, L, N, N, N,
    N, N, L, G, G, G, G, L, N, N,
    N, N, G, R, G, G, R, G, N, N,
    N, N, G, G, G, G, G, G, N, N,
    N, N, G, G, G, G, G, G, N, N,
    N, N, D, G, G, G, G, D, N, N,
    N, N, N, D, G, G, D, N, N, N,
    N, N, N, D, D, D, D, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static ENEMY_IDLE_1: SpriteData = SpriteData::new(10, 10, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, L, L, L, L, L, L, N, N,
    N, L, G, R, G, G, R, G, L, N,
    N, G, G, G, G, G, G, G, G, N,
    N, G, G, G, G, G, G, G, G, N,
    N, D, G, G, G, G, G, G, D, N,
    N, N, D, D, G, G, D, D, N, N,
    N, N, N, D, D, D, D, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static ENEMY_IDLE_ANIM: AnimationData = AnimationData {
    frames: &[&ENEMY_IDLE_0, &ENEMY_IDLE_1],
    frame_duration: 0.5,
    looping: true,
};

// =============================================================================
// DEATH — 4 frames, 0.15s/frame, one-shot
// Flatten → squash → white flash → fade
// =============================================================================

#[rustfmt::skip]
static ENEMY_DEATH_0: SpriteData = SpriteData::new(10, 10, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, L, L, L, L, L, L, N, N,
    N, L, G, G, G, G, G, G, L, N,
    N, G, G, G, G, G, G, G, G, N,
    N, D, G, G, G, G, G, G, D, N,
    N, N, D, D, D, D, D, D, N, N,
    N, N, N, D, D, D, D, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static ENEMY_DEATH_1: SpriteData = SpriteData::new(10, 10, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, L, L, L, L, L, L, L, L, N,
    N, G, G, G, G, G, G, G, G, N,
    N, D, D, D, D, D, D, D, D, N,
    N, N, D, D, D, D, D, D, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static ENEMY_DEATH_2: SpriteData = SpriteData::new(10, 10, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, W, W, W, W, W, W, W, W, N,
    N, W, W, W, W, W, W, W, W, N,
    N, N, W, W, W, W, W, W, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

#[rustfmt::skip]
static ENEMY_DEATH_3: SpriteData = SpriteData::new(10, 10, &[
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
    N, N, W, N, W, W, N, W, N, N,
    N, N, N, W, N, N, W, N, N, N,
    N, N, N, N, N, N, N, N, N, N,
]);

pub static ENEMY_DEATH_ANIM: AnimationData = AnimationData {
    frames: &[
        &ENEMY_DEATH_0,
        &ENEMY_DEATH_1,
        &ENEMY_DEATH_2,
        &ENEMY_DEATH_3,
    ],
    frame_duration: 0.15,
    looping: false,
};
