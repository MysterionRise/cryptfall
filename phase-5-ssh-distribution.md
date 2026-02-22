# Phase 5: SSH Playability & Distribution — Zero-Install Viral Launch

## Overview
**Duration:** Weeks 23–26 (6–8 sessions)
**Goal:** Anyone plays by typing `ssh play.cryptfall.dev`. Binary releases for all platforms. README with viral GIF.
**Prerequisite:** Phase 4 complete (full game loop with progression)

This phase transforms a cool project into a launchable product. SSH playability is the #1 viral vector for terminal games.

---

## Session 5.1 — SSH Server Foundation

### Claude Code Prompt

```
You are the Infrastructure Engineer for Cryptfall. Build the SSH server that lets anyone play by connecting via SSH.

### Create crates/server/

Dependencies:
```toml
[dependencies]
russh = "0.48"          # SSH server library (pure Rust, async)
russh-keys = "0.48"     # SSH key handling
tokio = { version = "1", features = ["full"] }
engine = { path = "../engine" }
game = { path = "../game" }  # game logic as a library
```

### Architecture:

The server accepts SSH connections and spawns a game instance per connection:

```rust
// server/src/main.rs
use russh::server::{self, Auth, Session};
use russh::{Channel, ChannelId};

struct CryptfallServer {
    // Server-wide state
    active_sessions: Arc<Mutex<usize>>,
    max_sessions: usize,  // cap at e.g. 50 concurrent
}

struct CryptfallSession {
    // Per-connection state
    channel: Option<Channel<server::Msg>>,
    game: Option<GameInstance>,
    terminal_size: (u16, u16),
}
```

### SSH handshake flow:

1. Client connects → server accepts (password auth: accept any, or no auth)
2. Client requests PTY → server records terminal size
3. Client opens channel → server stores channel handle
4. Server sends welcome banner:
   ```
   ╔══════════════════════════════════╗
   ║  CRYPTFALL — Terminal Roguelike  ║
   ║  github.com/you/cryptfall       ║
   ╚══════════════════════════════════╝
   Detecting terminal capabilities...
   ```
5. Server probes terminal:
   - Query terminal size (already from PTY request)
   - Check true color support: send a test color sequence, ideally check $COLORTERM
   - Minimum size: 80×40 terminal cells. If smaller, show error and disconnect.
6. Server creates GameInstance with appropriate settings
7. Game loop runs, writing frames to the SSH channel
8. Input from SSH channel feeds into the game's input system

### Key implementation details:

**Output:** The game engine's renderer currently writes to stdout. For SSH, it needs to write to the SSH channel instead. Refactor the renderer to accept a `Write` trait object:

```rust
impl Renderer {
    pub fn render_to(&self, writer: &mut dyn Write, fb: &FrameBuffer) -> io::Result<()> {
        // Same escape sequence generation, but writes to arbitrary output
    }
}
```

For local play: writer = stdout
For SSH play: writer = SSH channel

**Input:** SSH channels deliver raw bytes. These need to be parsed into crossterm-compatible key events. The escape sequences are standard ANSI, so:
- Arrow keys: ESC [ A/B/C/D
- Enter: \r
- Escape: ESC (tricky — need timeout to distinguish from escape sequences)
- Regular keys: single bytes

Create a simple ANSI input parser:
```rust
pub fn parse_ssh_input(bytes: &[u8]) -> Vec<GameKey> {
    // Parse ANSI escape sequences into GameKey events
}
```

**Frame rate control:** The game loop must respect SSH bandwidth. Over a network:
- Limit to 20 FPS (vs 30 FPS local) to reduce bandwidth
- More aggressive diff rendering (higher threshold for "changed" cell)
- Consider: if rendering a frame takes too long to transmit, skip frames

**Graceful disconnect:**
- Client disconnects → clean up game state, decrement session counter
- Server shutdown → send "Server shutting down" to all clients, clean up
- Game over → show stats, then prompt "Press any key to disconnect"

### Configuration:

```rust
struct ServerConfig {
    listen_addr: String,       // "0.0.0.0"
    port: u16,                 // 2222 (or 22 if running as root)
    max_sessions: usize,       // 50
    host_key_path: String,     // "./cryptfall_host_key"
    fps_ssh: u32,              // 20
    enable_sound: bool,        // false (SSH can't do sound)
    save_dir: String,          // "./saves/"
}
```

### Generate host key on first run:
If host key file doesn't exist, generate one:
```rust
let key = russh_keys::key::KeyPair::generate_ed25519().unwrap();
// Save to file
```

### Test:
- Run the server locally: `cargo run --bin server -- --port 2222`
- Connect: `ssh localhost -p 2222`
- Verify: welcome banner appears, game starts, input works
- Verify: disconnecting cleans up properly
- Verify: two simultaneous connections work independently
- Verify: terminal too small → shows error message
```

