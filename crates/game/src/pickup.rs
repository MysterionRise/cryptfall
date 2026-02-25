//! Pickup items that drop after combat encounters.

use engine::FrameBuffer;

use crate::sprites::pickups;

/// Type of pickup item.
pub enum PickupType {
    SmallHeal,
    #[allow(dead_code)] // Phase 4: treasure rooms and boss drops
    BigHeal,
    #[allow(dead_code)] // Phase 4: currency system
    Gold(u32),
}

/// A collectible item in the world.
pub struct Pickup {
    pub x: f32,
    pub y: f32,
    pub pickup_type: PickupType,
    pub alive: bool,
    bob_timer: f32,
}

const BOB_SPEED: f32 = 3.0;
const BOB_AMPLITUDE: f32 = 2.0;
const COLLECT_RADIUS: f32 = 8.0;

impl Pickup {
    pub fn new(x: f32, y: f32, pickup_type: PickupType) -> Self {
        Self {
            x,
            y,
            pickup_type,
            alive: true,
            bob_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.alive {
            return;
        }
        self.bob_timer += dt * BOB_SPEED;
    }

    pub fn render(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        if !self.alive {
            return;
        }

        let bob_offset = (self.bob_timer.sin() * BOB_AMPLITUDE) as i32;
        let sprite = match self.pickup_type {
            PickupType::SmallHeal => &pickups::PICKUP_HEART_SMALL,
            PickupType::BigHeal => &pickups::PICKUP_HEART_BIG,
            PickupType::Gold(_) => &pickups::PICKUP_COIN,
        };

        let px = self.x as i32 - cam_x;
        let py = self.y as i32 - cam_y + bob_offset;
        fb.blit_sprite(sprite, px, py);
    }

    /// Check if the player overlaps this pickup. Uses a simple distance check.
    pub fn check_collection(&self, player_x: f32, player_y: f32, player_w: f32, player_h: f32) -> bool {
        if !self.alive {
            return false;
        }

        let sprite = match self.pickup_type {
            PickupType::SmallHeal => &pickups::PICKUP_HEART_SMALL,
            PickupType::BigHeal => &pickups::PICKUP_HEART_BIG,
            PickupType::Gold(_) => &pickups::PICKUP_COIN,
        };

        let cx = self.x + sprite.width as f32 / 2.0;
        let cy = self.y + sprite.height as f32 / 2.0;
        let pcx = player_x + player_w / 2.0;
        let pcy = player_y + player_h / 2.0;

        let dx = cx - pcx;
        let dy = cy - pcy;
        (dx * dx + dy * dy) < COLLECT_RADIUS * COLLECT_RADIUS
    }

    /// Returns the healing amount for this pickup (0 for non-heal pickups).
    pub fn heal_amount(&self) -> i32 {
        match self.pickup_type {
            PickupType::SmallHeal => 1,
            PickupType::BigHeal => 3,
            PickupType::Gold(_) => 0,
        }
    }
}
