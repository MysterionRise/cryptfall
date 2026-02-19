use crate::framebuffer::FrameBuffer;
use crate::sprite::SpriteData;

pub const TILE_SIZE: usize = 8;

/// A tile in the map.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
    WallTop,
    Door,
    Pit,
}

/// A room made of tiles.
pub struct TileMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<TileType>,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Floor; width * height],
        }
    }

    pub fn get(&self, tx: usize, ty: usize) -> TileType {
        if tx < self.width && ty < self.height {
            self.tiles[ty * self.width + tx]
        } else {
            TileType::Wall // out of bounds = solid
        }
    }

    pub fn set(&mut self, tx: usize, ty: usize, tile: TileType) {
        if tx < self.width && ty < self.height {
            self.tiles[ty * self.width + tx] = tile;
        }
    }

    /// Wall and WallTop are solid.
    pub fn is_solid(&self, tx: usize, ty: usize) -> bool {
        matches!(self.get(tx, ty), TileType::Wall | TileType::WallTop)
    }

    /// Check if a pixel-space rectangle collides with any solid tile.
    pub fn collides(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        let ts = TILE_SIZE as f32;
        let tx0 = (x / ts).floor() as i32;
        let ty0 = (y / ts).floor() as i32;
        let tx1 = ((x + w - 0.01) / ts).floor() as i32;
        let ty1 = ((y + h - 0.01) / ts).floor() as i32;

        for tiy in ty0.max(0)..=ty1.min(self.height as i32 - 1) {
            for tix in tx0.max(0)..=tx1.min(self.width as i32 - 1) {
                if self.is_solid(tix as usize, tiy as usize) {
                    return true;
                }
            }
        }
        false
    }

    pub fn pixel_width(&self) -> usize {
        self.width * TILE_SIZE
    }

    pub fn pixel_height(&self) -> usize {
        self.height * TILE_SIZE
    }
}

/// Render visible tiles to the framebuffer.
/// `tile_sprite` maps each TileType to its sprite data.
pub fn render_tilemap(
    fb: &mut FrameBuffer,
    tilemap: &TileMap,
    tile_sprite: fn(TileType) -> &'static SpriteData,
    camera_x: i32,
    camera_y: i32,
) {
    let ts = TILE_SIZE as i32;
    let fb_w = fb.width() as i32;
    let fb_h = fb.height() as i32;

    // Calculate visible tile range
    let tx0 = (camera_x / ts).max(0) as usize;
    let ty0 = (camera_y / ts).max(0) as usize;
    let tx1 = ((camera_x + fb_w + ts - 1) / ts).min(tilemap.width as i32) as usize;
    let ty1 = ((camera_y + fb_h + ts - 1) / ts).min(tilemap.height as i32) as usize;

    for ty in ty0..ty1 {
        for tx in tx0..tx1 {
            let tile = tilemap.get(tx, ty);
            let sprite = tile_sprite(tile);
            let px = tx as i32 * ts - camera_x;
            let py = ty as i32 * ts - camera_y;
            fb.blit_sprite(sprite, px, py);
        }
    }
}
