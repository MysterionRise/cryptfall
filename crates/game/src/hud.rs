use engine::color::Color;
use engine::FrameBuffer;

use crate::dungeon::floor_gen::FloorLayout;
use crate::dungeon::room_template::RoomType;
use crate::sprites::effects::{HEART_EMPTY, HEART_FULL};
use crate::sprites::font::render_digit;

pub struct DamageNumber {
    pub value: i32,
    pub x: f32,
    pub y: f32,
    velocity_y: f32,
    lifetime: f32,
    max_lifetime: f32,
    pub color: Color,
}

impl DamageNumber {
    pub fn new(value: i32, x: f32, y: f32, color: Color) -> Self {
        Self {
            value,
            x,
            y,
            velocity_y: -30.0, // float upward
            lifetime: 0.0,
            max_lifetime: 0.8,
            color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime += dt;
        self.y += self.velocity_y * dt;
        self.velocity_y *= 0.95; // slow down
    }

    pub fn alive(&self) -> bool {
        self.lifetime < self.max_lifetime
    }

    pub fn render(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        // Fade based on remaining lifetime
        let fade = 1.0 - (self.lifetime / self.max_lifetime);
        let c = [
            (self.color[0] as f32 * fade) as u8,
            (self.color[1] as f32 * fade) as u8,
            (self.color[2] as f32 * fade) as u8,
        ];

        let sx = self.x as i32 - cam_x;
        let sy = self.y as i32 - cam_y;

        // Render each digit using the 3x5 pixel font
        let (digits, count) = get_digits(self.value);
        let mut offset_x = 0i32;
        for &digit in &digits[..count] {
            render_digit(fb, digit, sx + offset_x, sy, c);
            offset_x += 4; // 3 wide + 1 spacing
        }
    }
}

/// Returns digits of a non-negative integer as a fixed-size array.
/// Returns (array, count) where count is the number of valid digits.
fn get_digits(mut value: i32) -> ([u8; 5], usize) {
    if value <= 0 {
        let mut arr = [0u8; 5];
        arr[0] = 0;
        return (arr, 1);
    }
    let mut buf = [0u8; 5];
    let mut count = 0usize;
    while value > 0 && count < 5 {
        buf[count] = (value % 10) as u8;
        value /= 10;
        count += 1;
    }
    // Reverse the digits in-place
    buf[..count].reverse();
    (buf, count)
}

/// Render health hearts at screen position. Hearts are 5x5 with 1px spacing.
pub fn render_hearts(fb: &mut FrameBuffer, hp: i32, max_hp: i32, sx: i32, sy: i32) {
    let hp = hp.max(0);
    let max_hp = max_hp.max(0);
    for i in 0..max_hp {
        let x = sx + i * 6;
        let sprite = if i < hp { &HEART_FULL } else { &HEART_EMPTY };
        fb.blit_sprite(sprite, x, sy);
    }
}

/// Render a boss health bar centered at the top of the screen.
pub fn render_boss_bar(fb: &mut FrameBuffer, name: &str, hp: i32, max_hp: i32) {
    let fw = fb.width() as i32;
    let bar_width = 40; // pixels wide
    let bar_height = 3;
    let bar_x = (fw - bar_width) / 2;
    let bar_y = 10; // below the HUD

    // Boss name centered above the bar
    let tw = crate::sprites::font::text_width(name);
    let tx = (fw - tw) / 2;
    crate::sprites::font::render_text(fb, name, tx, bar_y - 7, [200, 180, 150]);

    // Background bar (dark)
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + bar_width) {
            fb.set_pixel_safe(x, y, [40, 10, 10]);
        }
    }

    // Foreground bar (red, proportional to HP)
    let hp_clamped = hp.max(0) as f32;
    let max_clamped = max_hp.max(1) as f32;
    let fill = ((hp_clamped / max_clamped) * bar_width as f32) as i32;
    for y in bar_y..(bar_y + bar_height) {
        for x in bar_x..(bar_x + fill) {
            fb.set_pixel_safe(x, y, [200, 30, 30]);
        }
    }

    // Bright edge on the fill bar
    if fill > 0 {
        for y in bar_y..(bar_y + bar_height) {
            fb.set_pixel_safe(bar_x + fill - 1, y, [255, 80, 80]);
        }
    }

    // Border
    let border: Color = [100, 50, 50];
    for x in (bar_x - 1)..(bar_x + bar_width + 1) {
        fb.set_pixel_safe(x, bar_y - 1, border);
        fb.set_pixel_safe(x, bar_y + bar_height, border);
    }
    for y in (bar_y - 1)..(bar_y + bar_height + 1) {
        fb.set_pixel_safe(bar_x - 1, y, border);
        fb.set_pixel_safe(bar_x + bar_width, y, border);
    }
}

