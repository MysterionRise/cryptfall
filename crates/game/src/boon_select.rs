use engine::{Color, FrameBuffer, GameKey, InputState};

use crate::boons::{boon_def, BoonId, Rarity};
use crate::sprites::boon_icons;
use crate::sprites::font::{render_text, text_width};

/// Rarity border colors.
fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common => [200, 200, 200],
        Rarity::Rare => [80, 120, 255],
        Rarity::Legendary => [255, 200, 50],
    }
}

/// Rarity label text.
fn rarity_label(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "COMMON",
        Rarity::Rare => "RARE",
        Rarity::Legendary => "LEGEND",
    }
}

/// Get the boon icon sprite for a given BoonId.
fn boon_icon(id: BoonId) -> &'static engine::SpriteData {
    match id {
        BoonId::SharpenedBlade => &boon_icons::ICON_SHARPENED_BLADE,
        BoonId::BerserkersRage => &boon_icons::ICON_BERSERKERS_RAGE,
        BoonId::SwiftStrikes => &boon_icons::ICON_SWIFT_STRIKES,
        BoonId::KillingBlow => &boon_icons::ICON_KILLING_BLOW,
        BoonId::ChainLightning => &boon_icons::ICON_CHAIN_LIGHTNING,
        BoonId::ProjectileSlash => &boon_icons::ICON_PROJECTILE_SLASH,
        BoonId::CriticalEdge => &boon_icons::ICON_CRITICAL_EDGE,
        BoonId::Fury => &boon_icons::ICON_FURY,
        BoonId::ToughSkin => &boon_icons::ICON_TOUGH_SKIN,
        BoonId::IronShield => &boon_icons::ICON_IRON_SHIELD,
        BoonId::LifeSteal => &boon_icons::ICON_LIFE_STEAL,
        BoonId::VampiricTouch => &boon_icons::ICON_VAMPIRIC_TOUCH,
        BoonId::Retaliation => &boon_icons::ICON_RETALIATION,
        BoonId::SecondWind => &boon_icons::ICON_SECOND_WIND,
        BoonId::SwiftFeet => &boon_icons::ICON_SWIFT_FEET,
        BoonId::PhantomDash => &boon_icons::ICON_PHANTOM_DASH,
        BoonId::ShadowStep => &boon_icons::ICON_SHADOW_STEP,
        BoonId::DashStrike => &boon_icons::ICON_DASH_STRIKE,
        BoonId::GoldMagnet => &boon_icons::ICON_GOLD_MAGNET,
        BoonId::Lucky => &boon_icons::ICON_LUCKY,
        BoonId::TreasureSense => &boon_icons::ICON_TREASURE_SENSE,
        BoonId::DeathsBargain => &boon_icons::ICON_DEATHS_BARGAIN,
    }
}

const CARD_W: i32 = 24;
const CARD_H: i32 = 26;
const CARD_GAP: i32 = 4;

/// Duration of the selection flash animation in seconds.
const FLASH_DURATION: f32 = 0.4;

pub struct BoonSelectScreen {
    pub options: Vec<BoonId>,
    pub selected: usize,
    pub active: bool,
    /// Set to Some(index) when a boon is confirmed, triggers flash before closing.
    confirmed: Option<usize>,
    flash_timer: f32,
}

impl BoonSelectScreen {
    pub fn new(options: Vec<BoonId>) -> Self {
        Self {
            options,
            selected: 0,
            active: true,
            confirmed: None,
            flash_timer: 0.0,
        }
    }

