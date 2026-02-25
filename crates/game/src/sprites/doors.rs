#![allow(dead_code)]

use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;
const W: Option<Color> = Some([139, 90, 43]); // wood main
const P: Option<Color> = Some([120, 75, 35]); // wood dark plank
const L: Option<Color> = Some([160, 110, 60]); // wood light plank
const I: Option<Color> = Some([60, 60, 60]); // iron bands/hinges
const F: Option<Color> = Some([40, 35, 30]); // dark floor (open doorway)
const M: Option<Color> = Some([100, 70, 40]); // door frame

/// Closed wooden door. Brown wood planks with dark iron bands across.
#[rustfmt::skip]
pub static DOOR_CLOSED: SpriteData = SpriteData::new(8, 8, &[
    I, I, I, I, I, I, I, I,
    W, P, L, W, W, L, P, W,
    W, L, W, P, P, W, L, W,
    I, I, I, I, I, I, I, I,
    P, W, L, W, W, L, W, P,
    L, W, P, W, W, P, W, L,
    I, I, I, I, I, I, I, I,
    W, P, W, L, L, W, P, W,
]);

/// Open doorway. Dark floor with thin door frame edges on left and right.
#[rustfmt::skip]
pub static DOOR_OPEN: SpriteData = SpriteData::new(8, 8, &[
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
    M, F, F, F, F, F, F, M,
]);
