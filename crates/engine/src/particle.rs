use crate::color::Color;
use crate::framebuffer::FrameBuffer;

/// A single particle with physics and lifetime.
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub gravity: f32,
    pub friction: f32,
}

impl Particle {
    pub fn alive(&self) -> bool {
        self.lifetime < self.max_lifetime
    }

    /// Returns 0.0 (just born) to 1.0 (about to die).
    pub fn age_ratio(&self) -> f32 {
        self.lifetime / self.max_lifetime
    }
}

/// Configuration for spawning a burst of particles.
pub struct BurstConfig {
    pub count_min: u32,
    pub count_max: u32,
    pub speed_min: f32,
    pub speed_max: f32,
    pub lifetime_min: f32,
    pub lifetime_max: f32,
    pub colors: &'static [Color],
    pub gravity: f32,
    pub friction: f32,
    /// Angle spread in radians. PI = half circle, 2*PI = full circle.
    pub angle_spread: f32,
    /// Base angle in radians. 0 = right, PI/2 = down, PI = left, etc.
    pub base_angle: f32,
}

/// Manages a pool of particles with update and render.
pub struct ParticleSystem {
    particles: Vec<Particle>,
    rng_state: u32,
}

const MAX_PARTICLES: usize = 500;

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(256),
            rng_state: 98765,
        }
    }

    /// Spawn a burst of particles at the given world position.
    pub fn burst(&mut self, x: f32, y: f32, config: &BurstConfig) {
        let count = self.rand_range(config.count_min, config.count_max);
        for _ in 0..count {
            if self.particles.len() >= MAX_PARTICLES {
                break;
            }

            // Random angle within spread
            let angle_offset = self.rand_float() * config.angle_spread - config.angle_spread * 0.5;
            let angle = config.base_angle + angle_offset;

            // Random speed
            let speed =
                config.speed_min + self.rand_float() * (config.speed_max - config.speed_min);

            // Random lifetime
            let lifetime_max = config.lifetime_min
                + self.rand_float() * (config.lifetime_max - config.lifetime_min);

            // Random color from palette
            let color_idx = self.rand_u32() as usize % config.colors.len().max(1);
            let color = config.colors[color_idx.min(config.colors.len().saturating_sub(1))];

            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                color,
                lifetime: 0.0,
                max_lifetime: lifetime_max,
                gravity: config.gravity,
                friction: config.friction,
            });
        }
    }

    /// Update all particles: apply physics, age, and remove dead ones.
    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.vx *= p.friction.powf(dt * 30.0);
            p.vy *= p.friction.powf(dt * 30.0);
            p.vy += p.gravity * dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.lifetime += dt;
        }
        self.particles.retain(|p| p.alive());
    }

    /// Render all particles to the framebuffer.
    /// Uses lifetime-based fade: particles get dimmer as they age.
    pub fn render(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        for p in &self.particles {
            let sx = p.x as i32 - cam_x;
            let sy = p.y as i32 - cam_y;

            // Fade based on age
            let fade = 1.0 - p.age_ratio();
            let c = [
                (p.color[0] as f32 * fade) as u8,
                (p.color[1] as f32 * fade) as u8,
                (p.color[2] as f32 * fade) as u8,
            ];

            fb.set_pixel_safe(sx, sy, c);
        }
    }

    /// Number of active particles.
    pub fn count(&self) -> usize {
        self.particles.len()
    }

    /// Clear all particles.
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    // --- Internal PRNG (same LCG pattern as Camera) ---

    fn rand_u32(&mut self) -> u32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        self.rng_state >> 16
    }

    /// Returns a float in [0.0, 1.0).
    fn rand_float(&mut self) -> f32 {
        self.rand_u32() as f32 / 65536.0
    }

    /// Returns a random u32 in [min, max] inclusive.
    fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        min + self.rand_u32() % (max - min + 1)
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}
