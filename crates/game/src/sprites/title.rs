#![allow(dead_code)]

use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;

// 5-band vertical gradient: icy blue at top to near-white at bottom
const A: Option<Color> = Some([70, 120, 255]);   // rows 0-1:  deep icy blue
const B: Option<Color> = Some([100, 150, 255]);  // rows 2-3:  icy blue
const C: Option<Color> = Some([140, 185, 255]);  // rows 4-5:  mid blue
const D: Option<Color> = Some([180, 210, 255]);  // rows 6-7:  light blue
const E: Option<Color> = Some([220, 232, 255]);  // rows 8-10: near white

// Dark outline/shadow for bottom edges
const K: Option<Color> = Some([30, 45, 100]);

/// CRYPTFALL title logo — 58x11 pixels.
/// Chunky NES-style block letters, 2px-wide strokes, icy blue-to-white gradient.
/// Each letter is 5 columns wide with 1-2 col gaps. 9 letters total.
///
/// Letter grid (each letter 5w, gap 1-2):
///   C(5) 1 R(5) 1 Y(5) 1 P(5) 1 T(5) 1 F(5) 1 A(5) 1 L(5) 1 L(5)
///   = 45 letter + 8 gap = 53..58 depending on spacing
#[rustfmt::skip]
pub static TITLE_CRYPTFALL: SpriteData = SpriteData::new(58, 11, &[
    // Row 0 (color A — deep icy blue)
    //  C           .  R           .  Y           .  P           .  T           .  F           .  A           .  L           .  L
    N, A, A, A, A, N, N, A, A, A, N, N, N, A, N, A, N, N, N, A, A, A, A, N, N, A, A, A, A, A, N, N, A, A, A, A, N, N, N, A, A, N, N, N, A, N, N, N, N, N, A, N, N, N, N, N, N, N,
    // Row 1 (color A)
    A, A, N, N, A, A, N, A, A, N, A, A, N, A, A, N, A, A, N, A, A, N, A, A, N, N, N, A, A, N, N, A, A, N, N, A, A, N, A, A, N, A, A, N, A, A, N, N, N, N, A, A, N, N, N, N, N, N,
    // Row 2 (color B)
    B, B, N, N, N, N, N, B, B, N, B, B, N, B, B, N, B, B, N, B, B, N, B, B, N, N, N, B, B, N, N, B, B, N, N, N, N, N, B, B, B, B, N, N, B, B, N, N, N, N, B, B, N, N, N, N, N, N,
    // Row 3 (color B)
    B, B, N, N, N, N, N, B, B, B, B, N, N, N, B, B, B, N, N, B, B, B, B, N, N, N, N, B, B, N, N, B, B, B, B, N, N, B, B, N, B, B, N, N, B, B, N, N, N, N, B, B, N, N, N, N, N, N,
    // Row 4 (color C)
    C, C, N, N, N, N, N, C, C, N, C, C, N, N, N, C, C, N, N, C, C, N, N, N, N, N, N, C, C, N, N, C, C, N, N, N, N, C, C, N, C, C, N, N, C, C, N, N, N, N, C, C, N, N, N, N, N, N,
    // Row 5 (color C)
    C, C, N, N, N, N, N, C, C, N, C, C, N, N, N, C, C, N, N, C, C, N, N, N, N, N, N, C, C, N, N, C, C, N, N, N, N, C, C, N, C, C, N, N, C, C, N, N, N, N, C, C, N, N, N, N, N, N,
    // Row 6 (color D)
    D, D, N, N, N, N, N, D, D, N, D, D, N, N, N, D, D, N, N, D, D, N, N, N, N, N, N, D, D, N, N, D, D, N, N, N, N, D, D, N, D, D, N, N, D, D, N, N, N, N, D, D, N, N, N, N, N, N,
    // Row 7 (color D)
    D, D, N, N, D, D, N, D, D, N, D, D, N, N, N, D, D, N, N, D, D, N, N, N, N, N, N, D, D, N, N, D, D, N, N, N, N, D, D, N, D, D, N, N, D, D, N, N, N, N, D, D, N, N, N, N, N, N,
    // Row 8 (color E — near white)
    N, E, E, E, E, N, N, E, E, N, E, E, N, N, N, E, E, N, N, E, E, N, N, N, N, N, N, E, E, N, N, E, E, N, N, N, N, E, E, N, E, E, N, N, E, E, E, E, E, N, E, E, E, E, E, N, N, N,
    // Row 9: bottom dark outline
    N, K, K, K, K, N, N, K, K, N, K, K, N, N, N, K, K, N, N, K, K, N, N, N, N, N, N, K, K, N, N, K, K, N, N, N, N, K, K, N, K, K, N, N, K, K, K, K, K, N, K, K, K, K, K, N, N, N,
    // Row 10: drop shadow
    N, N, K, K, N, N, N, N, K, N, N, K, N, N, N, N, K, N, N, N, K, N, N, N, N, N, N, N, K, N, N, N, K, N, N, N, N, N, K, N, N, K, N, N, N, K, K, K, N, N, N, K, K, K, N, N, N, N,
]);
