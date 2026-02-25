mod boon_select;
mod boons;
mod combat;
mod dungeon;
mod enemies;
mod hud;
mod pickup;
mod player;
mod projectile;
mod run_state;
mod save;
mod sprites;
mod tiles;
mod tuning;
mod weapon_select;
mod weapons;


use boons::effects::PlayerBoons;
use boons::selection::select_boon_options;
use boons::BOON_DEFS;
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
use run_state::*;
use tuning::*;
use weapons::WeaponId;

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

// --- Title screen ember particles ---

const EMBER_COLORS: &[Color] = &[
    [255, 100, 30],
    [255, 140, 50],
    [200, 80, 20],
    [255, 60, 10],
    [180, 60, 15],
];

const EMBER_CONFIG: BurstConfig = BurstConfig {
    count_min: 1,
    count_max: 2,
    speed_min: 5.0,
    speed_max: 15.0,
    lifetime_min: 1.0,
    lifetime_max: 2.5,
    colors: EMBER_COLORS,
    gravity: -8.0,
    friction: 0.98,
    angle_spread: std::f32::consts::PI * 0.5,
    base_angle: -std::f32::consts::FRAC_PI_2,
};

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

// --- Screen state machine ---

enum GameScreen {
    Title,
    WeaponSelect,
    Playing,
    BoonSelect,
    UpgradeShop,
    RunEnd,
}

// --- Title screen state ---

struct TitleState {
    selected: usize, // 0=NEW RUN, 1=UPGRADES, 2=QUIT
    particles: ParticleSystem,
    ember_timer: f32,
    ember_seed: u32,
}

impl TitleState {
    fn new() -> Self {
        Self {
            selected: 0,
            particles: ParticleSystem::new(),
            ember_timer: 0.0,
            ember_seed: 12345,
        }
    }

    fn next_ember_pos(&mut self, fw: f32) -> f32 {
        self.ember_seed = self.ember_seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.ember_seed >> 16) as f32 % fw
    }
}

// --- Upgrade shop state ---

struct UpgradeShopState {
    selected: usize,
}

impl UpgradeShopState {
    fn new() -> Self {
        Self { selected: 0 }
    }
}

// --- Run end state ---

struct RunEndState {
    victory: bool,
}

// --- Gameplay state (only exists during Playing screen) ---

struct PlayingState {
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
    boons: PlayerBoons,
    combat_rooms_cleared: u32,
    boon_seed: u64,
}

impl PlayingState {
    fn new(weapon_id: WeaponId, save_data: &save::SaveData) -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(42);

        let dungeon = DungeonWorld::new(1, seed);
        let tilemap = dungeon.build_tilemap();
        let (px, py) = dungeon.player_spawn_position(None);
        let mut player = Player::new(px, py);
        player.equip_weapon(weapon_id);

        // Apply permanent upgrades
        let (bonus_hp, _bonus_dmg, _dash_charges, _rerolls) = save_data.upgrades.stat_bonuses();
        player.max_hp += bonus_hp;
        player.hp = player.max_hp;

        let base_hp = player.max_hp;

        let mut camera = Camera::new(80, 48);
        let (cx, cy) = player.center();
        camera.follow(cx, cy);
        camera.snap();
        camera.clamp_to_bounds(tilemap.pixel_width() as f32, tilemap.pixel_height() as f32);

        Self {
            player,
            enemies: Vec::new(),
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
            last_hp: base_hp,
            room_entry_invincibility: 0.0,
            floor_clear: false,
            sealed_flash_timer: 0.0,
            paused: false,
            minimap_visible: true,
            boons: PlayerBoons::new(),
            combat_rooms_cleared: 0,
            boon_seed: seed.wrapping_mul(7919),
        }
    }
}

// --- Main game state ---

struct CryptfallGame {
    screen: GameScreen,
    save_data: save::SaveData,
    run_state: run_state::RunState,
    // Screen-specific state
    title: TitleState,
    weapon_select: weapon_select::WeaponSelectScreen,
    boon_select: Option<boon_select::BoonSelectScreen>,
    upgrade_shop: UpgradeShopState,
    run_end: RunEndState,
    // Gameplay state (only valid during Playing/BoonSelect)
    playing: Option<PlayingState>,
}

impl CryptfallGame {
    fn new() -> Self {
        let save_data = save::SaveData::load();
        Self {
            screen: GameScreen::Title,
            save_data,
            run_state: run_state::RunState::new(),
            title: TitleState::new(),
            weapon_select: weapon_select::WeaponSelectScreen::new(),
            boon_select: None,
            upgrade_shop: UpgradeShopState::new(),
            run_end: RunEndState { victory: false },
            playing: None,
        }
    }

    fn start_new_run(&mut self, weapon_id: WeaponId) {
        self.run_state = run_state::RunState::new();
        self.run_state.floor_reached = 1;
        self.playing = Some(PlayingState::new(weapon_id, &self.save_data));
        self.screen = GameScreen::Playing;
    }

    fn end_run(&mut self, victory: bool) {
        // Commit run stats to save data
        self.save_data.total_runs += 1;
        self.save_data.total_kills += self.run_state.kills;
        if !victory {
            self.save_data.total_deaths += 1;
        }
        self.save_data.total_gold += self.run_state.gold_earned;
        if self.run_state.floor_reached > self.save_data.best_floor {
            self.save_data.best_floor = self.run_state.floor_reached;
        }
        self.save_data.save();

        self.run_end = RunEndState { victory };
        self.screen = GameScreen::RunEnd;
    }

