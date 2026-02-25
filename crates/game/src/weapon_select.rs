use engine::{Color, FrameBuffer, GameKey, InputState};

use crate::sprites::font::{render_text, text_width};
use crate::weapons::{all_weapons, WeaponId};

/// Border color per weapon type.
fn weapon_color(id: WeaponId) -> Color {
    match id {
        WeaponId::Sword => [180, 180, 200],
        WeaponId::Spear => [160, 120, 80],
        WeaponId::Daggers => [80, 180, 80],
    }
}

const CARD_W: i32 = 28;
const CARD_H: i32 = 28;
const CARD_GAP: i32 = 4;

/// Duration of the selection flash animation in seconds.
const FLASH_DURATION: f32 = 0.4;

pub struct WeaponSelectScreen {
    pub selected: usize,
    pub active: bool,
    confirmed: Option<usize>,
    flash_timer: f32,
}

impl WeaponSelectScreen {
    pub fn new() -> Self {
        Self {
            selected: 0,
            active: true,
            confirmed: None,
            flash_timer: 0.0,
        }
    }

    /// Handle input. Returns Some(WeaponId) when selection animation completes.
    pub fn update(&mut self, input: &InputState, dt: f32) -> Option<WeaponId> {
        if !self.active {
            return None;
        }

        // Confirmation flash countdown
        if let Some(idx) = self.confirmed {
            self.flash_timer -= dt;
            if self.flash_timer <= 0.0 {
                self.active = false;
                let weapons = all_weapons();
                return Some(weapons[idx].id);
            }
            return None;
        }

        let weapons = all_weapons();
        let count = weapons.len();

        if input.is_pressed(GameKey::Left) && self.selected > 0 {
            self.selected -= 1;
        }
        if input.is_pressed(GameKey::Right) && self.selected + 1 < count {
            self.selected += 1;
        }

        if input.is_pressed(GameKey::Attack) {
            self.confirmed = Some(self.selected);
            self.flash_timer = FLASH_DURATION;
        }

        None
    }

    /// Render the weapon selection screen.
    pub fn render(&self, fb: &mut FrameBuffer) {
        if !self.active {
            return;
        }

        let fw = fb.width() as i32;
        let fh = fb.height() as i32;

        // Dark background (full screen, not overlay -- this is a pre-game screen)
        for y in 0..fh {
            for x in 0..fw {
                fb.set_pixel_safe(x, y, [10, 10, 15]);
            }
        }

        // Selection flash
        if self.confirmed.is_some() && self.flash_timer > FLASH_DURATION * 0.7 {
            let intensity = ((self.flash_timer - FLASH_DURATION * 0.7) / (FLASH_DURATION * 0.3)).min(1.0);
            fb.overlay([255, 255, 255], intensity * 0.3);
        }

        // Title
        let title = "CHOOSE YOUR WEAPON";
        let tw = text_width(title);
        let tx = (fw - tw) / 2;
        let title_y = fh / 2 - CARD_H / 2 - 10;
        render_text(fb, title, tx, title_y, [255, 220, 100]);

        let weapons = all_weapons();
        let count = weapons.len() as i32;
        let total_w = count * CARD_W + (count - 1) * CARD_GAP;
        let start_x = (fw - total_w) / 2;
        let start_y = (fh - CARD_H) / 2;

        for (i, weapon) in weapons.iter().enumerate() {
            let is_selected = i == self.selected;
            let is_confirmed = self.confirmed == Some(i);
            let card_x = start_x + i as i32 * (CARD_W + CARD_GAP);
            let card_y = start_y;

            self.render_weapon_card(fb, weapon, card_x, card_y, is_selected, is_confirmed);
        }

        // Navigation hint
        if self.confirmed.is_none() {
            let hint = "LEFT/RIGHT - ATTACK SELECT";
            let hw = text_width(hint);
            let hx = (fw - hw) / 2;
            let hy = start_y + CARD_H + 4;
            render_text(fb, hint, hx, hy, [120, 120, 120]);
        }
    }

    fn render_weapon_card(
        &self,
        fb: &mut FrameBuffer,
        weapon: &crate::weapons::WeaponDef,
        x: i32,
        y: i32,
        selected: bool,
        confirmed: bool,
    ) {
        let border_color = weapon_color(weapon.id);

        let bg: Color = if confirmed {
            [50, 50, 80]
        } else if selected {
            [30, 30, 50]
        } else {
            [20, 20, 30]
        };

        // Fill background
        for py in (y + 1)..(y + CARD_H - 1) {
            for px in (x + 1)..(x + CARD_W - 1) {
                fb.set_pixel_safe(px, py, bg);
            }
        }

        // Border
        let bc: Color = if confirmed {
            [255, 255, 255]
        } else if selected {
            [
                border_color[0].saturating_add(40),
                border_color[1].saturating_add(40),
                border_color[2].saturating_add(40),
            ]
        } else {
            [
                border_color[0] / 2,
                border_color[1] / 2,
                border_color[2] / 2,
            ]
        };

        for px in x..(x + CARD_W) {
            fb.set_pixel_safe(px, y, bc);
            fb.set_pixel_safe(px, y + CARD_H - 1, bc);
        }
        for py in y..(y + CARD_H) {
            fb.set_pixel_safe(x, py, bc);
            fb.set_pixel_safe(x + CARD_W - 1, py, bc);
        }

        // Weapon name at top, centered
        let name_color: Color = if selected || confirmed {
            [255, 255, 255]
        } else {
            [160, 160, 160]
        };
        let nw = text_width(weapon.name);
        let nx = x + (CARD_W - nw) / 2;
        render_text(fb, weapon.name, nx, y + 2, name_color);

        // Stats
        let stat_color: Color = if selected || confirmed {
            [180, 180, 200]
        } else {
            [120, 120, 130]
        };

        let dmg_str = format!("DMG {}", weapon.base_damage);
        render_text(fb, &dmg_str, x + 2, y + 9, stat_color);

        let spd_str = format!("SPD {}", weapon.speed_label);
        render_text(fb, &spd_str, x + 2, y + 15, stat_color);

        let rng_str = format!("RNG {}", weapon.range_label);
        render_text(fb, &rng_str, x + 2, y + 21, stat_color);
    }
}
