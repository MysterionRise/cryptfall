# Tuning Values Reference

All gameplay constants organized by category. Adjust these to rebalance the game.

## Player

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| PLAYER_SPEED | 60.0 | Walking speed in pixels/sec | player.rs |
| DASH_SPEED | 200.0 | Dash speed in pixels/sec | player.rs |
| DASH_DURATION | 0.15 | Dash length in seconds | player.rs |
| ATTACK_COOLDOWN | 0.3 | Minimum time between attacks in seconds | player.rs |
| ATTACK_ACTIVE_FRAME | 2 | Animation frame index where attack hitbox is active | player.rs |
| PLAYER_KNOCKBACK_SPEED | 100.0 | Initial knockback velocity on player hit | player.rs |
| PLAYER_KNOCKBACK_FRICTION | 0.85 | Knockback decay per frame (lower = faster decay) | player.rs |
| DAMAGE_INVINCIBILITY | 1.0 | I-frame duration after taking damage in seconds | player.rs |
| HP (initial) | 5 | Player starting and max hit points | player.rs |
| COLLISION_W | 8.0 | Player foot collision box width | player.rs |
| COLLISION_H | 4.0 | Player foot collision box height | player.rs |
| COLLISION_OFFSET_X | 1.0 | Collision box X offset from sprite origin | player.rs |
| COLLISION_OFFSET_Y | 10.0 | Collision box Y offset from sprite origin | player.rs |
| PLAYER_HURTBOX | (2, 3, 6, 8) | Player hurtbox (x, y, w, h) relative to sprite | player.rs |
| ATTACK_HITBOX_RIGHT | (8, 3, 10, 8) | Right-facing attack hitbox relative to sprite | player.rs |
| ATTACK_HITBOX_LEFT | (-10, 3, 10, 8) | Left-facing attack hitbox relative to sprite | player.rs |

## Enemies - Common

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| FLASH_DURATION | 0.12 | White flash duration on hit in seconds | enemies/mod.rs |
| KNOCKBACK_SPEED | 120.0 | Initial knockback velocity on enemy hit | enemies/mod.rs |
| KNOCKBACK_FRICTION | 0.85 | Knockback decay per frame | enemies/mod.rs |
| STAGGER_DURATION | 0.2 | Stagger lock duration on hit in seconds | enemies/mod.rs |
| Slime HP | 3 | Slime starting hit points | enemies/mod.rs |
| Skeleton HP | 3 | Skeleton starting hit points | enemies/mod.rs |
| Ghost HP | 2 | Ghost starting hit points | enemies/mod.rs |
| Contact damage cooldown | 0.5 | Slime contact damage cooldown in seconds | enemies/mod.rs |

## Enemies - Skeleton

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| DETECT_RANGE | 80.0 | Distance to detect player and begin chase | enemies/skeleton.rs |
| ATTACK_RANGE | 20.0 | Distance to begin wind-up attack | enemies/skeleton.rs |
| PATROL_SPEED | 25.0 | Patrol movement speed in pixels/sec | enemies/skeleton.rs |
| CHASE_SPEED | 40.0 | Chase movement speed in pixels/sec | enemies/skeleton.rs |
| LUNGE_DISTANCE | 15.0 | Forward lunge distance during attack | enemies/skeleton.rs |
| WINDUP_DURATION | 0.4 | Telegraph time before attack in seconds | enemies/skeleton.rs |
| ATTACK_DURATION | 0.15 | Active attack swing duration in seconds | enemies/skeleton.rs |
| COOLDOWN_DURATION | 0.6 | Post-attack cooldown in seconds | enemies/skeleton.rs |
| STAGGER_DURATION | 0.3 | Skeleton-specific stagger lock in seconds | enemies/skeleton.rs |
| IDLE_MIN | 1.0 | Minimum idle time before patrolling | enemies/skeleton.rs |
| IDLE_MAX | 2.0 | Maximum idle time before patrolling | enemies/skeleton.rs |
| SKEL_ATTACK_HITBOX_RIGHT | (8, 3, 10, 8) | Right-facing skeleton attack hitbox | enemies/skeleton.rs |
| SKEL_ATTACK_HITBOX_LEFT | (-10, 3, 10, 8) | Left-facing skeleton attack hitbox | enemies/skeleton.rs |