/// Render the floor minimap in the top-right corner.
///
/// Each room is drawn as a small colored rectangle. Connections are shown as
/// single-pixel lines between room centers. Only discovered rooms are shown;
/// rooms adjacent to discovered rooms show as dim outlines.
pub fn render_minimap(
    fb: &mut FrameBuffer,
    floor: &FloorLayout,
    current_room: usize,
    visible: bool,
) {
    if !visible {
        return;
    }

    // Pixel dimensions for each grid cell on the minimap
    const CELL_W: i32 = 6;
    const CELL_H: i32 = 4;
    // Room rect size within each cell
    const ROOM_W: i32 = 5;
    const ROOM_H: i32 = 3;
    // Margin from screen edge
    const MARGIN: i32 = 3;

    // Find bounding box of discovered rooms (and their neighbors)
    let mut min_gx = i32::MAX;
    let mut max_gx = i32::MIN;
    let mut min_gy = i32::MAX;
    let mut max_gy = i32::MIN;

    // Build adjacency for neighbor detection
    let num_rooms = floor.rooms.len();
    let mut adjacent: Vec<Vec<usize>> = vec![Vec::new(); num_rooms];
    for &(a, b) in &floor.connections {
        adjacent[a].push(b);
        adjacent[b].push(a);
    }

    // Determine which rooms are visible on the minimap
    let mut room_visible = vec![false; num_rooms];
    let mut room_dim = vec![false; num_rooms]; // dim outline for undiscovered neighbors

    for (i, room) in floor.rooms.iter().enumerate() {
        if room.discovered {
            room_visible[i] = true;
            // Also show neighbors as dim outlines
            for &neighbor in &adjacent[i] {
                if !floor.rooms[neighbor].discovered {
                    room_visible[neighbor] = true;
                    room_dim[neighbor] = true;
                }
            }
        }
    }

    // Compute bounding box of visible rooms
    for (i, room) in floor.rooms.iter().enumerate() {
        if room_visible[i] {
            min_gx = min_gx.min(room.grid_x);
            max_gx = max_gx.max(room.grid_x);
            min_gy = min_gy.min(room.grid_y);
            max_gy = max_gy.max(room.grid_y);
        }
    }

    if min_gx > max_gx {
        return; // No visible rooms
    }

    let grid_w = max_gx - min_gx + 1;
    let grid_h = max_gy - min_gy + 1;
    let map_pixel_w = grid_w * CELL_W;
    let map_pixel_h = grid_h * CELL_H;

    // Position in top-right corner
    let base_x = fb.width() as i32 - map_pixel_w - MARGIN;
    let base_y = MARGIN;

    // Draw dark background for the minimap area
    for y in (base_y - 1)..(base_y + map_pixel_h + 1) {
        for x in (base_x - 1)..(base_x + map_pixel_w + 1) {
            fb.set_pixel_safe(x, y, [15, 15, 20]);
        }
    }

    // Draw connections first (behind rooms)
    for &(a, b) in &floor.connections {
        if !room_visible[a] || !room_visible[b] {
            continue;
        }
        let ra = &floor.rooms[a];
        let rb = &floor.rooms[b];
        let ax = base_x + (ra.grid_x - min_gx) * CELL_W + ROOM_W / 2;
        let ay = base_y + (ra.grid_y - min_gy) * CELL_H + ROOM_H / 2;
        let bx = base_x + (rb.grid_x - min_gx) * CELL_W + ROOM_W / 2;
        let by = base_y + (rb.grid_y - min_gy) * CELL_H + ROOM_H / 2;

        let conn_color: Color = if room_dim[a] || room_dim[b] {
            [30, 30, 35]
        } else {
            [60, 60, 70]
        };

        // Simple line drawing (only horizontal/vertical since grid is axis-aligned)
        if ax == bx {
            let y0 = ay.min(by);
            let y1 = ay.max(by);
            for y in y0..=y1 {
                fb.set_pixel_safe(ax, y, conn_color);
            }
        } else {
            let x0 = ax.min(bx);
            let x1 = ax.max(bx);
            for x in x0..=x1 {
                fb.set_pixel_safe(x, ay, conn_color);
            }
        }
    }

    // Draw rooms
    for (i, room) in floor.rooms.iter().enumerate() {
        if !room_visible[i] {
            continue;
        }

        let rx = base_x + (room.grid_x - min_gx) * CELL_W;
        let ry = base_y + (room.grid_y - min_gy) * CELL_H;

        if room_dim[i] {
            // Undiscovered neighbor: dim outline only
            let c: Color = [40, 40, 45];
            for x in rx..(rx + ROOM_W) {
                fb.set_pixel_safe(x, ry, c);
                fb.set_pixel_safe(x, ry + ROOM_H - 1, c);
            }
            for y in ry..(ry + ROOM_H) {
                fb.set_pixel_safe(rx, y, c);
                fb.set_pixel_safe(rx + ROOM_W - 1, y, c);
            }
        } else {
            // Discovered room: filled with type color
            let fill = room_color(room.room_type, room.cleared);
            for y in ry..(ry + ROOM_H) {
                for x in rx..(rx + ROOM_W) {
                    fb.set_pixel_safe(x, y, fill);
                }
            }

            // Current room: bright white border
            if i == current_room {
                let border: Color = [255, 255, 255];
                for x in (rx - 1)..(rx + ROOM_W + 1) {
                    fb.set_pixel_safe(x, ry - 1, border);
                    fb.set_pixel_safe(x, ry + ROOM_H, border);
                }
                for y in (ry - 1)..(ry + ROOM_H + 1) {
                    fb.set_pixel_safe(rx - 1, y, border);
                    fb.set_pixel_safe(rx + ROOM_W, y, border);
                }
            }
        }
    }
}

/// Return the minimap color for a room type.
fn room_color(room_type: RoomType, cleared: bool) -> Color {
    match room_type {
        RoomType::Start => [200, 200, 200],
        RoomType::Combat => {
            if cleared {
                [140, 140, 140]
            } else {
                [100, 100, 100]
            }
        }
        RoomType::Treasure => [200, 180, 50],
        RoomType::Shop => [50, 180, 50],
        RoomType::Boss => [200, 50, 50],
        RoomType::Exit => [50, 100, 200],
        RoomType::Corridor => {
            if cleared {
                [90, 90, 90]
            } else {
                [70, 70, 70]
            }
        }
    }
}