    /// Handle input. Returns Some(BoonId) when the selection animation completes.
    pub fn update(&mut self, input: &InputState, dt: f32) -> Option<BoonId> {
        if !self.active || self.options.is_empty() {
            return None;
        }

        // If we're playing the confirmation flash, count down
        if let Some(idx) = self.confirmed {
            self.flash_timer -= dt;
            if self.flash_timer <= 0.0 {
                self.active = false;
                return Some(self.options[idx]);
            }
            return None;
        }

        let count = self.options.len();

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

    /// Render the boon selection overlay onto the framebuffer.
    pub fn render(&self, fb: &mut FrameBuffer) {
        if !self.active || self.options.is_empty() {
            return;
        }

        let fw = fb.width() as i32;
        let fh = fb.height() as i32;

        // Darken the background
        fb.overlay([0, 0, 0], 0.6);

        // Selection flash: brief white flash on confirm
        if self.confirmed.is_some() && self.flash_timer > FLASH_DURATION * 0.7 {
            let intensity = ((self.flash_timer - FLASH_DURATION * 0.7) / (FLASH_DURATION * 0.3)).min(1.0);
            fb.overlay([255, 255, 255], intensity * 0.3);
        }

        // Title
        let title = "CHOOSE A BOON";
        let tw = text_width(title);
        let tx = (fw - tw) / 2;
        let title_y = fh / 2 - CARD_H / 2 - 10;
        render_text(fb, title, tx, title_y, [255, 220, 100]);

        let count = self.options.len() as i32;
        let total_w = count * CARD_W + (count - 1) * CARD_GAP;
        let start_x = (fw - total_w) / 2;
        let start_y = (fh - CARD_H) / 2;

        for (i, &boon_id) in self.options.iter().enumerate() {
            let is_selected = i == self.selected;
            let card_x = start_x + i as i32 * (CARD_W + CARD_GAP);
            let card_y = start_y;

            // During flash, highlight the confirmed card
            let is_confirmed = self.confirmed == Some(i);
            self.render_card(fb, boon_id, card_x, card_y, is_selected, is_confirmed);
        }

        // Navigation hint (hide during confirmation flash)
        if self.confirmed.is_none() {
            let hint = "LEFT/RIGHT - ATTACK SELECT";
            let hw = text_width(hint);
            let hx = (fw - hw) / 2;
            let hy = start_y + CARD_H + 4;
            render_text(fb, hint, hx, hy, [120, 120, 120]);
        }
    }

    fn render_card(
        &self,
        fb: &mut FrameBuffer,
        boon_id: BoonId,
        x: i32,
        y: i32,
        selected: bool,
        confirmed: bool,
    ) {
        let def = boon_def(boon_id);
        let border_color = rarity_color(def.rarity);

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

        // Top and bottom edges
        for px in x..(x + CARD_W) {
            fb.set_pixel_safe(px, y, bc);
            fb.set_pixel_safe(px, y + CARD_H - 1, bc);
        }
        // Left and right edges
        for py in y..(y + CARD_H) {
            fb.set_pixel_safe(x, py, bc);
            fb.set_pixel_safe(x + CARD_W - 1, py, bc);
        }

        // Icon centered near top of card (8x8 icon)
        let icon = boon_icon(boon_id);
        let icon_x = x + (CARD_W - icon.width as i32) / 2;
        let icon_y = y + 2;
        fb.blit_sprite(icon, icon_x, icon_y);

        // Name below icon
        let name_color: Color = if selected || confirmed {
            [255, 255, 255]
        } else {
            [160, 160, 160]
        };
        let nw = text_width(def.name);
        let nx = x + (CARD_W - nw) / 2;
        let ny = y + 12;
        render_text(fb, def.name, nx, ny, name_color);

        // Rarity label
        let rarity_label_text = rarity_label(def.rarity);
        let rlw = text_width(rarity_label_text);
        let rlx = x + (CARD_W - rlw) / 2;
        let rly = y + 18;
        let rl_color = if selected || confirmed {
            rarity_color(def.rarity)
        } else {
            let rc = rarity_color(def.rarity);
            [rc[0] / 2, rc[1] / 2, rc[2] / 2]
        };
        render_text(fb, rarity_label_text, rlx, rly, rl_color);
    }
}
