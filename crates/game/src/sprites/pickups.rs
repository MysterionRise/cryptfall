#![allow(dead_code)]

use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;

// Heart colors
const R: Option<Color> = Some([220, 40, 40]); // bright red
const D: Option<Color> = Some([180, 25, 25]); // darker red
const H: Option<Color> = Some([255, 100, 100]); // highlight red
const W: Option<Color> = Some([255, 200, 200]); // white highlight (big heart)

// Coin colors
const G: Option<Color> = Some([255, 200, 50]); // gold
const K: Option<Color> = Some([200, 160, 30]); // darker gold
const L: Option<Color> = Some([255, 230, 120]); // gold highlight

/// Small red heart pickup for +1 HP. Classic heart shape, 5x5.
#[rustfmt::skip]
pub static PICKUP_HEART_SMALL: SpriteData = SpriteData::new(5, 5, &[
    N, H, N, H, N,
    R, R, R, R, R,
    R, R, R, R, R,
    N, D, R, D, N,
    N, N, D, N, N,
]);

/// Big bright heart pickup for +3 HP. Larger with white highlight, 7x7.
#[rustfmt::skip]
pub static PICKUP_HEART_BIG: SpriteData = SpriteData::new(7, 7, &[
    N, H, H, N, H, H, N,
    H, W, R, R, R, W, H,
    R, R, R, R, R, R, R,
    R, R, R, R, R, R, R,
    N, R, R, R, R, R, N,
    N, N, D, R, D, N, N,
    N, N, N, D, N, N, N,
]);

/// Gold coin pickup. Circle shape, 5x5.
#[rustfmt::skip]
pub static PICKUP_COIN: SpriteData = SpriteData::new(5, 5, &[
    N, K, G, K, N,
    K, G, L, G, K,
    G, L, G, G, G,
    K, G, G, G, K,
    N, K, G, K, N,
]);