### Success Criteria
- [ ] SSH server accepts connections and starts game
- [ ] Input works over SSH (movement, attack, dash)
- [ ] Rendering is correct over SSH (no artifacts)
- [ ] Multiple concurrent sessions work independently
- [ ] Disconnect is handled cleanly
- [ ] Too-small terminal shows helpful error
- [ ] Host key is auto-generated on first run

---

## Session 5.2 — SSH Polish & Save System

### Claude Code Prompt

```
You are the Infrastructure Engineer for Cryptfall. SSH basics work. Now add save persistence and session polish.

### Per-player save data via SSH key fingerprint:

When a player connects via SSH, identify them by their public key fingerprint:

```rust
// In the SSH auth handler:
fn auth_publickey(self, user: &str, key: &PublicKey) -> Result<Auth> {
    let fingerprint = key.fingerprint();
    // Store fingerprint for this session
    // Load save data from saves/{fingerprint}.json
    Ok(Auth::Accept)
}

// Also accept password auth (for players without keys):
fn auth_password(self, user: &str, password: &str) -> Result<Auth> {
    // Accept any password, use username as identifier
    // Save to saves/user_{username}.json
    Ok(Auth::Accept)
}

// Also accept no auth:
fn auth_none(self, user: &str) -> Result<Auth> {
    // Generate a random session ID, no persistence
    Ok(Auth::Accept)
}
```

Save data structure:
- saves/{fingerprint}.json — for key-authenticated players
- saves/user_{username}.json — for password-authenticated players
- No save for anonymous connections (they can still play, just no persistence)

### Connection banner:

After authentication, show:
```
Welcome to CRYPTFALL!

Player: {fingerprint_short or username}
Runs: {total_runs}  |  Best Floor: {best_floor}  |  Gold: {total_gold}

Controls: Arrows=Move, Z=Attack, X=Dash, Esc=Pause, Q=Quit

Press any key to start...
```

### Network resilience:

1. **Output buffering:** Batch all frame output and write in one large chunk. If the write blocks or returns WouldBlock, skip the frame (drop it, render next frame instead).

2. **Input buffering:** Buffer received bytes and process them every game tick. Don't process input byte-by-byte.

3. **Timeout:** If no input received for 5 minutes, disconnect with message "Disconnected due to inactivity."

4. **Bandwidth monitoring:** Track bytes written per second. If exceeding 50KB/s, reduce FPS dynamically (drop to 15 FPS). Log bandwidth stats.

### Session management dashboard:

Create a simple admin interface (separate from game):
- `cargo run --bin server -- --admin`
- Shows: active sessions count, total bandwidth, per-session FPS

Or simpler: log session events to stdout:
```
[2026-02-18 10:23:15] Session started: user=alice, fingerprint=SHA256:abc...
[2026-02-18 10:23:15] Active sessions: 3/50
[2026-02-18 10:25:42] Session ended: user=alice, reason=quit, duration=2m27s
```

### Terminal capability detection:

On connection, detect what the terminal supports:
```rust
fn detect_capabilities(channel: &mut Channel) -> TermCapabilities {
    // 1. Terminal size: from PTY request
    // 2. True color: try sending a true color sequence and checking
    //    In practice, assume true color if terminal is modern enough to SSH
    // 3. Unicode: send a known-width Unicode test character, check cursor position
    //    In practice, assume UTF-8
    
    TermCapabilities {
        true_color: true,  // assume yes for SSH clients
        unicode: true,
        size: (cols, rows),
        kitty_keyboard: false,  // probably not over SSH
    }
}
```

### Test:
- Connect with SSH key → verify save persists between connections
- Connect with username/password → verify save persists by username
- Connect anonymously → verify no crash, no save
- Kill the SSH connection mid-game → verify server doesn't crash
- Connect 5 simultaneous sessions → verify they all work
- Leave a session idle for 6 minutes → verify timeout
- Check server logs for proper session tracking
```