    fn return_to_title(&mut self) {
        self.playing = None;
        self.title = TitleState::new();
        self.screen = GameScreen::Title;
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
        let dt_f32 = dt as f32;

        match self.screen {
            GameScreen::Title => self.update_title(input, dt_f32),
            GameScreen::WeaponSelect => self.update_weapon_select(input, dt_f32),
            GameScreen::BoonSelect => self.update_boon_select(input, dt_f32),
            GameScreen::UpgradeShop => self.update_upgrade_shop(input, dt_f32),
            GameScreen::RunEnd => self.update_run_end(input, dt_f32),
            GameScreen::Playing => self.update_playing(input, dt, dt_f32),
        }
    }

    fn render(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        match self.screen {
            GameScreen::Title => {
                self.render_title(fb);
                return;
            }
            GameScreen::WeaponSelect => {
                self.weapon_select.render(fb);
                return;
            }
            GameScreen::BoonSelect => {
                // Render the playing screen underneath, then overlay boon select
                self.render_playing(fb, info, alpha);
                if let Some(ref bs) = self.boon_select {
                    bs.render(fb);
                }
                return;
            }
            GameScreen::UpgradeShop => {
                self.render_upgrade_shop(fb);
                return;
            }
            GameScreen::RunEnd => {
                self.render_run_end(fb);
                return;
            }
            GameScreen::Playing => {}
        }

        self.render_playing(fb, info, alpha);
    }
}

// --- Screen-specific update methods ---

impl CryptfallGame {
    fn update_title(&mut self, input: &InputState, dt: f32) -> bool {
        if input.is_pressed(GameKey::Quit) {
            return false;
        }

        // Update title particles
        self.title.ember_timer -= dt;
        if self.title.ember_timer <= 0.0 {
            self.title.ember_timer = 0.15;
            let x = self.title.next_ember_pos(80.0);
            self.title.particles.burst(x, 48.0, &EMBER_CONFIG);
        }
        self.title.particles.update(dt);

        // Menu navigation
        if input.is_pressed(GameKey::Up) && self.title.selected > 0 {
            self.title.selected -= 1;
        }
        if input.is_pressed(GameKey::Down) && self.title.selected < 2 {
            self.title.selected += 1;
        }

        if input.is_pressed(GameKey::Attack) {
            match self.title.selected {
                0 => {
                    // NEW RUN -> weapon select
                    self.weapon_select = weapon_select::WeaponSelectScreen::new();
                    self.screen = GameScreen::WeaponSelect;
                }
                1 => {
                    // UPGRADES
                    self.upgrade_shop = UpgradeShopState::new();
                    self.screen = GameScreen::UpgradeShop;
                }
                2 => {
                    // QUIT
                    return false;
                }
                _ => {}
            }
        }

        true
    }

    fn update_weapon_select(&mut self, input: &InputState, dt: f32) -> bool {
        if input.is_pressed(GameKey::Pause) {
            self.screen = GameScreen::Title;
            return true;
        }

        if let Some(weapon_id) = self.weapon_select.update(input, dt) {
            self.start_new_run(weapon_id);
        }
        true
    }

    fn update_boon_select(&mut self, input: &InputState, dt: f32) -> bool {
        let selected_boon = if let Some(ref mut bs) = self.boon_select {
            bs.update(input, dt)
        } else {
            self.screen = GameScreen::Playing;
            return true;
        };

        if let Some(boon_id) = selected_boon {
            // Add boon to player and apply effects
            if let Some(ref mut ps) = self.playing {
                ps.boons.add(boon_id);

                // Apply max HP bonus from boons (e.g. ToughSkin)
                let base_max_hp = 5 + self.save_data.upgrades.stat_bonuses().0;
                let hp_bonus = ps.boons.effective_max_hp_bonus(base_max_hp);
                let new_max = base_max_hp + hp_bonus;
                let old_max = ps.player.max_hp;
                ps.player.max_hp = new_max;
                // If max HP increased, give the bonus HP
                if new_max > old_max {
                    ps.player.hp += new_max - old_max;
                }
                // If deaths bargain, clamp HP
                if ps.boons.has_deaths_bargain {
                    ps.player.max_hp = 1;
                    ps.player.hp = ps.player.hp.min(1);
                }
            }
            self.run_state.boons_collected += 1;
            self.boon_select = None;
            self.screen = GameScreen::Playing;
        }
        true
    }

    fn update_upgrade_shop(&mut self, input: &InputState, dt: f32) -> bool {
        let _ = dt;
        let max_idx = save::UPGRADES.len().saturating_sub(1);

        if input.is_pressed(GameKey::Pause) || input.is_pressed(GameKey::Quit) {
            self.screen = GameScreen::Title;
            return true;
        }

        if input.is_pressed(GameKey::Up) && self.upgrade_shop.selected > 0 {
            self.upgrade_shop.selected -= 1;
        }
        if input.is_pressed(GameKey::Down) && self.upgrade_shop.selected < max_idx {
            self.upgrade_shop.selected += 1;
        }

        if input.is_pressed(GameKey::Attack) {
            let idx = self.upgrade_shop.selected;
            let upgrade = &save::UPGRADES[idx];
            let current_level = (upgrade.current_level_fn)(&self.save_data.upgrades);
            if current_level < upgrade.max_level && self.save_data.can_afford(upgrade.cost) {
                self.save_data.spend_gold(upgrade.cost);
                // Apply the upgrade
                match idx {
                    0 => self.save_data.upgrades.vitality_level = (self.save_data.upgrades.vitality_level + 1).min(3),
                    1 => self.save_data.upgrades.vitality_level = (self.save_data.upgrades.vitality_level + 1).min(3),
                    2 => self.save_data.upgrades.vitality_level = (self.save_data.upgrades.vitality_level + 1).min(3),
                    3 => self.save_data.upgrades.strength_level = (self.save_data.upgrades.strength_level + 1).min(2),
                    4 => self.save_data.upgrades.strength_level = (self.save_data.upgrades.strength_level + 1).min(2),
                    5 => self.save_data.upgrades.twin_dash = true,
                    6 => self.save_data.upgrades.boon_reroll_level = (self.save_data.upgrades.boon_reroll_level + 1).min(2),
                    7 => self.save_data.upgrades.boon_reroll_level = (self.save_data.upgrades.boon_reroll_level + 1).min(2),
                    _ => {}
                }
                self.save_data.save();
            }
        }

        true
    }

