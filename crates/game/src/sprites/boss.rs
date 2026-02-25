#![allow(dead_code)]

use engine::animation::AnimationData;
use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;
const B: Option<Color> = Some([230, 220, 200]); // bone white
const D: Option<Color> = Some([200, 190, 170]); // bone shadow
const E: Option<Color> = Some([160, 150, 130]); // dark bone
const R: Option<Color> = Some([100, 20, 20]); // maroon robe
const M: Option<Color> = Some([80, 15, 15]); // dark maroon
const G: Option<Color> = Some([255, 200, 50]); // gold crown
const K: Option<Color> = Some([200, 160, 30]); // dark gold
const F: Option<Color> = Some([255, 40, 40]); // glowing red eyes
const W: Option<Color> = Some([190, 185, 175]); // weapon bone
const X: Option<Color> = Some([220, 215, 205]); // weapon highlight

/// Bone King idle stance. Standing upright holding massive bone greatsword.
/// 20x24 pixels — double the size of normal enemies for boss presence.
#[rustfmt::skip]
pub static BONE_KING_IDLE: SpriteData = SpriteData::new(20, 24, &[
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, // 0  crown
    N, N, N, N, N, N, G, G, K, G, G, K, G, G, N, N, N, N, N, N, // 1  crown
    N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, // 2  crown base
    N, N, N, N, N, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, // 3  skull top
    N, N, N, N, B, B, B, B, B, B, B, B, B, B, B, B, N, N, N, N, // 4  skull
    N, N, N, N, B, B, F, B, B, B, B, B, B, F, B, B, N, N, N, N, // 5  eyes
    N, N, N, N, N, B, B, B, D, D, D, D, B, B, B, N, N, N, N, N, // 6  nose
    N, N, N, N, N, B, D, B, B, B, B, B, B, D, B, N, N, N, N, N, // 7  jaw
    N, N, N, N, N, N, D, D, E, D, D, E, D, D, N, N, N, N, N, N, // 8  neck
    N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, // 9  robe collar
    N, N, N, N, R, R, M, R, R, R, R, R, R, M, R, R, N, N, N, N, // 10 shoulders
    N, N, N, B, R, R, M, R, R, R, R, R, R, M, R, R, B, N, N, N, // 11 upper arms
    N, N, N, D, R, R, R, R, R, R, R, R, R, R, R, R, D, W, N, N, // 12 torso + weapon start
    N, N, N, E, R, R, R, R, M, R, R, M, R, R, R, R, E, W, N, N, // 13 mid torso + weapon
    N, N, N, N, B, R, R, R, R, R, R, R, R, R, R, B, N, W, N, N, // 14 waist + weapon
    N, N, N, N, D, R, R, R, R, R, R, R, R, R, R, D, N, X, N, N, // 15 lower waist + weapon
    N, N, N, N, N, R, R, R, M, R, R, M, R, R, R, N, N, W, N, N, // 16 hips + weapon
    N, N, N, N, N, M, R, R, R, R, R, R, R, R, M, N, N, X, N, N, // 17 upper legs + weapon
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, W, N, N, // 18 legs
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 19 shins
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 20 shins
    N, N, N, N, N, N, E, B, E, N, N, E, B, E, N, N, N, N, N, N, // 21 ankles
    N, N, N, N, N, E, E, B, E, E, E, E, B, E, E, N, N, N, N, N, // 22 feet
    N, N, N, N, N, E, E, E, E, E, E, E, E, E, E, N, N, N, N, N, // 23 feet base
]);

