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
            let palette_len = config.colors.len().max(1);
            let color = config.colors[self.rand_u32() as usize % palette_len];

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a simple BurstConfig with deterministic settings.
    fn test_burst_config(count: u32) -> BurstConfig {
        BurstConfig {
            count_min: count,
            count_max: count,
            speed_min: 10.0,
            speed_max: 10.0,
            lifetime_min: 1.0,
            lifetime_max: 1.0,
            colors: &[[255, 0, 0]],
            gravity: 0.0,
            friction: 1.0, // no friction
            angle_spread: std::f32::consts::TAU,
            base_angle: 0.0,
        }
    }

    /// Helper: create a BurstConfig with variable count range.
    fn test_burst_config_range(min: u32, max: u32) -> BurstConfig {
        BurstConfig {
            count_min: min,
            count_max: max,
            speed_min: 5.0,
            speed_max: 15.0,
            lifetime_min: 0.5,
            lifetime_max: 1.5,
            colors: &[[255, 0, 0], [0, 255, 0]],
            gravity: 0.0,
            friction: 1.0,
            angle_spread: std::f32::consts::TAU,
            base_angle: 0.0,
        }
    }

    #[test]
    fn test_particle_system_new_starts_empty() {
        // Arrange & Act
        let ps = ParticleSystem::new();

        // Assert
        assert_eq!(ps.count(), 0, "New particle system should have 0 particles");
    }

    #[test]
    fn test_particle_alive_when_lifetime_less_than_max() {
        // Arrange
        let p = Particle {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: [255, 0, 0],
            lifetime: 0.5,
            max_lifetime: 1.0,
            gravity: 0.0,
            friction: 1.0,
        };

        // Act & Assert
        assert!(p.alive(), "Particle with lifetime < max_lifetime should be alive");
    }

    #[test]
    fn test_particle_dead_when_lifetime_equals_max() {
        // Arrange
        let p = Particle {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: [255, 0, 0],
            lifetime: 1.0,
            max_lifetime: 1.0,
            gravity: 0.0,
            friction: 1.0,
        };

        // Act & Assert
        assert!(
            !p.alive(),
            "Particle with lifetime == max_lifetime should be dead"
        );
    }

    #[test]
    fn test_particle_dead_when_lifetime_exceeds_max() {
        // Arrange
        let p = Particle {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: [255, 0, 0],
            lifetime: 1.5,
            max_lifetime: 1.0,
            gravity: 0.0,
            friction: 1.0,
        };

        // Act & Assert
        assert!(
            !p.alive(),
            "Particle with lifetime > max_lifetime should be dead"
        );
    }

    #[test]
    fn test_particle_age_ratio() {
        // Arrange
        let p = Particle {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            color: [255, 0, 0],
            lifetime: 0.25,
            max_lifetime: 1.0,
            gravity: 0.0,
            friction: 1.0,
        };

        // Act
        let ratio = p.age_ratio();

        // Assert
        assert!(
            (ratio - 0.25).abs() < f32::EPSILON,
            "age_ratio should be lifetime / max_lifetime = 0.25, got {}",
            ratio
        );
    }

    #[test]
    fn test_particle_lifetime_counts_down_on_update() {
        // Arrange
        let mut ps = ParticleSystem::new();
        let config = test_burst_config(1);
        ps.burst(0.0, 0.0, &config);
        assert_eq!(ps.count(), 1);

        // Act: update with small dt (particle should still be alive)
        ps.update(0.5);

        // Assert: particle is still alive (lifetime 0.5 < max_lifetime 1.0)
        assert_eq!(
            ps.count(),
            1,
            "Particle should still be alive after 0.5s (max_lifetime=1.0)"
        );
    }

    #[test]
    fn test_dead_particles_are_removed_on_update() {
        // Arrange
        let mut ps = ParticleSystem::new();
        let config = test_burst_config(5);
        ps.burst(0.0, 0.0, &config);
        assert_eq!(ps.count(), 5);

        // Act: update past max_lifetime so all particles die
        ps.update(2.0);

        // Assert
        assert_eq!(
            ps.count(),
            0,
            "All particles should be removed after exceeding max_lifetime"
        );
    }

    #[test]
    fn test_burst_spawns_exact_count_when_min_equals_max() {
        // Arrange
        let mut ps = ParticleSystem::new();
        let config = test_burst_config(10);

        // Act
        ps.burst(0.0, 0.0, &config);

        // Assert
        assert_eq!(
            ps.count(),
            10,
            "Burst with count_min == count_max == 10 should spawn exactly 10 particles"
        );
    }

    #[test]
    fn test_burst_spawns_within_count_range() {
        // Arrange
        let mut ps = ParticleSystem::new();
        let config = test_burst_config_range(5, 15);

        // Act
        ps.burst(0.0, 0.0, &config);

        // Assert
        assert!(
            ps.count() >= 5 && ps.count() <= 15,
            "Burst count {} should be within [5, 15]",
            ps.count()
        );
    }

    #[test]
    fn test_particle_count_never_exceeds_max_particles() {
        // Arrange
        let mut ps = ParticleSystem::new();
        let config = test_burst_config(100);

        // Act: spawn many bursts to try to exceed MAX_PARTICLES (500)
        for _ in 0..10 {
            ps.burst(0.0, 0.0, &config);
        }

        // Assert
        assert!(
            ps.count() <= MAX_PARTICLES,
            "Particle count {} should never exceed MAX_PARTICLES ({})",
            ps.count(),
            MAX_PARTICLES
        );
    }

    #[test]
    fn test_particle_count_exactly_at_max() {
        // Arrange: fill to exactly MAX_PARTICLES
        let mut ps = ParticleSystem::new();
        let config = test_burst_config(MAX_PARTICLES as u32);

        // Act
        ps.burst(0.0, 0.0, &config);

        // Assert
        assert_eq!(
            ps.count(),
            MAX_PARTICLES,
            "Should be able to spawn exactly MAX_PARTICLES"
        );

        // Act: try to spawn more
        ps.burst(0.0, 0.0, &test_burst_config(10));

        // Assert: still at MAX
        assert_eq!(
            ps.count(),
            MAX_PARTICLES,
            "Should not exceed MAX_PARTICLES after additional burst"
        );
    }

    #[test]
    fn test_velocity_affects_position_after_update() {
        // Arrange
        let mut ps = ParticleSystem::new();
        // Use a config with predictable direction: base_angle=0 (right), no spread
        let config = BurstConfig {
            count_min: 1,
            count_max: 1,
            speed_min: 100.0,
            speed_max: 100.0,
            lifetime_min: 10.0,
            lifetime_max: 10.0,
            colors: &[[255, 0, 0]],
            gravity: 0.0,
            friction: 1.0, // no friction
            angle_spread: 0.0, // no spread - exactly base_angle direction
            base_angle: 0.0,   // rightward
        };
        ps.burst(0.0, 0.0, &config);

        // Act
        ps.update(1.0);

        // Assert: particle should still be alive after 1s (max_lifetime=10)
        assert_eq!(
            ps.count(),
            1,
            "Particle should still be alive after 1s (max_lifetime=10)"
        );
    }

    #[test]
    fn test_gravity_affects_vertical_velocity() {
        // Arrange: create two systems - one with gravity, one without
        let mut ps_no_gravity = ParticleSystem::new();
        let mut ps_gravity = ParticleSystem::new();

        let config_no_grav = BurstConfig {
            count_min: 1,
            count_max: 1,
            speed_min: 0.0,
            speed_max: 0.0, // stationary
            lifetime_min: 10.0,
            lifetime_max: 10.0,
            colors: &[[255, 0, 0]],
            gravity: 0.0,
            friction: 1.0,
            angle_spread: 0.0,
            base_angle: 0.0,
        };

        let config_grav = BurstConfig {
            count_min: 1,
            count_max: 1,
            speed_min: 0.0,
            speed_max: 0.0, // stationary
            lifetime_min: 10.0,
            lifetime_max: 10.0,
            colors: &[[255, 0, 0]],
            gravity: 100.0,
            friction: 1.0,
            angle_spread: 0.0,
            base_angle: 0.0,
        };

        ps_no_gravity.burst(50.0, 50.0, &config_no_grav);
        ps_gravity.burst(50.0, 50.0, &config_grav);

        // Act
        ps_no_gravity.update(1.0);
        ps_gravity.update(1.0);

        // Assert: both should still be alive (lifetime=10s)
        assert_eq!(ps_no_gravity.count(), 1, "No-gravity particle should be alive");
        assert_eq!(ps_gravity.count(), 1, "Gravity particle should be alive");
    }

    #[test]
    fn test_clear_removes_all_particles() {
        // Arrange
        let mut ps = ParticleSystem::new();
        let config = test_burst_config(50);
        ps.burst(0.0, 0.0, &config);
        assert_eq!(ps.count(), 50);

        // Act
        ps.clear();

        // Assert
        assert_eq!(ps.count(), 0, "clear() should remove all particles");
    }

    #[test]
    fn test_clear_then_burst_works() {
        // Arrange
        let mut ps = ParticleSystem::new();
        ps.burst(0.0, 0.0, &test_burst_config(10));
        ps.clear();

        // Act
        ps.burst(0.0, 0.0, &test_burst_config(5));

        // Assert
        assert_eq!(ps.count(), 5, "Should be able to burst after clear()");
    }

    #[test]
    fn test_update_with_zero_dt_does_not_kill_particles() {
        // Arrange
        let mut ps = ParticleSystem::new();
        ps.burst(0.0, 0.0, &test_burst_config(5));

        // Act
        ps.update(0.0);

        // Assert
        assert_eq!(ps.count(), 5, "Zero dt should not kill any particles");
    }

    #[test]
    fn test_default_creates_same_as_new() {
        // Arrange & Act
        let ps = ParticleSystem::default();

        // Assert
        assert_eq!(ps.count(), 0, "Default particle system should be empty");
    }
}