### Success Criteria
- [ ] SSH key-authenticated players get persistent saves
- [ ] Password-authenticated players get persistent saves
- [ ] Anonymous players can play without saves
- [ ] Network resilience: dropped bytes don't crash server
- [ ] Inactivity timeout works
- [ ] Session logging is informative
- [ ] Multiple concurrent sessions remain stable

---

## Session 5.3 — CI/CD & Cross-Platform Builds

### Claude Code Prompt

```
You are the Infrastructure Engineer for Cryptfall. Build the CI/CD pipeline for automated testing and binary releases.

### Create .github/workflows/ci.yml

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all
      - run: cargo clippy --all -- -D warnings
      - run: cargo fmt --all -- --check

  build:
    needs: test
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            name: cryptfall-linux-x86_64
          - target: x86_64-apple-darwin
            os: macos-latest
            name: cryptfall-macos-x86_64
          - target: aarch64-apple-darwin
            os: macos-latest
            name: cryptfall-macos-aarch64
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            name: cryptfall-windows-x86_64.exe
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo build --release --target ${{ matrix.target }} --bin cryptfall
      - run: cargo build --release --target ${{ matrix.target }} --bin cryptfall-server
      - uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.name }}
          path: target/${{ matrix.target }}/release/cryptfall*
```

### Create .github/workflows/release.yml

Triggered on version tags (v*):

```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  release:
    # ... build all targets, create GitHub Release, attach binaries
```

### Create Dockerfile for SSH server:

```dockerfile
FROM rust:1.77-slim as builder
WORKDIR /build
COPY . .
RUN cargo build --release --bin cryptfall-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/cryptfall-server /usr/local/bin/
EXPOSE 2222
VOLUME ["/data/saves", "/data/keys"]
CMD ["cryptfall-server", "--port", "2222", "--save-dir", "/data/saves", "--key-path", "/data/keys/host_key"]
```

### Docker Compose for easy deployment:

```yaml
version: '3.8'
services:
  cryptfall:
    build: .
    ports:
      - "2222:2222"
    volumes:
      - cryptfall-saves:/data/saves
      - cryptfall-keys:/data/keys
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 512M
          cpus: '1.0'

volumes:
  cryptfall-saves:
  cryptfall-keys:
```

### Headless test framework:

Create a basic test that runs the game without a real terminal:

```rust
// tests/headless.rs
#[test]
fn test_game_starts_and_runs() {
    let mut fb = FrameBuffer::new(80, 50);
    let mut game = GameInstance::new_headless();
    
    // Simulate 30 frames
    for _ in 0..30 {
        game.update(&InputState::empty(), 1.0 / 30.0);
        game.render(&mut fb, 1.0);
    }
    
    // Verify: framebuffer has non-empty content
    assert!(fb.pixels.iter().any(|p| p.is_some()));
}

#[test]
fn test_floor_generation() {
    for seed in 0..100 {
        let floor = generate_floor(1, seed);
        assert!(floor.rooms.len() >= 6);
        assert!(floor.rooms.iter().any(|r| r.room_type == RoomType::Start));
        assert!(floor.rooms.iter().any(|r| r.room_type == RoomType::Boss));
        assert!(floor.rooms.iter().any(|r| r.room_type == RoomType::Exit));
        // Verify connectivity...
    }
}
```

### Cargo workspace organization:

Ensure the workspace is clean:
```toml
[workspace]
members = ["crates/engine", "crates/game", "crates/server"]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
```

### Test:
- `cargo test --all` passes
- `cargo clippy --all` has no warnings
- Docker build succeeds
- Docker container starts and accepts SSH connections
- Cross-compilation works (at least verify the CI config)
```

### Success Criteria
- [ ] CI runs tests, clippy, and fmt on every push
- [ ] Release workflow produces binaries for Linux, macOS (both archs), Windows
- [ ] Docker container builds and runs
- [ ] Headless tests pass
- [ ] No clippy warnings

---

## Session 5.4 — README & Launch Materials

### Claude Code Prompt

```
You are the Visual Designer and QA Lead for Cryptfall. This is the most important marketing session. Create the README and capture the GIF.