    fn update_run_end(&mut self, input: &InputState, _dt: f32) -> bool {
        if input.is_pressed(GameKey::Attack) || input.is_pressed(GameKey::Pause) {
            self.return_to_title();
        }
        true
    }

    fn update_playing(&mut self, input: &InputState, dt: f64, dt_f32: f32) -> bool {
        let ps = match self.playing.as_mut() {
            Some(ps) => ps,
            None => return true,
        };

        // Pause toggle
        if input.is_pressed(GameKey::Pause) {
            ps.paused = !ps.paused;
            return true;
        }
        if ps.paused {
            if input.is_pressed(GameKey::Quit) {
                self.end_run(false);
            }
            return true;
        }

        // Minimap toggle
        if input.is_pressed(GameKey::Map) {
            ps.minimap_visible = !ps.minimap_visible;
        }

        // Debug toggle
        if input.is_pressed(GameKey::Quit) {
            ps.debug_hitboxes = !ps.debug_hitboxes;
        }

        // Death sequence handling
        match ps.death_phase {
            DeathPhase::Alive => {}
            DeathPhase::Dying => {
                ps.death_timer += dt_f32;
                ps.player
                    .update_with_input(0.0, 0.0, false, false, dt, &ps.tilemap);
                ps.particles.update(dt_f32);
                ps.camera.update(dt);
                if ps.player.animation.is_finished() {
                    ps.death_phase = DeathPhase::FadeOut;
                    ps.death_timer = 0.0;
                }
                return true;
            }
            DeathPhase::FadeOut => {
                ps.death_timer += dt_f32;
                ps.particles.update(dt_f32);
                ps.camera.update(dt);
                if ps.death_timer >= DEATH_FADE_DURATION {
                    ps.death_phase = DeathPhase::Dead;
                }
                return true;
            }
            DeathPhase::Dead => {
                if input.is_pressed(GameKey::Attack) {
                    self.end_run(false);
                }
                return true;
            }
        }

        // Track elapsed time
        self.run_state.elapsed_secs += dt_f32;

        let ps = self.playing.as_mut().unwrap();

        // Floor cleared: wait for advance
        if ps.floor_clear {
            if input.is_pressed(GameKey::Attack) {
                let is_final_floor = ps.dungeon.floor_number >= 5;
                if is_final_floor {
                    self.end_run(true);
                    return true;
                }
                self.advance_floor();
                if let Some(ref mut ps) = self.playing {
                    ps.particles.update(dt_f32);
                }
                return true;
            }
            ps.particles.update(dt_f32);
            return true;
        }

        // Room transition handling
        if ps.dungeon.transition.is_some() {
            if let Some(event) = ps.dungeon.update_transition(dt_f32) {
                match event {
                    TransitionEvent::SwapRoom => {
                        self.perform_room_swap();
                    }
                    TransitionEvent::Complete => {
                        self.enter_current_room();
                    }
                }
            }
            let ps = self.playing.as_mut().unwrap();
            ps.particles.update(dt_f32);
            ps.camera.update(dt);
            return true;
        }

        // Normal gameplay

        // Demo mode management
        if has_input(input) {
            ps.idle_timer = 0.0;
            ps.demo = None;
        } else {
            ps.idle_timer += dt_f32;
        }

        let was_attacking = matches!(ps.player.state, player::PlayerState::Attacking);
        let was_dashing = ps.player.is_dashing();

        if ps.demo.is_some() || ps.idle_timer >= DEMO_IDLE_THRESHOLD {
            let demo = ps.demo.get_or_insert_with(DemoState::new);
            let (dx, dy, attack, dash) = demo.update(dt_f32);
            ps.player
                .update_with_input(dx, dy, attack, dash, dt, &ps.tilemap);

            if attack && ps.player.attack_cooldown > 0.0 {
                ps.flash_timer = FLASH_FRAMES;
                ps.camera.shake(3.0);
            }
            if dash {
                ps.camera.shake(6.0);
            }
        } else {
            ps.player.update(input, dt, &ps.tilemap);

            if input.is_pressed(GameKey::Attack) && ps.player.attack_cooldown > 0.0 {
                ps.flash_timer = FLASH_FRAMES;
                ps.camera.shake(3.0);
            }

            if input.is_pressed(GameKey::Dash) {
                ps.camera.shake(6.0);
            }
        }

        // Room entry invincibility countdown
        if ps.room_entry_invincibility > 0.0 {
            ps.room_entry_invincibility -= dt_f32;
        }
        if ps.sealed_flash_timer > 0.0 {
            ps.sealed_flash_timer -= dt_f32;
        }

        // Dash trail particles
        if ps.player.is_dashing() {
            let (cx, cy) = ps.player.center();
            ps.particles.burst(cx, cy, &DASH_TRAIL_CONFIG);
        }
        if ps.player.is_dashing() && !was_dashing {
            let (cx, cy) = ps.player.center();
            ps.particles.burst(cx, cy + 4.0, &DUST_PUFF_CONFIG);
        }

        // Reset hit tracking when player starts a new attack
        let is_attacking = matches!(ps.player.state, player::PlayerState::Attacking);
        if is_attacking && !was_attacking {
            for enemy in &mut ps.enemies {
                enemy.hit_this_attack = false;
            }
        }

        // Combat: player attacks enemies
        let effects = combat::check_player_attacks(
            &ps.player,
            &mut ps.enemies,
            &mut ps.particles,
            &mut ps.damage_numbers,
        );
        if effects.hit_pause_frames > 0 {
            ps.hit_pause_frames = effects.hit_pause_frames;
        }
        if effects.camera_shake > 0.0 {
            ps.camera.shake(effects.camera_shake);
        }

        // Track kills and gold for enemies that just died
        {
            let mut newly_dead = Vec::new();
            for enemy in &ps.enemies {
                if !enemy.alive && enemy.hp <= 0 && enemy.hit_this_attack {
                    let gold = match enemy.enemy_type {
                        enemies::EnemyType::Skeleton => GOLD_SKELETON,
                        enemies::EnemyType::Ghost => GOLD_GHOST,
                        enemies::EnemyType::BoneKing => GOLD_BONE_KING,
                        enemies::EnemyType::Slime => GOLD_SKELETON,
                    };
                    newly_dead.push(gold);
                }
            }
            for gold in newly_dead {
                self.run_state.record_kill(gold);
                if let Some(ps) = self.playing.as_mut() {
                    ps.boons.record_kill();
                }
            }
        }

        let ps = self.playing.as_mut().unwrap();

        // Update enemies
        let (pcx, pcy) = ps.player.center();
        for enemy in &mut ps.enemies {
            enemy.update(dt, &ps.tilemap, pcx, pcy);
        }

        // Boss-specific effects
        for enemy in &ps.enemies {
            if enemy.enemy_type != enemies::EnemyType::BoneKing {
                continue;
            }
            let (ecx, ecy) = enemy.center();
            if enemy.boss_slam_impact {
                ps.camera.shake(5.0);
                ps.particles
                    .burst(ecx, ecy + 8.0, &combat::BOSS_SLAM_BURST_CONFIG);
            }
            if enemy.boss_charge_wall_hit {
                ps.camera.shake(6.0);
                ps.hit_pause_frames = 6;
                ps.particles
                    .burst(ecx, ecy, &combat::BOSS_SLAM_BURST_CONFIG);
            }
            if enemy.boss_roaring {
                ps.camera.shake(6.0);
            }
            if enemy.boss_death_finished {
                ps.hit_pause_frames = 10;
                ps.camera.shake(8.0);
                ps.particles
                    .burst(ecx, ecy, &combat::BOSS_DEATH_BURST_CONFIG);
                ps.particles
                    .burst(ecx, ecy, &combat::BOSS_DEATH_BURST_CONFIG);
            }
        }

        // Projectile spawning and physics
        combat::spawn_enemy_projectiles(&ps.enemies, &mut ps.projectiles);
        combat::update_projectiles(
            &mut ps.projectiles,
            &ps.tilemap,
            &mut ps.particles,
            dt_f32,
        );

        // Combat: enemies attack player
        if ps.room_entry_invincibility <= 0.0 {
            let effects = combat::check_enemy_attacks(
                &mut ps.player,
                &ps.enemies,
                &mut ps.projectiles,
                &mut ps.particles,
                &mut ps.damage_numbers,
            );
            if effects.hit_pause_frames > 0 {
                ps.hit_pause_frames = effects.hit_pause_frames;
            }
            if effects.camera_shake > 0.0 {
                ps.camera.shake(effects.camera_shake);
            }
            if effects.player_died {
                ps.death_phase = DeathPhase::Dying;
                ps.death_timer = 0.0;
            }
        }

        // Heart flash on HP loss
        if ps.player.hp < ps.last_hp {
            ps.heart_flash_timer = 0.3;
        }
        ps.last_hp = ps.player.hp;
        if ps.heart_flash_timer > 0.0 {
            ps.heart_flash_timer -= dt_f32;
        }

        // Update particles and damage numbers
        ps.particles.update(dt_f32);
        for dn in &mut ps.damage_numbers {
            dn.update(dt_f32);
        }
        ps.damage_numbers.retain(|dn| dn.alive());

        if ps.flash_timer > 0 {
            ps.flash_timer -= 1;
        }

        // Pickup update and collection
        for p in &mut ps.pickups {
            p.update(dt_f32);
        }
        {
            let px = ps.player.transform.position.x;
            let py = ps.player.transform.position.y;
            for p in &mut ps.pickups {
                if p.check_collection(px, py, 10.0, 14.0) {
                    let heal = p.heal_amount();
                    if heal > 0 {
                        ps.player.hp = (ps.player.hp + heal).min(ps.player.max_hp);
                    }
                    ps.particles
                        .burst(p.x + 3.0, p.y + 3.0, &PICKUP_COLLECT_CONFIG);
                    p.alive = false;
                }
            }
            ps.pickups.retain(|p| p.alive);
        }

        // Room state progression
        let mut boon_ids_for_select: Vec<boons::BoonId> = Vec::new();
        let mut encounter_just_cleared = false;

        match ps.room_state {
            RoomState::Peaceful => {
                let (pcx, pcy) = ps.player.center();
                if let Some((to_room, direction)) =
                    ps.dungeon.check_door_collision(pcx, pcy, &ps.tilemap)
                {
                    ps.dungeon.start_transition(to_room, direction);
                }
            }
            RoomState::Combat => {
                let alive_count = ps.enemies.iter().filter(|e| e.alive).count();
                let all_animations_done =
                    ps.enemies.iter().all(|e| e.alive || e.animation.is_finished());

                if let Some(ref mut tracker) = ps.wave_tracker {
                    if tracker.has_more_waves() && tracker.should_spawn_next_wave(alive_count) {
                        let room_index = ps.dungeon.current_room_index;
                        let spawn_points = &ps.dungeon.current_room().template.spawn_points;
                        if let Some(wave) = tracker.advance() {
                            let new_enemies = encounters::instantiate_wave(
                                wave,
                                spawn_points,
                                room_index,
                                ps.spawn_seed,
                            );
                            ps.enemies.extend(new_enemies);
                        }
                    }
                }

                let alive_count = ps.enemies.iter().filter(|e| e.alive).count();
                let encounter_done = ps
                    .wave_tracker
                    .as_ref()
                    .is_some_and(|t| t.is_encounter_complete(alive_count));

                if encounter_done && all_animations_done {
                    ps.room_state = RoomState::Cleared;
                    ps.dungeon.mark_room_cleared(ps.dungeon.current_room_index);
                    world::set_doors(&mut ps.tilemap, true);
                    ps.camera.shake(4.0);
                    ps.wave_tracker = None;

                    ps.combat_rooms_cleared += 1;
                    let combat_rooms_cleared = ps.combat_rooms_cleared;

                    ps.boons.reset_room_state();

                    if combat_rooms_cleared > 0 && combat_rooms_cleared.is_multiple_of(2) {
                        ps.boon_seed =
                            ps.boon_seed.wrapping_add(combat_rooms_cleared as u64 * 997);
                        let options = select_boon_options(
                            BOON_DEFS,
                            &ps.boons.active,
                            ps.boons.lucky,
                            ps.boon_seed,
                        );
                        boon_ids_for_select = options.iter().map(|b| b.id).collect();
                    }

                    let drop_seed = ps
                        .spawn_seed
                        .wrapping_add(ps.dungeon.current_room_index as u64 * 997);
                    if drop_seed.is_multiple_of(4) {
                        let room = ps.dungeon.current_room();
                        let ts = engine::tilemap::TILE_SIZE as f32;
                        let cx = (room.template.width as f32 * ts) / 2.0 - 2.5;
                        let cy = (room.template.height as f32 * ts) / 2.0 - 2.5;
                        ps.pickups
                            .push(Pickup::new(cx, cy, pickup::PickupType::SmallHeal));
                    }

                    encounter_just_cleared = true;
                }
            }
            RoomState::Cleared => {
                let (pcx, pcy) = ps.player.center();
                if let Some((to_room, direction)) =
                    ps.dungeon.check_door_collision(pcx, pcy, &ps.tilemap)
                {
                    ps.dungeon.start_transition(to_room, direction);
                }
            }
        }

        // Camera follows player center
        let (cx, cy) = ps.player.center();
        ps.camera.follow(cx, cy);
        ps.camera.update(dt);
        ps.camera
            .clamp_to_bounds(ps.tilemap.pixel_width() as f32, ps.tilemap.pixel_height() as f32);

        // Deferred encounter clear: update fields that need &mut self
        if encounter_just_cleared {
            self.run_state.record_room_clear(GOLD_ROOM_CLEAR_BONUS);
            if !boon_ids_for_select.is_empty() {
                self.boon_select = Some(boon_select::BoonSelectScreen::new(boon_ids_for_select));
                self.screen = GameScreen::BoonSelect;
                return true;
            }
        }

        true
    }
}

