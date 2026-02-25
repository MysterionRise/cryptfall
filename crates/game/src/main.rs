mod combat;
mod dungeon;
mod enemies;
mod hud;
mod pickup;
mod player;
mod projectile;
mod sprites;
mod tiles;
mod tuning;


use dungeon::encounters::{
    self, EncounterDifficulty, WaveTracker,
};
use dungeon::world::{self, DungeonWorld, TransitionEvent};
use engine::{
    color, render_tilemap, BurstConfig, Camera, Color, FrameBuffer, FrameInfo, Game, GameKey,
    InputState, ParticleSystem, TileMap,
};
use pickup::Pickup;
use player::Player;
use tuning::*;

// --- Particle burst configurations for player movement effects ---

const DASH_TRAIL_COLORS: &[Color] = &[[100, 160, 255], [150, 200, 255], [200, 230, 255]];

const DASH_TRAIL_CONFIG: BurstConfig = BurstConfig {
    count_min: 2,
    count_max: 3,
    speed_min: 5.0,
    speed_max: 15.0,
    lifetime_min: 0.1,
    lifetime_max: 0.25,
    colors: DASH_TRAIL_COLORS,
    gravity: 0.0,
    friction: 0.8,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const DUST_PUFF_COLORS: &[Color] = &[[120, 100, 70], [140, 120, 90], [100, 80, 60]];

const DUST_PUFF_CONFIG: BurstConfig = BurstConfig {
    count_min: 4,
    count_max: 6,
    speed_min: 10.0,
    speed_max: 25.0,
    lifetime_min: 0.2,
    lifetime_max: 0.4,
    colors: DUST_PUFF_COLORS,
    gravity: -10.0,
    friction: 0.85,
    angle_spread: std::f32::consts::PI,
    base_angle: -std::f32::consts::FRAC_PI_2,
};

const PICKUP_COLLECT_COLORS: &[Color] = &[[255, 100, 100], [255, 200, 200], [255, 150, 150]];

const PICKUP_COLLECT_CONFIG: BurstConfig = BurstConfig {
    count_min: 6,
    count_max: 10,
    speed_min: 15.0,
    speed_max: 40.0,
    lifetime_min: 0.2,
    lifetime_max: 0.4,
    colors: PICKUP_COLLECT_COLORS,
    gravity: -20.0,
    friction: 0.85,
    angle_spread: std::f32::consts::TAU,
    base_angle: 0.0,
};

const FRAC_1_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2;

// --- Demo mode auto-play ---

struct DemoState {
    timer: f32,
    dx: f32,
    dy: f32,
    attack_timer: f32,
    seed: u32,
}

impl DemoState {
    fn new() -> Self {
        Self {
            timer: 0.0,
            dx: 1.0,
            dy: 0.0,
            attack_timer: 2.0,
            seed: 54321,
        }
    }

    fn next_random(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed >> 16
    }

    fn update(&mut self, dt: f32) -> (f32, f32, bool, bool) {
        self.timer -= dt;
        self.attack_timer -= dt;

        let attack = if self.attack_timer <= 0.0 {
            self.attack_timer = 1.5 + (self.next_random() % 30) as f32 * 0.1;
            true
        } else {
            false
        };

        if self.timer <= 0.0 {
            self.timer = 0.8 + (self.next_random() % 20) as f32 * 0.1;
            let dir = self.next_random() % 9;
            let (dx, dy) = match dir {
                0 => (1.0, 0.0),
                1 => (-1.0, 0.0),
                2 => (0.0, 1.0),
                3 => (0.0, -1.0),
                4 => (FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                5 => (-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
                6 => (FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                7 => (-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                _ => (0.0, 0.0),
            };
            self.dx = dx;
            self.dy = dy;
        }

        let dash = self.timer > 0.0
            && self.timer < dt
            && (self.dx != 0.0 || self.dy != 0.0)
            && self.next_random().is_multiple_of(5);

        (self.dx, self.dy, attack, dash)
    }
}

// --- Death sequence phases ---

enum DeathPhase {
    Alive,
    Dying,
    FadeOut,
    Dead,
}

// --- Room combat state ---

enum RoomState {
    /// No combat (start room, cleared room, treasure, shop, exit)
    Peaceful,
    /// Enemies are alive, doors closed
    Combat,
    /// All enemies dead, doors opening
    Cleared,
}

// --- Main game state ---

struct CryptfallGame {
    player: Player,
    enemies: Vec<enemies::Enemy>,
    projectiles: projectile::ProjectileSystem,
    tilemap: TileMap,
    camera: Camera,
    particles: ParticleSystem,
    damage_numbers: Vec<hud::DamageNumber>,
    dungeon: DungeonWorld,
    room_state: RoomState,
    wave_tracker: Option<WaveTracker>,
    pickups: Vec<Pickup>,
    spawn_seed: u64,
    flash_timer: u32,
    hit_pause_frames: u32,
    idle_timer: f32,
    demo: Option<DemoState>,
    debug_hitboxes: bool,
    death_phase: DeathPhase,
    death_timer: f32,
    heart_flash_timer: f32,
    last_hp: i32,
    room_entry_invincibility: f32,
    floor_clear: bool,
    sealed_flash_timer: f32,
    paused: bool,
    minimap_visible: bool,
}

impl CryptfallGame {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(42);

        let dungeon = DungeonWorld::new(1, seed);
        let tilemap = dungeon.build_tilemap();
        let (px, py) = dungeon.player_spawn_position(None);
        let player = Player::new(px, py);

        let mut camera = Camera::new(80, 48);
        let (cx, cy) = player.center();
        camera.follow(cx, cy);
        camera.snap();
        camera.clamp_to_bounds(tilemap.pixel_width() as f32, tilemap.pixel_height() as f32);

        Self {
            player,
            enemies: Vec::new(), // Start room has no enemies
            projectiles: projectile::ProjectileSystem::new(),
            tilemap,
            camera,
            particles: ParticleSystem::new(),
            damage_numbers: Vec::new(),
            dungeon,
            room_state: RoomState::Peaceful,
            wave_tracker: None,
            pickups: Vec::new(),
            spawn_seed: seed,
            flash_timer: 0,
            hit_pause_frames: 0,
            idle_timer: 0.0,
            demo: None,
            debug_hitboxes: false,
            death_phase: DeathPhase::Alive,
            death_timer: 0.0,
            heart_flash_timer: 0.0,
            last_hp: 5,
            room_entry_invincibility: 0.0,
            floor_clear: false,
            sealed_flash_timer: 0.0,
            paused: false,
            minimap_visible: true,
        }
    }
}

fn has_input(input: &InputState) -> bool {
    input.is_held(GameKey::Up)
        || input.is_held(GameKey::Down)
        || input.is_held(GameKey::Left)
        || input.is_held(GameKey::Right)
        || input.is_pressed(GameKey::Attack)
        || input.is_pressed(GameKey::Dash)
}

impl Game for CryptfallGame {
    fn update(&mut self, input: &InputState, dt: f64) -> bool {
        // Minimap toggle (Tab) works anytime
        if input.is_pressed(GameKey::Map) {
            self.minimap_visible = !self.minimap_visible;
        }

        // Pause toggle (Esc) — while paused, Q quits
        if self.paused {
            if input.is_pressed(GameKey::Pause) {
                self.paused = false;
            }
            if input.is_pressed(GameKey::Quit) {
                return false;
            }
            return true;
        }

        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        if input.is_pressed(GameKey::Pause) {
            self.paused = true;
            return true;
        }

        let dt_f32 = dt as f32;

        // Hit pause: freeze all game logic
        if self.hit_pause_frames > 0 {
            self.hit_pause_frames -= 1;
            self.particles.update(dt_f32);
            return true;
        }

        // Death sequence management
        match self.death_phase {
            DeathPhase::Alive => {}
            DeathPhase::Dying => {
                self.death_timer += dt_f32;
                self.player
                    .update_with_input(0.0, 0.0, false, false, dt, &self.tilemap);
                self.particles.update(dt_f32);
                self.camera.update(dt);
                if self.player.animation.is_finished() {
                    self.death_phase = DeathPhase::FadeOut;
                    self.death_timer = 0.0;
                }
                return true;
            }
            DeathPhase::FadeOut => {
                self.death_timer += dt_f32;
                self.particles.update(dt_f32);
                self.camera.update(dt);
                if self.death_timer >= DEATH_FADE_DURATION {
                    self.death_phase = DeathPhase::Dead;
                }
                return true;
            }
            DeathPhase::Dead => {
                if input.is_pressed(GameKey::Attack) {
                    self.restart();
                }
                return true;
            }
        }

        // Floor cleared: wait for restart
        if self.floor_clear {
            if input.is_pressed(GameKey::Attack) {
                self.advance_floor();
            }
            self.particles.update(dt_f32);
            return true;
        }

        // --- Room transition handling ---
        if self.dungeon.transition.is_some() {
            if let Some(event) = self.dungeon.update_transition(dt_f32) {
                match event {
                    TransitionEvent::SwapRoom => {
                        self.perform_room_swap();
                    }
                    TransitionEvent::Complete => {
                        // Transition finished, check if we need to start combat
                        self.enter_current_room();
                    }
                }
            }
            // During transitions, only update particles and camera
            self.particles.update(dt_f32);
            self.camera.update(dt);
            return true;
        }

        // --- Normal gameplay ---

        // Demo mode management
        if has_input(input) {
            self.idle_timer = 0.0;
            self.demo = None;
        } else {
            self.idle_timer += dt_f32;
        }

        let was_attacking = matches!(self.player.state, player::PlayerState::Attacking);
        let was_dashing = self.player.is_dashing();

        if self.demo.is_some() || self.idle_timer >= DEMO_IDLE_THRESHOLD {
            let demo = self.demo.get_or_insert_with(DemoState::new);
            let (dx, dy, attack, dash) = demo.update(dt_f32);
            self.player
                .update_with_input(dx, dy, attack, dash, dt, &self.tilemap);

            if attack && self.player.attack_cooldown > 0.0 {
                self.flash_timer = FLASH_FRAMES;
                self.camera.shake(3.0);
            }
            if dash {
                self.camera.shake(6.0);
            }
        } else {
            self.player.update(input, dt, &self.tilemap);

            if input.is_pressed(GameKey::Attack) && self.player.attack_cooldown > 0.0 {
                self.flash_timer = FLASH_FRAMES;
                self.camera.shake(3.0);
            }

            if input.is_pressed(GameKey::Dash) {
                self.camera.shake(6.0);
            }
        }

        // Room entry invincibility countdown
        if self.room_entry_invincibility > 0.0 {
            self.room_entry_invincibility -= dt_f32;
        }
        if self.sealed_flash_timer > 0.0 {
            self.sealed_flash_timer -= dt_f32;
        }

        // Dash trail particles
        if self.player.is_dashing() {
            let (cx, cy) = self.player.center();
            self.particles.burst(cx, cy, &DASH_TRAIL_CONFIG);
        }
        if self.player.is_dashing() && !was_dashing {
            let (cx, cy) = self.player.center();
            self.particles.burst(cx, cy + 4.0, &DUST_PUFF_CONFIG);
        }

        // Reset hit tracking when player starts a new attack
        let is_attacking = matches!(self.player.state, player::PlayerState::Attacking);
        if is_attacking && !was_attacking {
            for enemy in &mut self.enemies {
                enemy.hit_this_attack = false;
            }
        }

        // --- Combat: player attacks enemies ---
        let effects = combat::check_player_attacks(
            &self.player,
            &mut self.enemies,
            &mut self.particles,
            &mut self.damage_numbers,
        );
        if effects.hit_pause_frames > 0 {
            self.hit_pause_frames = effects.hit_pause_frames;
        }
        if effects.camera_shake > 0.0 {
            self.camera.shake(effects.camera_shake);
        }

        // Update enemies
        let (pcx, pcy) = self.player.center();
        for enemy in &mut self.enemies {
            enemy.update(dt, &self.tilemap, pcx, pcy);
        }

        // Boss-specific effects (slam impact, charge wall hit, phase transition, death)
        for enemy in &self.enemies {
            if enemy.enemy_type != enemies::EnemyType::BoneKing {
                continue;
            }
            let (ecx, ecy) = enemy.center();
            if enemy.boss_slam_impact {
                self.camera.shake(5.0);
                self.particles.burst(ecx, ecy + 8.0, &combat::BOSS_SLAM_BURST_CONFIG);
            }
            if enemy.boss_charge_wall_hit {
                self.camera.shake(6.0);
                self.hit_pause_frames = 6;
                self.particles.burst(ecx, ecy, &combat::BOSS_SLAM_BURST_CONFIG);
            }
            if enemy.boss_roaring {
                self.camera.shake(6.0);
            }
            if enemy.boss_death_finished {
                // Big death burst
                self.hit_pause_frames = 10;
                self.camera.shake(8.0);
                self.particles.burst(ecx, ecy, &combat::BOSS_DEATH_BURST_CONFIG);
                self.particles.burst(ecx, ecy, &combat::BOSS_DEATH_BURST_CONFIG);
            }
        }

        // Projectile spawning and physics
        combat::spawn_enemy_projectiles(&self.enemies, &mut self.projectiles);
        combat::update_projectiles(
            &mut self.projectiles,
            &self.tilemap,
            &mut self.particles,
            dt_f32,
        );

        // --- Combat: enemies attack player ---
        if self.room_entry_invincibility <= 0.0 {
            let effects = combat::check_enemy_attacks(
                &mut self.player,
                &self.enemies,
                &mut self.projectiles,
                &mut self.particles,
                &mut self.damage_numbers,
            );
            if effects.hit_pause_frames > 0 {
                self.hit_pause_frames = effects.hit_pause_frames;
            }
            if effects.camera_shake > 0.0 {
                self.camera.shake(effects.camera_shake);
            }
            if effects.player_died {
                self.death_phase = DeathPhase::Dying;
                self.death_timer = 0.0;
            }
        }

        // Heart flash on HP loss
        if self.player.hp < self.last_hp {
            self.heart_flash_timer = 0.3;
        }
        self.last_hp = self.player.hp;
        if self.heart_flash_timer > 0.0 {
            self.heart_flash_timer -= dt_f32;
        }

        // Update particles and damage numbers
        self.particles.update(dt_f32);
        for dn in &mut self.damage_numbers {
            dn.update(dt_f32);
        }
        self.damage_numbers.retain(|dn| dn.alive());

        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // --- Pickup update and collection ---
        for p in &mut self.pickups {
            p.update(dt_f32);
        }
        {
            let px = self.player.transform.position.x;
            let py = self.player.transform.position.y;
            for p in &mut self.pickups {
                if p.check_collection(px, py, 10.0, 14.0) {
                    let heal = p.heal_amount();
                    if heal > 0 {
                        self.player.hp = (self.player.hp + heal).min(self.player.max_hp);
                    }
                    self.particles.burst(p.x + 3.0, p.y + 3.0, &PICKUP_COLLECT_CONFIG);
                    p.alive = false;
                }
            }
            self.pickups.retain(|p| p.alive);
        }

        // --- Room state progression ---
        match self.room_state {
            RoomState::Peaceful => {
                // Check door collisions — player can walk through open doors
                let (pcx, pcy) = self.player.center();
                if let Some((to_room, direction)) =
                    self.dungeon.check_door_collision(pcx, pcy, &self.tilemap)
                {
                    self.dungeon.start_transition(to_room, direction);
                }
            }
            RoomState::Combat => {
                let alive_count = self.enemies.iter().filter(|e| e.alive).count();
                let all_animations_done = self.enemies.iter().all(|e| e.alive || e.animation.is_finished());

                // Check if we should spawn the next wave
                if let Some(ref mut tracker) = self.wave_tracker {
                    if tracker.has_more_waves() && tracker.should_spawn_next_wave(alive_count) {
                        let room_index = self.dungeon.current_room_index;
                        let spawn_points = &self.dungeon.current_room().template.spawn_points;
                        if let Some(wave) = tracker.advance() {
                            let new_enemies = encounters::instantiate_wave(
                                wave,
                                spawn_points,
                                room_index,
                                self.spawn_seed,
                            );
                            self.enemies.extend(new_enemies);
                        }
                    }
                }

                // Check if encounter is fully complete
                let alive_count = self.enemies.iter().filter(|e| e.alive).count();
                let encounter_done = self
                    .wave_tracker
                    .as_ref()
                    .is_some_and(|t| t.is_encounter_complete(alive_count));

                if encounter_done && all_animations_done {
                    self.room_state = RoomState::Cleared;
                    self.dungeon.mark_room_cleared(self.dungeon.current_room_index);
                    world::set_doors(&mut self.tilemap, true);
                    self.camera.shake(4.0);
                    self.wave_tracker = None;

                    // 25% chance to spawn a heal pickup at room center
                    let drop_seed = self.spawn_seed.wrapping_add(self.dungeon.current_room_index as u64 * 997);
                    if drop_seed.is_multiple_of(4) {
                        let room = self.dungeon.current_room();
                        let ts = engine::tilemap::TILE_SIZE as f32;
                        let cx = (room.template.width as f32 * ts) / 2.0 - 2.5;
                        let cy = (room.template.height as f32 * ts) / 2.0 - 2.5;
                        self.pickups.push(Pickup::new(cx, cy, pickup::PickupType::SmallHeal));
                    }
                }
            }
            RoomState::Cleared => {
                // Room is cleared, player can now use doors
                let (pcx, pcy) = self.player.center();
                if let Some((to_room, direction)) =
                    self.dungeon.check_door_collision(pcx, pcy, &self.tilemap)
                {
                    self.dungeon.start_transition(to_room, direction);
                }
            }
        }

        // Camera follows player center
        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.update(dt);
        self.camera
            .clamp_to_bounds(self.tilemap.pixel_width() as f32, self.tilemap.pixel_height() as f32);

        true
    }

    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        let fw = fb.width();
        let fh = fb.height();

        self.camera.viewport_w = fw;
        self.camera.viewport_h = fh;

        let (cam_x, cam_y) = self.camera.offset();

        // --- Draw tile map ---
        render_tilemap(fb, &self.tilemap, tiles::tile_sprite, cam_x, cam_y);

        // --- Draw pickups ---
        for p in &self.pickups {
            p.render(fb, cam_x, cam_y);
        }

        // --- Draw enemies ---
        for enemy in &self.enemies {
            enemy.render(fb, alpha, cam_x, cam_y);
        }

        // --- Draw player ---
        let player_visible = if self.player.is_dead() {
            !self.player.animation.is_finished()
        } else if self.player.invincible_timer > 0.0 && !self.player.is_dashing() {
            ((self.player.invincible_timer * 15.0) as u32).is_multiple_of(2)
        } else {
            true
        };

        if player_visible {
            if self.flash_timer > 0 {
                self.player
                    .render_tinted(fb, alpha, cam_x, cam_y, ATTACK_TINT);
            } else if self.player.is_dashing() {
                self.player
                    .render_tinted(fb, alpha, cam_x, cam_y, DASH_TINT);
            } else if self.player.invincible_timer > 0.0 && !self.player.is_dead() {
                if ((self.player.invincible_timer * 30.0) as u32).is_multiple_of(4) {
                    self.player
                        .render_tinted(fb, alpha, cam_x, cam_y, IFRAME_TINT);
                } else {
                    self.player.render(fb, alpha, cam_x, cam_y);
                }
            } else {
                self.player.render(fb, alpha, cam_x, cam_y);
            }
        }

        // --- Draw projectiles ---
        self.projectiles.render(fb, cam_x, cam_y);

        // --- Draw particles ---
        self.particles.render(fb, cam_x, cam_y);

        // --- Draw damage numbers ---
        for dn in &self.damage_numbers {
            dn.render(fb, cam_x, cam_y);
        }

        // --- Debug hitbox overlay ---
        if self.debug_hitboxes {
            self.render_debug_hitboxes(fb, cam_x, cam_y);
        }

        // --- Room transition overlay ---
        let transition_opacity = self.dungeon.transition_opacity();
        if transition_opacity > 0.0 {
            fb.overlay([0, 0, 0], transition_opacity);
        }

        // --- "SEALED" flash when doors close ---
        if self.sealed_flash_timer > 0.0 {
            let alpha_val = (self.sealed_flash_timer / 1.5).min(1.0);
            let brightness = (255.0 * alpha_val) as u8;
            let text = "SEALED";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 4;
            sprites::font::render_text(fb, text, tx, ty, [brightness, 40, 40]);
        }

        // --- Death fade overlay ---
        match self.death_phase {
            DeathPhase::FadeOut => {
                let opacity = (self.death_timer / DEATH_FADE_DURATION).min(1.0);
                fb.overlay([0, 0, 0], opacity);
            }
            DeathPhase::Dead => {
                fb.overlay([0, 0, 0], 1.0);
                let text = "YOU DIED";
                let tw = sprites::font::text_width(text);
                let tx = (fw as i32 - tw) / 2;
                let ty = (fh as i32) / 2 - 4;
                sprites::font::render_text(fb, text, tx, ty, [200, 30, 30]);

                let text2 = "PRESS ATTACK";
                let tw2 = sprites::font::text_width(text2);
                let tx2 = (fw as i32 - tw2) / 2;
                sprites::font::render_text(fb, text2, tx2, ty + 8, [150, 150, 150]);
                return;
            }
            _ => {}
        }

        // --- Floor cleared overlay ---
        if self.floor_clear {
            fb.overlay([0, 0, 0], 0.5);
            let text = "FLOOR CLEARED";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 10;
            sprites::font::render_text(fb, text, tx, ty, [255, 220, 50]);

            // Show rooms explored
            let explored = self.dungeon.floor.rooms.iter().filter(|r| r.discovered).count();
            let total = self.dungeon.floor.rooms.len();
            let stats = format!("ROOMS {}/{}", explored, total);
            let sw = sprites::font::text_width(&stats);
            let sx = (fw as i32 - sw) / 2;
            sprites::font::render_text(fb, &stats, sx, ty + 10, [180, 180, 180]);

            let text2 = "PRESS ATTACK";
            let tw2 = sprites::font::text_width(text2);
            let tx2 = (fw as i32 - tw2) / 2;
            sprites::font::render_text(fb, text2, tx2, ty + 20, [150, 150, 150]);
        }

        // --- Room name announce (when entering a new room) ---

        // --- HUD ---
        let bar_h = 8;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        hud::render_hearts(fb, self.player.hp, self.player.max_hp, 2, 1);

        // Heart flash overlay on damage
        if self.heart_flash_timer > 0.0 {
            let flash_intensity = (self.heart_flash_timer / 0.3).min(1.0);
            let flash_color: Color = [
                (255.0 * flash_intensity) as u8,
                (255.0 * flash_intensity) as u8,
                (255.0 * flash_intensity) as u8,
            ];
            for y in 1..6 {
                for x in 2..(2 + self.player.max_hp as usize * 6) {
                    if let Some(c) = fb.get_pixel(x, y) {
                        let blended = [
                            ((c[0] as f32 + flash_color[0] as f32) / 2.0).min(255.0) as u8,
                            ((c[1] as f32 + flash_color[1] as f32) / 2.0).min(255.0) as u8,
                            ((c[2] as f32 + flash_color[2] as f32) / 2.0).min(255.0) as u8,
                        ];
                        fb.set_pixel(x, y, blended);
                    }
                }
            }
        }

        // Floor number in HUD
        let floor_text = match self.dungeon.floor_number {
            1 => "F1",
            2 => "F2",
            3 => "F3",
            4 => "F4",
            5 => "F5",
            _ => "F?",
        };
        let ftw = sprites::font::text_width(floor_text);
        let ftx = (fw as i32 - ftw) / 2;
        sprites::font::render_text(fb, floor_text, ftx, 1, [180, 180, 180]);

        // Performance bars
        let bar_w = fw / 3;
        let bar_x = fw - bar_w;

        let fps_pixels = ((info.fps as usize) * bar_w) / 60;
        for x in bar_x..bar_x + fps_pixels.min(bar_w) {
            fb.set_pixel(x, 0, color::GREEN);
        }

        if info.cells_total > 0 {
            let ratio_pixels = (info.cells_redrawn * bar_w) / info.cells_total.max(1);
            for x in bar_x..bar_x + ratio_pixels.min(bar_w) {
                fb.set_pixel(x, 1, [255, 255, 0]);
            }
        }

        let frame_budget_us: u64 = 33_000;
        let draw_timing_bar = |fb: &mut FrameBuffer, y: usize, us: u64, c: Color| {
            let pixels = ((us as usize * bar_w) / frame_budget_us as usize).min(bar_w);
            for x in bar_x..bar_x + pixels {
                fb.set_pixel(x, y, c);
            }
        };
        draw_timing_bar(fb, 2, info.input_us, [0, 255, 255]);
        draw_timing_bar(fb, 3, info.render_us, [255, 80, 80]);

        // --- Boss health bar ---
        for enemy in &self.enemies {
            if enemy.enemy_type == enemies::EnemyType::BoneKing && (enemy.alive || enemy.boss_dying)
            {
                if let Some(max_hp) = enemy.boss_max_hp() {
                    hud::render_boss_bar(fb, "BONE KING", enemy.hp, max_hp);
                }
                break;
            }
        }

        // --- Minimap ---
        hud::render_minimap(
            fb,
            &self.dungeon.floor,
            self.dungeon.current_room_index,
            self.minimap_visible,
        );

        // --- Pause overlay (rendered last so it covers everything) ---
        if self.paused {
            fb.overlay([0, 0, 0], 0.6);

            let text = "PAUSED";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 16;
            sprites::font::render_text(fb, text, tx, ty, [255, 255, 255]);

            // Floor number
            let floor_label = match self.dungeon.floor_number {
                1 => "FLOOR 1",
                2 => "FLOOR 2",
                3 => "FLOOR 3",
                4 => "FLOOR 4",
                5 => "FLOOR 5",
                _ => "FLOOR",
            };
            let flw = sprites::font::text_width(floor_label);
            let flx = (fw as i32 - flw) / 2;
            sprites::font::render_text(fb, floor_label, flx, ty + 10, [180, 180, 180]);

            // HP display
            let hp_str = format!("HP {}/{}", self.player.hp, self.player.max_hp);
            let hpw = sprites::font::text_width(&hp_str);
            let hpx = (fw as i32 - hpw) / 2;
            sprites::font::render_text(fb, &hp_str, hpx, ty + 18, [200, 80, 80]);

            // Rooms explored count
            let explored = self.dungeon.floor.rooms.iter().filter(|r| r.discovered).count();
            let total = self.dungeon.floor.rooms.len();
            let rooms_str = format!("ROOMS {}/{}", explored, total);
            let rw = sprites::font::text_width(&rooms_str);
            let rx = (fw as i32 - rw) / 2;
            sprites::font::render_text(fb, &rooms_str, rx, ty + 26, [140, 140, 140]);

            // Controls hint
            let hint = "ESC - RESUME";
            let hw = sprites::font::text_width(hint);
            let hx = (fw as i32 - hw) / 2;
            sprites::font::render_text(fb, hint, hx, ty + 36, [100, 100, 100]);

            let hint2 = "Q - QUIT";
            let hw2 = sprites::font::text_width(hint2);
            let hx2 = (fw as i32 - hw2) / 2;
            sprites::font::render_text(fb, hint2, hx2, ty + 44, [100, 100, 100]);

            // Full-size minimap on pause screen (always visible, even if minimap is toggled off)
            hud::render_minimap(
                fb,
                &self.dungeon.floor,
                self.dungeon.current_room_index,
                true,
            );
        }
    }
}

impl CryptfallGame {
    /// Perform the actual room swap during a transition's Load phase.
    fn perform_room_swap(&mut self) {
        let transition = self.dungeon.transition.as_ref().unwrap();
        let to_room = transition.to_room;
        let direction = transition.direction;

        self.dungeon.swap_to_room(to_room);
        self.tilemap = self.dungeon.build_tilemap();

        // Position player at the entry matching their arrival direction
        let (px, py) = self.dungeon.player_spawn_position(Some(direction));
        self.player.transform.position.x = px;
        self.player.transform.position.y = py;
        self.player.transform.commit();

        // Clear per-room state
        self.enemies.clear();
        self.projectiles.clear();
        self.particles.clear();
        self.damage_numbers.clear();
        self.pickups.clear();
        self.wave_tracker = None;

        // Snap camera to new position
        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.snap();
        self.camera
            .clamp_to_bounds(self.tilemap.pixel_width() as f32, self.tilemap.pixel_height() as f32);
    }

    /// Called when transition completes — decide if room needs combat.
    fn enter_current_room(&mut self) {
        self.room_entry_invincibility = 0.5;

        // Check if this is the exit room
        if self.dungeon.is_exit_room() {
            self.floor_clear = true;
            return;
        }

        let room = self.dungeon.current_room();
        let room_index = self.dungeon.current_room_index;

        if room.cleared {
            self.room_state = RoomState::Peaceful;
            return;
        }

        match room.room_type {
            dungeon::room_template::RoomType::Start
            | dungeon::room_template::RoomType::Treasure
            | dungeon::room_template::RoomType::Shop
            | dungeon::room_template::RoomType::Exit => {
                self.room_state = RoomState::Peaceful;
            }
            dungeon::room_template::RoomType::Combat
            | dungeon::room_template::RoomType::Boss
            | dungeon::room_template::RoomType::Corridor => {
                // Close doors and select encounter
                world::set_doors(&mut self.tilemap, false);

                let difficulty = match room.room_type {
                    dungeon::room_template::RoomType::Boss => EncounterDifficulty::Boss,
                    dungeon::room_template::RoomType::Corridor => EncounterDifficulty::Easy,
                    _ => {
                        // Combat rooms: difficulty based on spawn point count
                        let n = room.template.spawn_points.len();
                        if n <= 3 {
                            EncounterDifficulty::Easy
                        } else if n <= 6 {
                            EncounterDifficulty::Medium
                        } else {
                            EncounterDifficulty::Hard
                        }
                    }
                };

                let encounter_seed = self.spawn_seed.wrapping_add(room_index as u64 * 31337);
                let num_sp = room.template.spawn_points.len();
                let encounter = encounters::select_encounter(
                    difficulty,
                    self.dungeon.floor_number,
                    num_sp,
                    encounter_seed,
                );

                let mut tracker = WaveTracker::new(encounter);

                // Spawn the first wave immediately
                let spawn_points = &room.template.spawn_points;
                if let Some(wave) = tracker.advance() {
                    self.enemies = encounters::instantiate_wave(
                        wave,
                        spawn_points,
                        room_index,
                        self.spawn_seed,
                    );
                }

                self.wave_tracker = Some(tracker);
                self.room_state = RoomState::Combat;
                self.sealed_flash_timer = 1.5;
                self.camera.shake(3.0);
            }
        }
    }

    fn restart(&mut self) {
        self.dungeon.reset();
        self.tilemap = self.dungeon.build_tilemap();
        let (px, py) = self.dungeon.player_spawn_position(None);
        self.player = Player::new(px, py);
        self.enemies.clear();
        self.projectiles.clear();
        self.particles.clear();
        self.damage_numbers.clear();
        self.pickups.clear();
        self.wave_tracker = None;
        self.room_state = RoomState::Peaceful;
        self.flash_timer = 0;
        self.hit_pause_frames = 0;
        self.death_phase = DeathPhase::Alive;
        self.death_timer = 0.0;
        self.idle_timer = 0.0;
        self.demo = None;
        self.heart_flash_timer = 0.0;
        self.last_hp = 5;
        self.room_entry_invincibility = 0.0;
        self.floor_clear = false;
        self.sealed_flash_timer = 0.0;

        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.snap();
    }

    fn advance_floor(&mut self) {
        self.dungeon.next_floor();
        self.tilemap = self.dungeon.build_tilemap();
        let (px, py) = self.dungeon.player_spawn_position(None);
        self.player.transform.position.x = px;
        self.player.transform.position.y = py;
        self.player.transform.commit();
        // Preserve player HP across floors
        self.enemies.clear();
        self.projectiles.clear();
        self.particles.clear();
        self.damage_numbers.clear();
        self.pickups.clear();
        self.wave_tracker = None;
        self.room_state = RoomState::Peaceful;
        self.floor_clear = false;

        let (cx, cy) = self.player.center();
        self.camera.follow(cx, cy);
        self.camera.snap();
        self.camera
            .clamp_to_bounds(self.tilemap.pixel_width() as f32, self.tilemap.pixel_height() as f32);
    }

    fn render_debug_hitboxes(&self, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
        let phb = self.player.world_hurtbox();
        draw_aabb_outline(fb, &phb, cam_x, cam_y, color::GREEN);

        if let Some(ahb) = self.player.attack_hitbox() {
            draw_aabb_outline(fb, &ahb, cam_x, cam_y, color::RED);
        }

        for enemy in &self.enemies {
            if enemy.alive {
                let ehb = enemy.world_hurtbox();
                draw_aabb_outline(fb, &ehb, cam_x, cam_y, [0, 200, 0]);
            }
        }
    }
}

fn draw_aabb_outline(
    fb: &mut FrameBuffer,
    aabb: &engine::AABB,
    cam_x: i32,
    cam_y: i32,
    color: Color,
) {
    let x0 = aabb.x as i32 - cam_x;
    let y0 = aabb.y as i32 - cam_y;
    let x1 = (aabb.x + aabb.w) as i32 - cam_x;
    let y1 = (aabb.y + aabb.h) as i32 - cam_y;

    for x in x0..=x1 {
        fb.set_pixel_safe(x, y0, color);
        fb.set_pixel_safe(x, y1, color);
    }
    for y in y0..=y1 {
        fb.set_pixel_safe(x0, y, color);
        fb.set_pixel_safe(x1, y, color);
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = engine::Terminal::new()?;
    let mut game = CryptfallGame::new();
    engine::run(&mut terminal, &mut game);
    Ok(())
}
