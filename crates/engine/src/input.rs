use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crossterm::event::{Event, KeyCode, KeyEvent};

/// Release timeout: if a key hasn't been re-seen in this duration, consider it released.
const HELD_TIMEOUT_MS: u128 = 150;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameKey {
    Up,
    Down,
    Left,
    Right,
    Attack,
    Dash,
    Pause,
    Quit,
}

pub struct InputState {
    /// Keys newly pressed this frame.
    pressed: HashSet<GameKey>,
    /// Keys currently held, with the timestamp of the last event seen.
    held: HashMap<GameKey, Instant>,
    /// Keys that timed out (inferred release) this frame.
    released: HashSet<GameKey>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            held: HashMap::new(),
            released: HashSet::new(),
        }
    }

    /// Call at the start of each frame before processing events.
    /// Clears per-frame state and checks for timed-out held keys.
    /// Zero-allocation: uses a fixed-size stack buffer (max 8 game keys).
    pub fn begin_frame(&mut self) {
        self.pressed.clear();
        self.released.clear();

        let now = Instant::now();

        // Terminals only send key repeats for one key at a time, so when
        // holding two directional keys simultaneously, the first key stops
        // getting events and would time out. Fix: use the most recent
        // directional key timestamp for all held directional keys. This
        // keeps both alive as long as any direction is actively receiving
        // events.
        let dir_keys = [GameKey::Up, GameKey::Down, GameKey::Left, GameKey::Right];
        let latest_dir_time = dir_keys
            .iter()
            .filter_map(|k| self.held.get(k).copied())
            .max();

        let mut timed_out = [None; 8];
        let mut count = 0;
        for (&key, &last_seen) in &self.held {
            let effective_time = if dir_keys.contains(&key) {
                latest_dir_time.unwrap_or(last_seen)
            } else {
                last_seen
            };
            if now.duration_since(effective_time).as_millis() > HELD_TIMEOUT_MS
                && count < timed_out.len()
            {
                timed_out[count] = Some(key);
                count += 1;
            }
        }

        for key in timed_out[..count].iter().flatten() {
            self.held.remove(key);
            self.released.insert(*key);
        }
    }

    /// Process all events for this frame. Call after `begin_frame()`.
    pub fn process_events(&mut self, events: &[Event]) {
        for evt in events {
            if let Event::Key(KeyEvent { code, .. }) = evt {
                if let Some(game_key) = map_key(*code) {
                    if self.held.contains_key(&game_key) {
                        // Already held — update timestamp
                        self.held.insert(game_key, Instant::now());
                    } else {
                        // Newly pressed
                        self.pressed.insert(game_key);
                        self.held.insert(game_key, Instant::now());
                        // If it was in released this frame (rapid re-press), remove from released
                        self.released.remove(&game_key);
                    }
                }
            }
        }
    }

    /// True only on the first frame of a key press.
    pub fn is_pressed(&self, key: GameKey) -> bool {
        self.pressed.contains(&key)
    }

    /// True while a key is being held (including the first frame).
    pub fn is_held(&self, key: GameKey) -> bool {
        self.held.contains_key(&key)
    }

    /// True on the frame the key times out (inferred release).
    pub fn is_released(&self, key: GameKey) -> bool {
        self.released.contains(&key)
    }

    /// Returns a normalized (dx, dy) direction vector from arrow/WASD state.
    /// Range: each component in -1.0..=1.0, normalized for diagonals.
    pub fn direction(&self) -> (f32, f32) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        if self.is_held(GameKey::Left) {
            dx -= 1.0;
        }
        if self.is_held(GameKey::Right) {
            dx += 1.0;
        }
        if self.is_held(GameKey::Up) {
            dy -= 1.0;
        }
        if self.is_held(GameKey::Down) {
            dy += 1.0;
        }

        // Normalize diagonal movement
        if dx != 0.0 && dy != 0.0 {
            let inv_sqrt2 = std::f32::consts::FRAC_1_SQRT_2;
            dx *= inv_sqrt2;
            dy *= inv_sqrt2;
        }

        (dx, dy)
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// Map a crossterm KeyCode to a GameKey, if applicable.
fn map_key(code: KeyCode) -> Option<GameKey> {
    match code {
        KeyCode::Up | KeyCode::Char('w') => Some(GameKey::Up),
        KeyCode::Down | KeyCode::Char('s') => Some(GameKey::Down),
        KeyCode::Left | KeyCode::Char('a') => Some(GameKey::Left),
        KeyCode::Right | KeyCode::Char('d') => Some(GameKey::Right),
        KeyCode::Char('z') | KeyCode::Enter => Some(GameKey::Attack),
        KeyCode::Char('x') | KeyCode::Char(' ') => Some(GameKey::Dash),
        KeyCode::Esc => Some(GameKey::Pause),
        KeyCode::Char('q') => Some(GameKey::Quit),
        _ => None,
    }
}