// --- Screen-specific render methods ---

impl CryptfallGame {
    fn render_title(&mut self, fb: &mut FrameBuffer) {
        let fw = fb.width() as i32;
        let fh = fb.height() as i32;

        // Dark background
        for y in 0..fh {
            for x in 0..fw {
                // Subtle vertical gradient: dark blue to black
                let ratio = y as f32 / fh as f32;
                let r = (8.0 * (1.0 - ratio)) as u8;
                let g = (12.0 * (1.0 - ratio)) as u8;
                let b = (25.0 * (1.0 - ratio)) as u8;
                fb.set_pixel_safe(x, y, [r, g, b]);
            }
        }

        // Particles (embers rising from bottom)
        self.title.particles.render(fb, 0, 0);

        // Title logo centered near top
        let logo = &sprites::title::TITLE_CRYPTFALL;
        let logo_x = (fw - logo.width as i32) / 2;
        let logo_y = fh / 6;
        fb.blit_sprite(logo, logo_x, logo_y);

        // Tagline
        let tagline = "A TERMINAL ROGUELIKE";
        let tw = sprites::font::text_width(tagline);
        let tx = (fw - tw) / 2;
        sprites::font::render_text(fb, tagline, tx, logo_y + 14, [100, 100, 140]);

        // Menu items
        let menu_items = ["NEW RUN", "UPGRADES", "QUIT"];
        let menu_y = fh / 2 + 2;
        for (i, item) in menu_items.iter().enumerate() {
            let is_selected = i == self.title.selected;
            let color: Color = if is_selected {
                [255, 220, 100]
            } else {
                [120, 120, 140]
            };
            let iw = sprites::font::text_width(item);
            let ix = (fw - iw) / 2;
            let iy = menu_y + i as i32 * 8;

            // Selection indicator
            if is_selected {
                let arrow = ">";
                let aw = sprites::font::text_width(arrow);
                sprites::font::render_text(fb, arrow, ix - aw - 3, iy, [255, 220, 100]);
            }

            sprites::font::render_text(fb, item, ix, iy, color);
        }

        // Stats at bottom
        let gold_str = format!("GOLD: {}", self.save_data.total_gold);
        let gw = sprites::font::text_width(&gold_str);
        let gx = (fw - gw) / 2;
        sprites::font::render_text(fb, &gold_str, gx, fh - 14, [255, 200, 50]);

        let floor_str = format!("BEST FLOOR: {}", self.save_data.best_floor);
        let flw = sprites::font::text_width(&floor_str);
        let flx = (fw - flw) / 2;
        sprites::font::render_text(fb, &floor_str, flx, fh - 8, [140, 140, 160]);
    }

