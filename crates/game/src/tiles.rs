use engine::color::Color;
use engine::sprite::SpriteData;
use engine::TileType;

// Floor colors — dark gray stone
const A: Option<Color> = Some([45, 45, 50]);
const B: Option<Color> = Some([40, 40, 45]);
const C: Option<Color> = Some([35, 35, 40]); // mortar

// Wall colors — darker brick
const D: Option<Color> = Some([60, 55, 50]);
const E: Option<Color> = Some([50, 45, 40]);
const F: Option<Color> = Some([70, 65, 55]);

// WallTop colors — lighter ledge
const G: Option<Color> = Some([80, 75, 65]);
const H: Option<Color> = Some([70, 65, 55]);

// Door colors — warm wood
const J: Option<Color> = Some([120, 80, 40]);
const K: Option<Color> = Some([100, 65, 30]);
const L: Option<Color> = Some([140, 95, 50]);

// Pit colors — dark void
const P: Option<Color> = Some([15, 10, 20]);
const Q: Option<Color> = Some([10, 5, 15]);

/// Floor: stone blocks with mortar lines.
#[rustfmt::skip]
static FLOOR: SpriteData = SpriteData::new(
    8,
    8,
    &[
        A, A, A, C, B, B, A, B,
        A, B, A, C, B, A, B, B,
        C, C, C, C, C, C, C, C,
        B, B, A, B, A, A, A, C,
        B, A, B, B, A, B, A, C,
        C, C, C, C, C, C, C, C,
        A, A, A, C, B, B, A, B,
        A, B, A, C, B, A, B, B,
    ],
);

/// Wall: darker brick pattern.
#[rustfmt::skip]
static WALL: SpriteData = SpriteData::new(
    8,
    8,
    &[
        D, D, D, E, F, F, D, D,
        D, F, D, E, D, D, F, D,
        E, E, E, E, E, E, E, E,
        F, D, D, E, D, D, D, E,
        D, D, F, E, D, F, D, E,
        E, E, E, E, E, E, E, E,
        D, D, D, E, F, F, D, D,
        D, F, D, E, D, D, F, D,
    ],
);

/// WallTop: lighter ledge cap.
#[rustfmt::skip]
static WALL_TOP: SpriteData = SpriteData::new(
    8,
    8,
    &[
        G, G, G, G, G, G, G, G,
        G, H, G, G, H, G, G, H,
        H, H, H, H, H, H, H, H,
        G, G, H, G, G, G, H, G,
        G, G, G, G, H, G, G, G,
        H, H, H, H, H, H, H, H,
        G, H, G, G, G, H, G, G,
        H, G, G, H, G, G, G, H,
    ],
);

/// Door: vertical wood planks.
#[rustfmt::skip]
static DOOR: SpriteData = SpriteData::new(
    8,
    8,
    &[
        J, K, J, J, L, J, K, J,
        J, K, J, J, L, J, K, J,
        J, K, J, J, L, J, K, J,
        L, L, L, L, L, L, L, L,
        J, K, J, J, L, J, K, J,
        J, K, J, J, L, J, K, J,
        J, K, J, J, L, J, K, J,
        L, L, L, L, L, L, L, L,
    ],
);

/// Pit: dark void with subtle purple.
#[rustfmt::skip]
static PIT: SpriteData = SpriteData::new(
    8,
    8,
    &[
        P, Q, P, P, Q, P, Q, P,
        Q, P, Q, P, P, Q, P, P,
        P, P, P, Q, P, P, Q, Q,
        Q, P, Q, P, Q, P, P, P,
        P, Q, P, P, P, Q, P, Q,
        P, P, Q, P, Q, P, Q, P,
        Q, P, P, Q, P, P, P, Q,
        P, Q, P, P, Q, P, Q, P,
    ],
);

/// Map a TileType to its sprite. Used as the tile_sprite callback for render_tilemap.
pub fn tile_sprite(tile: TileType) -> &'static SpriteData {
    match tile {
        TileType::Floor => &FLOOR,
        TileType::Wall => &WALL,
        TileType::WallTop => &WALL_TOP,
        TileType::Door => &DOOR,
        TileType::Pit => &PIT,
    }
}
