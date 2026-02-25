#![allow(dead_code)]

use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;

// --- Projectile orb: 3x3, bright center with dim surround ---
const C: Option<Color> = Some([60, 200, 255]); // cyan center
const B: Option<Color> = Some([30, 100, 180]); // blue edge

#[rustfmt::skip]
pub static PROJECTILE_ORB: SpriteData = SpriteData::new(3, 3, &[
    N, B, N,
    B, C, B,
    N, B, N,
]);

// --- Heart sprites for HUD: 5x5 ---
const R: Option<Color> = Some([220, 30, 30]); // red heart
const D: Option<Color> = Some([160, 20, 20]); // dark red
const G: Option<Color> = Some([80, 80, 80]); // gray (empty heart)
const E: Option<Color> = Some([50, 50, 50]); // dark gray

#[rustfmt::skip]
pub static HEART_FULL: SpriteData = SpriteData::new(5, 5, &[
    N, R, N, R, N,
    R, R, R, R, R,
    R, R, R, R, R,
    N, D, R, D, N,
    N, N, D, N, N,
]);

#[rustfmt::skip]
pub static HEART_EMPTY: SpriteData = SpriteData::new(5, 5, &[
    N, G, N, G, N,
    G, E, G, E, G,
    G, E, E, E, G,
    N, E, E, E, N,
    N, N, E, N, N,
]);

// --- Spawn warning indicator: 7x7, red-orange ring on ground ---
const O: Option<Color> = Some([255, 100, 30]); // outer orange
const I: Option<Color> = Some([255, 60, 10]); // inner red-orange
const F: Option<Color> = Some([80, 30, 10]); // dim fill center

/// Red-orange warning ring that appears on the ground before enemies spawn.
#[rustfmt::skip]
pub static SPAWN_WARNING: SpriteData = SpriteData::new(7, 7, &[
    N, N, O, O, O, N, N,
    N, O, I, I, I, O, N,
    O, I, F, F, F, I, O,
    O, I, F, F, F, I, O,
    O, I, F, F, F, I, O,
    N, O, I, I, I, O, N,
    N, N, O, O, O, N, N,
]);