    fn render_upgrade_shop(&self, fb: &mut FrameBuffer) {
        let fw = fb.width() as i32;
        let fh = fb.height() as i32;

        // Dark background
        for y in 0..fh {
            for x in 0..fw {
                fb.set_pixel_safe(x, y, [10, 10, 15]);
            }
        }

        // Title
        let title = "UPGRADE SHOP";
        let tw = sprites::font::text_width(title);
        let tx = (fw - tw) / 2;
        sprites::font::render_text(fb, title, tx, 4, [255, 220, 100]);

        // Gold display
        let gold_str = format!("GOLD: {}", self.save_data.total_gold);
        let gw = sprites::font::text_width(&gold_str);
        sprites::font::render_text(fb, &gold_str, fw - gw - 4, 4, [255, 200, 50]);

        // Upgrade list
        let start_y = 16;
        for (i, upgrade) in save::UPGRADES.iter().enumerate() {
            let is_selected = i == self.upgrade_shop.selected;
            let current_level = (upgrade.current_level_fn)(&self.save_data.upgrades);
            let is_owned = current_level >= upgrade.max_level;
            let can_afford = self.save_data.can_afford(upgrade.cost);

            let y = start_y + i as i32 * 7;

            // Selection indicator
            if is_selected {
                sprites::font::render_text(fb, ">", 2, y, [255, 220, 100]);
            }

            // Name
            let name_color: Color = if is_owned {
                [80, 180, 80]
            } else if is_selected && can_afford {
                [255, 255, 255]
            } else if is_selected {
                [180, 80, 80]
            } else if can_afford {
                [160, 160, 170]
            } else {
                [80, 80, 90]
            };
            sprites::font::render_text(fb, upgrade.name, 8, y, name_color);

            // Description
            let desc_x = 8 + sprites::font::text_width(upgrade.name) + 4;
            let desc_color: Color = if is_owned {
                [60, 120, 60]
            } else {
                [100, 100, 110]
            };
            sprites::font::render_text(fb, upgrade.description, desc_x, y, desc_color);

            // Cost or OWNED
            if is_owned {
                let owned_str = "[OWNED]";
                let ow = sprites::font::text_width(owned_str);
                sprites::font::render_text(fb, owned_str, fw - ow - 4, y, [80, 180, 80]);
            } else {
                let cost_str = format!("{}G", upgrade.cost);
                let cw = sprites::font::text_width(&cost_str);
                let cost_color: Color = if can_afford {
                    [255, 200, 50]
                } else {
                    [100, 60, 60]
                };
                sprites::font::render_text(fb, &cost_str, fw - cw - 4, y, cost_color);
            }
        }

        // Back hint
        let hint = "ESC - BACK";
        let hw = sprites::font::text_width(hint);
        let hx = (fw - hw) / 2;
        sprites::font::render_text(fb, hint, hx, fh - 6, [80, 80, 90]);
    }

