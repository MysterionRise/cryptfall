use engine::collision::AABB;
use engine::color::Color;
use engine::tilemap::TileMap;
use engine::FrameBuffer;

use crate::sprites::effects::PROJECTILE_ORB;

const PROJECTILE_SPEED: f32 = 80.0;
const PROJECTILE_LIFETIME: f32 = 2.0;
const PROJECTILE_HITBOX: AABB = AABB::new(0.0, 0.0, 3.0, 3.0);
const TRAIL_INTERVAL: f32 = 0.05;

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    vx: f32,
    vy: f32,
    pub lifetime: f32,
    pub alive: bool,
    trail_timer: f32,
    pub damage: i32,
}

impl Projectile {
    pub fn new(x: f32, y: f32, dir_x: f32, dir_y: f32) -> Self {
        Self {
            x,
            y,
            vx: dir_x * PROJECTILE_SPEED,
            vy: dir_y * PROJECTILE_SPEED,
            lifetime: 0.0,
            alive: true,
            trail_timer: 0.0,
            damage: 1,
        }
    }

    pub fn world_hitbox(&self) -> AABB {
        PROJECTILE_HITBOX.at(self.x, self.y)
    }

    /// Update position. Returns true if a trail particle should be spawned.
    pub fn update(&mut self, dt: f32, tilemap: &TileMap) -> bool {
        if !self.alive {
            return false;
        }

        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.lifetime += dt;

        // Check wall collision
        if tilemap.collides(self.x, self.y, 3.0, 3.0) {
            self.alive = false;
            return false;
        }

        // Check lifetime
        if self.lifetime >= PROJECTILE_LIFETIME {
            self.alive = false;
            return false;
        }

        // Trail particle timing
        self.trail_timer += dt;
        if self.trail_timer >= TRAIL_INTERVAL {
            self.trail_timer -= TRAIL_INTERVAL;
            return true;
        }

        false
    }

    pub fn render(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        if !self.alive {
            return;
        }
        let px = self.x as i32 - cam_x;
        let py = self.y as i32 - cam_y;
        fb.blit_sprite(&PROJECTILE_ORB, px, py);
    }
}

pub struct ProjectileSystem {
    pub projectiles: Vec<Projectile>,
}

impl ProjectileSystem {
    pub fn new() -> Self {
        Self {
            projectiles: Vec::with_capacity(32),
        }
    }

    pub fn spawn(&mut self, x: f32, y: f32, dir_x: f32, dir_y: f32) {
        self.projectiles.push(Projectile::new(x, y, dir_x, dir_y));
    }

    /// Update all projectiles. Returns positions where trail particles should spawn
    /// and positions where impact particles should spawn (wall hits).
    pub fn update(
        &mut self,
        dt: f32,
        tilemap: &TileMap,
    ) -> (Vec<(f32, f32)>, Vec<(f32, f32)>) {
        let mut trail_positions = Vec::new();
        let mut impact_positions = Vec::new();

        for proj in &mut self.projectiles {
            let was_alive = proj.alive;
            let needs_trail = proj.update(dt, tilemap);

            if needs_trail {
                trail_positions.push((proj.x + 1.5, proj.y + 1.5));
            }

            if was_alive && !proj.alive {
                impact_positions.push((proj.x + 1.5, proj.y + 1.5));
            }
        }

        self.projectiles.retain(|p| p.alive);

        (trail_positions, impact_positions)
    }

    /// Check projectiles against the player hurtbox. Returns total damage and
    /// positions of hits for particle effects.
    pub fn check_player_hits(&mut self, player_hurtbox: &AABB) -> Vec<(f32, f32, i32)> {
        let mut hits = Vec::new();
        for proj in &mut self.projectiles {
            if !proj.alive {
                continue;
            }
            if proj.world_hitbox().overlaps(player_hurtbox) {
                hits.push((proj.x + 1.5, proj.y + 1.5, proj.damage));
                proj.alive = false;
            }
        }
        hits
    }

    pub fn render(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        for proj in &self.projectiles {
            proj.render(fb, cam_x, cam_y);
        }
    }

    pub fn clear(&mut self) {
        self.projectiles.clear();
    }

    pub fn count(&self) -> usize {
        self.projectiles.len()
    }
}