/// Bone King slam attack — weapon raised overhead, about to slam down.
/// Body leaning back slightly with weapon high above the head.
#[rustfmt::skip]
pub static BONE_KING_SLAM: SpriteData = SpriteData::new(20, 24, &[
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    N, N, N, N, N, N, N, N, N, X, W, X, N, N, N, N, N, N, N, N, // 0  weapon raised tip
    N, N, N, N, N, N, N, N, N, W, X, W, N, N, N, N, N, N, N, N, // 1  weapon shaft
    N, N, N, N, N, N, N, N, N, W, W, W, N, N, N, N, N, N, N, N, // 2  weapon shaft
    N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, // 3  crown
    N, N, N, N, N, N, G, G, K, G, G, K, G, G, N, N, N, N, N, N, // 4  crown
    N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, // 5  crown base
    N, N, N, N, N, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, // 6  skull top
    N, N, N, N, B, B, B, B, B, B, B, B, B, B, B, B, N, N, N, N, // 7  skull
    N, N, N, N, B, B, F, B, B, B, B, B, B, F, B, B, N, N, N, N, // 8  eyes
    N, N, N, N, N, B, B, B, D, D, D, D, B, B, B, N, N, N, N, N, // 9  nose
    N, N, N, N, N, B, D, B, B, B, B, B, B, D, B, N, N, N, N, N, // 10 jaw
    N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, // 11 collar
    N, N, N, N, R, R, M, R, R, R, R, R, R, M, R, R, N, N, N, N, // 12 shoulders (arms up)
    N, N, N, B, D, R, R, R, R, R, R, R, R, R, R, D, B, N, N, N, // 13 upper arms raised
    N, N, N, N, R, R, R, R, R, R, R, R, R, R, R, R, N, N, N, N, // 14 torso
    N, N, N, N, R, R, R, R, M, R, R, M, R, R, R, R, N, N, N, N, // 15 mid torso
    N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, // 16 waist
    N, N, N, N, N, M, R, R, R, R, R, R, R, R, M, N, N, N, N, N, // 17 hips
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 18 legs
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 19 shins
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 20 shins
    N, N, N, N, N, N, E, B, E, N, N, E, B, E, N, N, N, N, N, N, // 21 ankles
    N, N, N, N, N, E, E, B, E, E, E, E, B, E, E, N, N, N, N, N, // 22 feet
    N, N, N, N, N, E, E, E, E, E, E, E, E, E, E, N, N, N, N, N, // 23 feet base
]);

/// Bone King wide sweep attack — weapon extended to the side.
/// 22x24 pixels (wider frame to accommodate weapon reach).
#[rustfmt::skip]
pub static BONE_KING_SWEEP: SpriteData = SpriteData::new(22, 24, &[
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21
    N, N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, N, // 0  crown
    N, N, N, N, N, N, N, G, G, K, G, G, K, G, G, N, N, N, N, N, N, N, // 1  crown
    N, N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, N, // 2  crown base
    N, N, N, N, N, N, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, N, // 3  skull top
    N, N, N, N, N, B, B, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, // 4  skull
    N, N, N, N, N, B, B, F, B, B, B, B, B, B, F, B, B, N, N, N, N, N, // 5  eyes
    N, N, N, N, N, N, B, B, B, D, D, D, D, B, B, B, N, N, N, N, N, N, // 6  nose
    N, N, N, N, N, N, B, D, B, B, B, B, B, B, D, B, N, N, N, N, N, N, // 7  jaw
    N, N, N, N, N, N, N, D, D, E, D, D, E, D, D, N, N, N, N, N, N, N, // 8  neck
    N, N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, N, // 9  collar
    N, N, N, N, N, R, R, M, R, R, R, R, R, R, M, R, R, N, N, N, N, N, // 10 shoulders
    X, W, X, W, B, D, R, R, R, R, R, R, R, R, R, R, D, B, N, N, N, N, // 11 weapon extended + arms
    N, N, N, N, N, E, R, R, R, R, R, R, R, R, R, R, E, N, N, N, N, N, // 12 torso
    N, N, N, N, N, N, R, R, R, M, R, R, M, R, R, R, N, N, N, N, N, N, // 13 mid torso
    N, N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, N, // 14 waist
    N, N, N, N, N, N, M, R, R, R, R, R, R, R, R, M, N, N, N, N, N, N, // 15 lower waist
    N, N, N, N, N, N, N, R, R, M, R, R, M, R, R, N, N, N, N, N, N, N, // 16 hips
    N, N, N, N, N, N, N, M, R, R, R, R, R, R, M, N, N, N, N, N, N, N, // 17 upper legs
    N, N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, N, // 18 legs
    N, N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, N, // 19 shins
    N, N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, N, // 20 shins
    N, N, N, N, N, N, N, E, B, E, N, N, E, B, E, N, N, N, N, N, N, N, // 21 ankles
    N, N, N, N, N, N, E, E, B, E, E, E, E, B, E, E, N, N, N, N, N, N, // 22 feet
    N, N, N, N, N, N, E, E, E, E, E, E, E, E, E, E, N, N, N, N, N, N, // 23 feet base
]);