    fn render_run_end(&self, fb: &mut FrameBuffer) {
        let fw = fb.width() as i32;
        let fh = fb.height() as i32;

        // Dark background
        for y in 0..fh {
            for x in 0..fw {
                fb.set_pixel_safe(x, y, [5, 5, 8]);
            }
        }

        // Header
        let (header, header_color): (&str, Color) = if self.run_end.victory {
            ("RUN COMPLETE", [255, 220, 50])
        } else {
            ("YOU DIED", [200, 30, 30])
        };
        let hw = sprites::font::text_width(header);
        let hx = (fw - hw) / 2;
        sprites::font::render_text(fb, header, hx, fh / 4, header_color);

        // Stats
        let stats_y = fh / 4 + 12;
        let stats = [
            format!("FLOOR: {}", self.run_state.floor_reached),
            format!("KILLS: {}", self.run_state.kills),
            format!("ROOMS: {}", self.run_state.rooms_cleared),
            format!("BOONS: {}", self.run_state.boons_collected),
            format!("GOLD: +{}", self.run_state.gold_earned),
        ];

        // Time display
        let secs = self.run_state.elapsed_secs as u32;
        let mins = secs / 60;
        let remaining_secs = secs % 60;
        let time_str = format!("TIME: {}:{:02}", mins, remaining_secs);

        for (i, stat) in stats.iter().enumerate() {
            let sw = sprites::font::text_width(stat);
            let sx = (fw - sw) / 2;
            sprites::font::render_text(fb, stat, sx, stats_y + i as i32 * 7, [180, 180, 200]);
        }

        let tw = sprites::font::text_width(&time_str);
        let tx = (fw - tw) / 2;
        sprites::font::render_text(fb, &time_str, tx, stats_y + stats.len() as i32 * 7, [180, 180, 200]);

        // Continue hint
        let hint = "PRESS ATTACK";
        let hiw = sprites::font::text_width(hint);
        let hix = (fw - hiw) / 2;
        sprites::font::render_text(fb, hint, hix, fh - 10, [120, 120, 130]);
    }