### The GIF is everything.

Terminal game virality lives and dies by the first 3 lines of the README. The GIF must show:
- Game running in a terminal (real terminal chrome visible = authenticity)
- Player sprite walking through a tiled dungeon
- Real-time combat: dashing, attacking, enemies dying with particles
- Screen shake, hit sparks, damage numbers
- All in half-block pixel art with true color

### How to capture the GIF:

Option 1 (recommended): Use `vhs` by Charm (https://github.com/charmbracelet/vhs)
- Create a .tape file that scripts the game
- Or: record manually with `vhs record`

Option 2: Use `asciinema` + `agg` (asciinema gif generator)
- `asciinema rec demo.cast`
- Play through an exciting 15-second combat sequence
- `agg demo.cast demo.gif --cols 80 --rows 50`

Option 3: Screen record + ffmpeg to GIF
- Record terminal with screen capture
- `ffmpeg -i recording.mp4 -vf "fps=15,scale=800:-1" demo.gif`

### GIF content script (15-20 seconds):

1. (0-3s) Player walks into a combat room, doors seal
2. (3-8s) Fights 3 skeletons: dash through one, attack two, particles flying
3. (8-12s) Ghost mage shoots projectiles, player dodges, counter-attacks
4. (12-15s) Last enemy dies with big particle explosion, doors open
5. (15-18s) Boon selection appears, player picks one
6. (18-20s) Player walks to next room

The GIF should be 800px wide, optimized under 5MB for GitHub.

### README.md structure:

```markdown
# ⚔️ Cryptfall

> A Hades-style roguelike that runs in your terminal.

[demo GIF here — FIRST THING, above the fold]

**Play instantly — no install required:**
```bash
ssh play.cryptfall.dev
```

**Or download:**
```bash
# macOS
brew install cryptfall

# Linux
curl -L https://github.com/you/cryptfall/releases/latest/download/cryptfall-linux-x86_64 -o cryptfall && chmod +x cryptfall

# Windows
scoop install cryptfall
```

## Features

- ⚔️ Real-time combat with dash, dodge, and three weapon types
- 🎨 Half-block pixel art with true color rendering
- 🏰 Procedurally generated dungeons with boss fights
- ⚡ 30+ boons and upgrades for unique build variety
- 🔑 SSH playable — zero install, works in any modern terminal
- 🎮 Built in Rust for buttery smooth 30fps terminal rendering

## Controls

| Key | Action |
|-----|--------|
| Arrow Keys / WASD | Move |
| Z / Enter | Attack |
| X / Space | Dash |
| Escape | Pause |
| Tab | Toggle minimap |

## Screenshots

[2-3 more GIFs or screenshots showing: boss fight, boon selection, title screen]

## Building from source

```bash
git clone https://github.com/you/cryptfall
cd cryptfall
cargo build --release
./target/release/cryptfall
```

## Terminal compatibility

Works best in: Kitty, Alacritty, iTerm2, WezTerm, Windows Terminal
Works in: most terminals with true color and Unicode support
Minimum: 80×40 terminal size

## How it works

Cryptfall renders game graphics using Unicode half-block characters (▄) with 24-bit true color. Each terminal cell becomes two vertical pixels, giving 80×100 pixel resolution in a standard 80×50 terminal. Differential rendering and synchronized output protocol ensure flicker-free animation at 30 FPS.

[Link to blog post: "How I Built a Real-Time Game Engine for the Terminal"]

## License

MIT
```

### Additional launch materials:

1. **CONTRIBUTING.md** — how to contribute (issue templates, PR guidelines)
2. **CHANGELOG.md** — version history starting with v0.1.0
3. **.github/ISSUE_TEMPLATE/** — bug report and feature request templates

### Test:
- README renders correctly on GitHub (preview with grip or similar)
- GIF is under 5MB and loops well
- SSH command works (or placeholder is clear)
- All links work
- Build instructions are correct
```

### Success Criteria
- [ ] GIF captures 15-20 seconds of exciting gameplay
- [ ] GIF is under 5MB and looks crisp
- [ ] README has play command in first 5 lines
- [ ] README is complete with features, controls, compatibility
- [ ] Building from source instructions verified

---

## Session 5.5 — Deployment & Launch Checklist

### Claude Code Prompt

```
You are the Infrastructure Engineer and QA Lead for Cryptfall. Final pre-launch checklist.

### Deployment:

1. Deploy SSH server to a cloud instance:
   - Recommended: DigitalOcean $6/mo droplet (1 vCPU, 1GB RAM) or equivalent
   - Or: any VPS with Docker support
   - Set up domain: play.cryptfall.dev → server IP
   - Open port 2222 (or 22 if you want the cleanest UX)
   - Deploy via Docker Compose
   - Test SSH from external network

2. Set up monitoring:
   - Server health: uptime, memory, CPU
   - Session metrics: connections per hour, average session duration
   - Error logging: capture panics, disconnection reasons

### Terminal compatibility testing matrix:

Test the game (both local and SSH) on:

| Terminal | OS | True Color | Unicode | Half-Block | Status |
|----------|----|------------|---------|------------|--------|
| Kitty | Linux/macOS | ✅ | ✅ | ✅ | |
| Alacritty | Linux/macOS | ✅ | ✅ | ✅ | |
| iTerm2 | macOS | ✅ | ✅ | ✅ | |
| WezTerm | All | ✅ | ✅ | ✅ | |
| Windows Terminal | Windows | ✅ | ✅ | ✅ | |
| macOS Terminal.app | macOS | ⚠️ | ✅ | ✅ | |
| GNOME Terminal | Linux | ✅ | ✅ | ✅ | |
| tmux | All | ✅ | ✅ | ✅ | |
| VS Code Terminal | All | ✅ | ✅ | ✅ | |

For each: verify rendering, input, performance, and colors.
Document any issues in docs/terminal-compatibility.md.

### Pre-launch checklist:

**Code:**
- [ ] `cargo test --all` passes
- [ ] `cargo clippy --all` clean  
- [ ] No TODO/FIXME in shipped code (or they're tracked in issues)
- [ ] Panic handler works (terminal restored on crash)
- [ ] Save file versioning (handle future format changes)

**Game:**
- [ ] 5+ successful runs from start to floor 3
- [ ] All 3 weapons viable
- [ ] No game-breaking bugs
- [ ] Boss is beatable
- [ ] Meta-progression feels rewarding over 5 runs

**Distribution:**
- [ ] GitHub Release with binaries for all platforms
- [ ] SSH server running and accepting connections
- [ ] README GIF captured and embedded
- [ ] LICENSE file present (MIT)

**Launch posts draft:**
Prepare (but don't post yet) submissions for:
- Hacker News: "Show HN: Cryptfall – A Hades-style roguelike that runs in your terminal"
- r/rust: "I built a real-time terminal roguelike in Rust with half-block pixel rendering"
- r/roguelikedev: "Sharing Saturday – Cryptfall: real-time combat roguelike rendered in Unicode half-blocks"
- r/programming: "Play a Hades-style roguelike by typing: ssh play.cryptfall.dev"

Each post should lead with the GIF or SSH command. The HN title should include "Show HN" and the SSH command.

### Final test:
- Ask 2-3 friends to SSH in and play without any instructions beyond the README
- Watch for confusion points
- Note: did they understand the controls? Did they die on floor 1? Did they want to play again?
```

### Success Criteria
- [ ] SSH server deployed and accessible from the internet
- [ ] Game tested on 8+ terminal emulators
- [ ] All platform binaries built and released
- [ ] 3+ external playtesters have tried the game
- [ ] Launch posts drafted
- [ ] Terminal compatibility document written
- [ ] READY TO LAUNCH

---

## Phase 5 File Manifest

```
crates/server/
├── Cargo.toml
└── src/
    ├── main.rs          # SSH server entry point
    ├── session.rs       # Per-player session management
    ├── input_parser.rs  # ANSI escape → GameKey
    ├── bandwidth.rs     # Output rate limiting
    └── config.rs        # Server configuration

.github/
├── workflows/
│   ├── ci.yml           # Test + lint on every push
│   └── release.yml      # Build + publish binaries on tag
├── ISSUE_TEMPLATE/
│   ├── bug_report.md
│   └── feature_request.md

Dockerfile
docker-compose.yml
README.md
CONTRIBUTING.md
CHANGELOG.md
LICENSE

docs/
├── terminal-compatibility.md
├── deployment.md
└── launch-posts.md      # Draft social media posts
```