/// Bone King charging forward — body leaning into the rush.
#[rustfmt::skip]
pub static BONE_KING_CHARGE: SpriteData = SpriteData::new(20, 24, &[
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, // 0  (empty - body shifted down)
    N, N, N, N, N, N, N, N, K, G, G, G, G, K, N, N, N, N, N, N, // 1  crown (shifted right/forward)
    N, N, N, N, N, N, N, N, G, G, K, G, K, G, G, N, N, N, N, N, // 2  crown
    N, N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, // 3  crown base
    N, N, N, N, N, N, N, B, B, B, B, B, B, B, B, B, N, N, N, N, // 4  skull
    N, N, N, N, N, N, B, B, B, B, B, B, B, B, B, B, B, N, N, N, // 5  skull wide
    N, N, N, N, N, N, B, B, F, B, B, B, B, B, F, B, B, N, N, N, // 6  eyes
    N, N, N, N, N, N, N, B, B, D, D, D, D, B, B, N, N, N, N, N, // 7  nose
    N, N, N, N, N, N, N, B, D, B, B, B, B, D, B, N, N, N, N, N, // 8  jaw
    N, N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, // 9  collar (leaning)
    N, N, N, N, N, R, R, M, R, R, R, R, R, R, M, R, R, N, N, N, // 10 shoulders
    N, N, N, N, B, R, R, R, R, R, R, R, R, R, R, R, B, W, N, N, // 11 arms + weapon
    N, N, N, N, D, R, R, R, R, R, R, R, R, R, R, R, D, W, N, N, // 12 torso
    N, N, N, N, N, R, R, R, M, R, R, M, R, R, R, R, N, X, N, N, // 13 mid torso
    N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, W, N, N, // 14 waist
    N, N, N, N, N, M, R, R, R, R, R, R, R, R, M, N, N, N, N, N, // 15 hips
    N, N, N, N, N, N, R, R, M, R, R, M, R, R, N, N, N, N, N, N, // 16 hips
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 17 legs (stride)
    N, N, N, N, N, D, B, D, N, N, N, N, D, B, D, N, N, N, N, N, // 18 wide stride
    N, N, N, N, N, D, B, D, N, N, N, N, D, B, D, N, N, N, N, N, // 19 shins
    N, N, N, N, D, B, D, N, N, N, N, N, N, D, B, D, N, N, N, N, // 20 wide shins
    N, N, N, N, E, B, E, N, N, N, N, N, N, E, B, E, N, N, N, N, // 21 ankles
    N, N, N, E, E, B, E, E, N, N, N, N, E, E, B, E, E, N, N, N, // 22 feet
    N, N, N, E, E, E, E, E, N, N, N, N, E, E, E, E, E, N, N, N, // 23 feet base
]);

/// Bone King stunned — dazed pose after hitting a wall. Body slumped.
#[rustfmt::skip]
pub static BONE_KING_STUNNED: SpriteData = SpriteData::new(20, 24, &[
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    N, N, N, N, N, N, N, N, G, N, N, N, G, N, N, N, N, N, N, N, // 0  crown askew (tilted)
    N, N, N, N, N, K, G, G, G, G, G, G, G, K, N, N, N, N, N, N, // 1  crown slipping
    N, N, N, N, N, G, G, K, G, G, K, G, G, N, N, N, N, N, N, N, // 2  crown
    N, N, N, N, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, N, // 3  skull (tilted left)
    N, N, N, B, B, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, // 4  skull
    N, N, N, B, B, F, B, B, B, B, B, B, F, B, B, N, N, N, N, N, // 5  eyes (dazed)
    N, N, N, N, B, B, B, D, D, D, D, B, B, B, N, N, N, N, N, N, // 6  nose
    N, N, N, N, B, D, B, B, B, B, B, B, D, B, N, N, N, N, N, N, // 7  jaw (slack)
    N, N, N, N, N, D, D, E, D, D, E, D, D, N, N, N, N, N, N, N, // 8  neck
    N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, N, // 9  collar (slumped)
    N, N, N, R, R, M, R, R, R, R, R, R, M, R, R, N, N, N, N, N, // 10 shoulders drooped
    N, N, B, D, R, R, R, R, R, R, R, R, R, R, D, B, N, N, N, N, // 11 arms hanging
    N, N, E, N, R, R, R, R, R, R, R, R, R, R, N, E, N, N, N, N, // 12 torso slumped
    N, N, N, N, R, R, R, M, R, R, M, R, R, R, N, N, N, N, N, N, // 13 mid torso
    N, N, N, N, N, R, R, R, R, R, R, R, R, N, N, N, W, N, N, N, // 14 waist + weapon dropped
    N, N, N, N, N, R, R, R, R, R, R, R, R, N, N, N, W, N, N, N, // 15 lower waist
    N, N, N, N, N, M, R, R, M, R, M, R, R, M, N, N, X, N, N, N, // 16 hips
    N, N, N, N, N, N, D, B, D, N, D, B, D, N, N, N, W, N, N, N, // 17 upper legs
    N, N, N, N, N, N, D, B, D, N, D, B, D, N, N, N, N, N, N, N, // 18 legs
    N, N, N, N, N, N, D, B, D, N, D, B, D, N, N, N, N, N, N, N, // 19 shins
    N, N, N, N, N, N, D, B, D, N, D, B, D, N, N, N, N, N, N, N, // 20 shins
    N, N, N, N, N, N, E, B, E, N, E, B, E, N, N, N, N, N, N, N, // 21 ankles
    N, N, N, N, N, E, E, B, E, E, E, B, E, E, N, N, N, N, N, N, // 22 feet
    N, N, N, N, N, E, E, E, E, E, E, E, E, E, N, N, N, N, N, N, // 23 feet base
]);