    fn render_playing(&mut self, fb: &mut FrameBuffer, info: &FrameInfo, alpha: f32) {
        let ps = match self.playing.as_mut() {
            Some(ps) => ps,
            None => return,
        };

        let fw = fb.width();
        let fh = fb.height();

        ps.camera.viewport_w = fw;
        ps.camera.viewport_h = fh;

        let (cam_x, cam_y) = ps.camera.offset();

        // --- Draw tile map ---
        render_tilemap(fb, &ps.tilemap, tiles::tile_sprite, cam_x, cam_y);

        // --- Draw pickups ---
        for p in &ps.pickups {
            p.render(fb, cam_x, cam_y);
        }

        // --- Draw enemies ---
        for enemy in &ps.enemies {
            enemy.render(fb, alpha, cam_x, cam_y);
        }

        // --- Draw player ---
        let player_visible = if ps.player.is_dead() {
            !ps.player.animation.is_finished()
        } else if ps.player.invincible_timer > 0.0 && !ps.player.is_dashing() {
            ((ps.player.invincible_timer * 15.0) as u32).is_multiple_of(2)
        } else {
            true
        };

        if player_visible {
            if ps.flash_timer > 0 {
                ps.player
                    .render_tinted(fb, alpha, cam_x, cam_y, ATTACK_TINT);
            } else if ps.player.is_dashing() {
                ps.player
                    .render_tinted(fb, alpha, cam_x, cam_y, DASH_TINT);
            } else if ps.player.invincible_timer > 0.0 && !ps.player.is_dead() {
                if ((ps.player.invincible_timer * 30.0) as u32).is_multiple_of(4) {
                    ps.player
                        .render_tinted(fb, alpha, cam_x, cam_y, IFRAME_TINT);
                } else {
                    ps.player.render(fb, alpha, cam_x, cam_y);
                }
            } else {
                ps.player.render(fb, alpha, cam_x, cam_y);
            }
        }

        // --- Draw projectiles ---
        ps.projectiles.render(fb, cam_x, cam_y);

        // --- Draw particles ---
        ps.particles.render(fb, cam_x, cam_y);

        // --- Draw damage numbers ---
        for dn in &ps.damage_numbers {
            dn.render(fb, cam_x, cam_y);
        }

        // --- Debug hitbox overlay ---
        if ps.debug_hitboxes {
            render_debug_hitboxes(ps, fb, cam_x, cam_y);
        }

        // --- Room transition overlay ---
        let transition_opacity = ps.dungeon.transition_opacity();
        if transition_opacity > 0.0 {
            fb.overlay([0, 0, 0], transition_opacity);
        }

        // --- "SEALED" flash when doors close ---
        if ps.sealed_flash_timer > 0.0 {
            let alpha_val = (ps.sealed_flash_timer / 1.5).min(1.0);
            let brightness = (255.0 * alpha_val) as u8;
            let text = "SEALED";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 4;
            sprites::font::render_text(fb, text, tx, ty, [brightness, 40, 40]);
        }

        // --- Death fade overlay ---
        match ps.death_phase {
            DeathPhase::FadeOut => {
                let opacity = (ps.death_timer / DEATH_FADE_DURATION).min(1.0);
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
        if ps.floor_clear {
            fb.overlay([0, 0, 0], 0.5);
            let text = if ps.dungeon.floor_number >= 5 { "VICTORY" } else { "FLOOR CLEARED" };
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 10;
            sprites::font::render_text(fb, text, tx, ty, [255, 220, 50]);

            let explored = ps.dungeon.floor.rooms.iter().filter(|r| r.discovered).count();
            let total = ps.dungeon.floor.rooms.len();
            let stats = format!("ROOMS {}/{}", explored, total);
            let sw = sprites::font::text_width(&stats);
            let sx = (fw as i32 - sw) / 2;
            sprites::font::render_text(fb, &stats, sx, ty + 10, [180, 180, 180]);

            let text2 = "PRESS ATTACK";
            let tw2 = sprites::font::text_width(text2);
            let tx2 = (fw as i32 - tw2) / 2;
            sprites::font::render_text(fb, text2, tx2, ty + 20, [150, 150, 150]);
        }

        // --- HUD ---
        let bar_h = 8;
        for y in 0..bar_h.min(fh) {
            for x in 0..fw {
                fb.set_pixel(x, y, [0, 0, 0]);
            }
        }

        hud::render_hearts(fb, ps.player.hp, ps.player.max_hp, 2, 1);

        // Heart flash overlay on damage
        if ps.heart_flash_timer > 0.0 {
            let flash_intensity = (ps.heart_flash_timer / 0.3).min(1.0);
            let flash_color: Color = [
                (255.0 * flash_intensity) as u8,
                (255.0 * flash_intensity) as u8,
                (255.0 * flash_intensity) as u8,
            ];
            for y in 1..6 {
                for x in 2..(2 + ps.player.max_hp as usize * 6) {
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
        let floor_text = match ps.dungeon.floor_number {
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

        // Gold in HUD (right side, below perf bars)
        let gold_str = format!("{}G", self.run_state.gold_earned);
        let gsw = sprites::font::text_width(&gold_str);
        sprites::font::render_text(fb, &gold_str, fw as i32 - gsw - 2, 5, [255, 200, 50]);

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
        for enemy in &ps.enemies {
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
            &ps.dungeon.floor,
            ps.dungeon.current_room_index,
            ps.minimap_visible,
        );

        // --- Pause overlay (rendered last so it covers everything) ---
        if ps.paused {
            fb.overlay([0, 0, 0], 0.6);

            let text = "PAUSED";
            let tw = sprites::font::text_width(text);
            let tx = (fw as i32 - tw) / 2;
            let ty = (fh as i32) / 2 - 16;
            sprites::font::render_text(fb, text, tx, ty, [255, 255, 255]);

            let floor_label = match ps.dungeon.floor_number {
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

            let hp_str = format!("HP {}/{}", ps.player.hp, ps.player.max_hp);
            let hpw = sprites::font::text_width(&hp_str);
            let hpx = (fw as i32 - hpw) / 2;
            sprites::font::render_text(fb, &hp_str, hpx, ty + 18, [200, 80, 80]);

            let explored = ps.dungeon.floor.rooms.iter().filter(|r| r.discovered).count();
            let total = ps.dungeon.floor.rooms.len();
            let rooms_str = format!("ROOMS {}/{}", explored, total);
            let rw = sprites::font::text_width(&rooms_str);
            let rx = (fw as i32 - rw) / 2;
            sprites::font::render_text(fb, &rooms_str, rx, ty + 26, [140, 140, 140]);

            // Boon count
            let boon_str = format!("BOONS: {}", ps.boons.active.len());
            let bw = sprites::font::text_width(&boon_str);
            let bx = (fw as i32 - bw) / 2;
            sprites::font::render_text(fb, &boon_str, bx, ty + 34, [140, 140, 140]);

            let hint = "ESC - RESUME  Q - QUIT";
            let hw = sprites::font::text_width(hint);
            let hx = (fw as i32 - hw) / 2;
            sprites::font::render_text(fb, hint, hx, ty + 44, [100, 100, 100]);

            // Full-size minimap on pause screen
            hud::render_minimap(
                fb,
                &ps.dungeon.floor,
                ps.dungeon.current_room_index,
                true,
            );
        }
    }
}

// --- Gameplay helper methods ---

impl CryptfallGame {
    /// Perform the actual room swap during a transition's Load phase.
    fn perform_room_swap(&mut self) {
        let ps = self.playing.as_mut().unwrap();
        let transition = ps.dungeon.transition.as_ref().unwrap();
        let to_room = transition.to_room;
        let direction = transition.direction;

        ps.dungeon.swap_to_room(to_room);
        ps.tilemap = ps.dungeon.build_tilemap();

        let (px, py) = ps.dungeon.player_spawn_position(Some(direction));
        ps.player.transform.position.x = px;
        ps.player.transform.position.y = py;
        ps.player.transform.commit();

        ps.enemies.clear();
        ps.projectiles.clear();
        ps.particles.clear();
        ps.damage_numbers.clear();
        ps.pickups.clear();
        ps.wave_tracker = None;

        let (cx, cy) = ps.player.center();
        ps.camera.follow(cx, cy);
        ps.camera.snap();
        ps.camera
            .clamp_to_bounds(ps.tilemap.pixel_width() as f32, ps.tilemap.pixel_height() as f32);
    }

    /// Called when transition completes -- decide if room needs combat.
    fn enter_current_room(&mut self) {
        let ps = self.playing.as_mut().unwrap();
        ps.room_entry_invincibility = 0.5;

        // Check if this is the exit room
        if ps.dungeon.is_exit_room() {
            ps.floor_clear = true;
            self.run_state.floor_reached = ps.dungeon.floor_number;
            return;
        }

        let room = ps.dungeon.current_room();
        let room_index = ps.dungeon.current_room_index;

        if room.cleared {
            ps.room_state = RoomState::Peaceful;
            return;
        }

        match room.room_type {
            dungeon::room_template::RoomType::Start
            | dungeon::room_template::RoomType::Treasure
            | dungeon::room_template::RoomType::Shop
            | dungeon::room_template::RoomType::Exit => {
                ps.room_state = RoomState::Peaceful;
            }
            dungeon::room_template::RoomType::Combat
            | dungeon::room_template::RoomType::Boss
            | dungeon::room_template::RoomType::Corridor => {
                world::set_doors(&mut ps.tilemap, false);

                let difficulty = match room.room_type {
                    dungeon::room_template::RoomType::Boss => EncounterDifficulty::Boss,
                    dungeon::room_template::RoomType::Corridor => EncounterDifficulty::Easy,
                    _ => {
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

                let encounter_seed = ps.spawn_seed.wrapping_add(room_index as u64 * 31337);
                let num_sp = room.template.spawn_points.len();
                let encounter = encounters::select_encounter(
                    difficulty,
                    ps.dungeon.floor_number,
                    num_sp,
                    encounter_seed,
                );

                let mut tracker = WaveTracker::new(encounter);

                let spawn_points = &room.template.spawn_points;
                if let Some(wave) = tracker.advance() {
                    ps.enemies = encounters::instantiate_wave(
                        wave,
                        spawn_points,
                        room_index,
                        ps.spawn_seed,
                    );
                }

                ps.wave_tracker = Some(tracker);
                ps.room_state = RoomState::Combat;
                ps.sealed_flash_timer = 1.5;
                ps.camera.shake(3.0);

                // Reset per-room boon state
                ps.boons.reset_room_state();
            }
        }
    }

    fn advance_floor(&mut self) {
        let ps = self.playing.as_mut().unwrap();
        ps.dungeon.next_floor();
        ps.tilemap = ps.dungeon.build_tilemap();
        let (px, py) = ps.dungeon.player_spawn_position(None);
        ps.player.transform.position.x = px;
        ps.player.transform.position.y = py;
        ps.player.transform.commit();
        ps.enemies.clear();
        ps.projectiles.clear();
        ps.particles.clear();
        ps.damage_numbers.clear();
        ps.pickups.clear();
        ps.wave_tracker = None;
        ps.room_state = RoomState::Peaceful;
        ps.floor_clear = false;

        // Update floor tracking
        self.run_state.floor_reached = ps.dungeon.floor_number;

        // Reset per-floor boon state
        ps.boons.reset_floor_state();

        let (cx, cy) = ps.player.center();
        ps.camera.follow(cx, cy);
        ps.camera.snap();
        ps.camera
            .clamp_to_bounds(ps.tilemap.pixel_width() as f32, ps.tilemap.pixel_height() as f32);
    }
}

fn render_debug_hitboxes(ps: &PlayingState, fb: &mut FrameBuffer, cam_x: i32, cam_y: i32) {
    let phb = ps.player.world_hurtbox();
    draw_aabb_outline(fb, &phb, cam_x, cam_y, color::GREEN);

    if let Some(ahb) = ps.player.attack_hitbox() {
        draw_aabb_outline(fb, &ahb, cam_x, cam_y, color::RED);
    }

    for enemy in &ps.enemies {
        if enemy.alive {
            let ehb = enemy.world_hurtbox();
            draw_aabb_outline(fb, &ehb, cam_x, cam_y, [0, 200, 0]);
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