## Enemies - Ghost

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| PREFERRED_DIST | 60.0 | Ideal distance ghost tries to maintain from player | enemies/ghost.rs |
| TOO_CLOSE | 40.0 | Distance that triggers retreat | enemies/ghost.rs |
| REPOSITION_SPEED | 35.0 | Retreat/reposition speed in pixels/sec | enemies/ghost.rs |
| AIM_DURATION | 0.6 | Time spent aiming before firing in seconds | enemies/ghost.rs |
| SHOOT_COOLDOWN | 1.2 | Cooldown between shots in seconds | enemies/ghost.rs |
| STAGGER_DURATION | 0.3 | Ghost-specific stagger lock in seconds | enemies/ghost.rs |
| AIM_CANCEL_DISTANCE | 28.0 | Player distance that cancels aiming | enemies/ghost.rs |
| MAX_AIM_RANGE | 120.0 | Maximum distance for aiming at player | enemies/ghost.rs |

## Projectiles

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| PROJECTILE_SPEED | 80.0 | Projectile travel speed in pixels/sec | projectile.rs |
| PROJECTILE_LIFETIME | 2.0 | Maximum projectile lifetime in seconds | projectile.rs |
| PROJECTILE_HITBOX | (0, 0, 3, 3) | Projectile collision box (x, y, w, h) | projectile.rs |
| TRAIL_INTERVAL | 0.05 | Time between trail particle spawns in seconds | projectile.rs |
| Projectile damage | 1 | Damage per projectile hit | projectile.rs |

## Waves

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| MAX_WAVES | 3 | Total number of waves in the arena | waves.rs |
| WAVE_TRANSITION_DELAY | 1.5 | Delay between wave clear and next spawn in seconds | waves.rs |
| WAVE_ANNOUNCE_DURATION | 2.0 | Duration of "WAVE N" text display in seconds | waves.rs |
| Wave 1 composition | 3 skeletons | Enemy types and count | waves.rs |
| Wave 2 composition | 2 skeletons + 1 ghost | Enemy types and count | waves.rs |
| Wave 3 composition | 4 skeletons + 2 ghosts | Enemy types and count | waves.rs |

## Visual Feedback

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| FLASH_FRAMES | 5 | Attack flash overlay duration in frames (at 30 FPS) | tuning.rs |
| DEMO_IDLE_THRESHOLD | 5.0 | Seconds of idle before demo mode activates | tuning.rs |
| DEATH_FADE_DURATION | 1.5 | Death fade-to-black duration in seconds | tuning.rs |
| DASH_TINT | [100, 160, 255] | Cool blue tint during dash i-frames | tuning.rs |
| ATTACK_TINT | [255, 80, 80] | Warm red tint during attack flash | tuning.rs |
| IFRAME_TINT | [255, 255, 255] | Bright white tint during i-frame flicker | tuning.rs |

## Combat Feedback

| Constant | Value | Description | File |
|----------|-------|-------------|------|
| Hit pause (normal hit) | 3 frames | Freeze frames on non-lethal enemy hit | combat.rs |
| Hit pause (kill) | 5 frames | Freeze frames on enemy kill | combat.rs |
| Hit pause (player hit) | 4 frames | Freeze frames when player takes damage | combat.rs |
| Hit pause (player death) | 8 frames | Freeze frames on player death | combat.rs |
| Camera shake (normal hit) | 2.5 | Shake intensity on non-lethal enemy hit | combat.rs |
| Camera shake (kill) | 5.0 | Shake intensity on enemy kill | combat.rs |
| Camera shake (player hit) | 4.0 | Shake intensity when player takes damage | combat.rs |
| Camera shake (player death) | 8.0 | Shake intensity on player death | combat.rs |
| Camera shake (attack swing) | 3.0 | Shake intensity on attack initiation | main.rs |
| Camera shake (dash) | 6.0 | Shake intensity on dash initiation | main.rs |