/// Bone King roar — head tilted back, mouth open. Phase transition pose.
#[rustfmt::skip]
pub static BONE_KING_ROAR: SpriteData = SpriteData::new(20, 24, &[
//   0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19
    N, N, N, N, N, N, N, G, K, G, G, K, G, N, N, N, N, N, N, N, // 0  crown tips (tilted back)
    N, N, N, N, N, N, K, G, G, G, G, G, G, K, N, N, N, N, N, N, // 1  crown
    N, N, N, N, N, N, G, G, K, G, G, K, G, G, N, N, N, N, N, N, // 2  crown
    N, N, N, N, N, B, B, B, B, B, B, B, B, B, B, N, N, N, N, N, // 3  skull tilted back
    N, N, N, N, B, B, B, B, B, B, B, B, B, B, B, B, N, N, N, N, // 4  skull
    N, N, N, N, B, B, F, B, B, B, B, B, B, F, B, B, N, N, N, N, // 5  eyes (fierce)
    N, N, N, N, N, B, B, D, D, D, D, D, D, B, B, N, N, N, N, N, // 6  nose
    N, N, N, N, B, D, B, B, B, B, B, B, B, B, D, B, N, N, N, N, // 7  jaw wide open
    N, N, N, N, N, B, E, E, D, D, D, D, E, E, B, N, N, N, N, N, // 8  open mouth interior
    N, N, N, N, N, N, D, B, B, E, E, B, B, D, N, N, N, N, N, N, // 9  lower jaw
    N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, // 10 collar
    N, N, N, B, R, R, M, R, R, R, R, R, R, M, R, R, B, N, N, N, // 11 shoulders (puffed out)
    N, N, B, D, R, R, R, R, R, R, R, R, R, R, R, R, D, B, N, N, // 12 arms out (roaring)
    N, N, E, N, R, R, R, R, R, R, R, R, R, R, R, R, N, E, N, N, // 13 torso expanded
    N, N, N, N, R, R, R, R, M, R, R, M, R, R, R, R, N, N, N, N, // 14 mid torso
    N, N, N, N, N, R, R, R, R, R, R, R, R, R, R, N, N, N, N, N, // 15 waist
    N, N, N, N, N, M, R, R, R, R, R, R, R, R, M, N, N, N, N, N, // 16 hips
    N, N, N, N, N, N, R, R, M, R, R, M, R, R, N, N, N, N, N, N, // 17 upper legs
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 18 legs
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 19 shins
    N, N, N, N, N, N, D, B, D, N, N, D, B, D, N, N, N, N, N, N, // 20 shins
    N, N, N, N, N, N, E, B, E, N, N, E, B, E, N, N, N, N, N, N, // 21 ankles
    N, N, N, N, N, E, E, B, E, E, E, E, B, E, E, N, N, N, N, N, // 22 feet
    N, N, N, N, N, E, E, E, E, E, E, E, E, E, E, N, N, N, N, N, // 23 feet base
]);

// =============================================================================
// ANIMATION DEFINITIONS — single-frame anims for each boss pose
// =============================================================================

pub static BONE_KING_IDLE_ANIM: AnimationData = AnimationData {
    frames: &[&BONE_KING_IDLE],
    frame_duration: 0.5,
    looping: true,
};

pub static BONE_KING_SLAM_ANIM: AnimationData = AnimationData {
    frames: &[&BONE_KING_SLAM],
    frame_duration: 0.5,
    looping: true,
};

pub static BONE_KING_SWEEP_ANIM: AnimationData = AnimationData {
    frames: &[&BONE_KING_SWEEP],
    frame_duration: 0.5,
    looping: true,
};

pub static BONE_KING_CHARGE_ANIM: AnimationData = AnimationData {
    frames: &[&BONE_KING_CHARGE],
    frame_duration: 0.5,
    looping: true,
};

pub static BONE_KING_STUNNED_ANIM: AnimationData = AnimationData {
    frames: &[&BONE_KING_STUNNED],
    frame_duration: 0.5,
    looping: true,
};

pub static BONE_KING_ROAR_ANIM: AnimationData = AnimationData {
    frames: &[&BONE_KING_ROAR],
    frame_duration: 0.5,
    looping: true,
};
