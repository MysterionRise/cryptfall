# Phase 4 Balance Pass

Session 4.5 balance review. All values analyzed for weapon parity, boon impact, gold economy pacing, and floor difficulty scaling.

## Weapon DPS Analysis

| Weapon | Damage | Cooldown | Raw DPS | Hitbox Area | Hits to Kill Skeleton (3 HP) | Hits to Kill Ghost (2 HP) | Hits to Kill Boss (20 HP) |
|--------|--------|----------|---------|-------------|------------------------------|---------------------------|---------------------------|
| Sword | 2 | 0.35s | 5.71 | 96 sq px | 2 | 1 | 10 |
| Spear | 3 | 0.50s | 6.00 | 72 sq px | 1 | 1 | 7 |
| Daggers | 1 | 0.15s | 6.67 | 60 sq px | 3 | 2 | 20 |

**Assessment:** DPS spread is 5.71-6.67, a 17% range. This is healthy -- close enough that no weapon is strictly dominant, but each has distinct feel:
- **Sword** is the generalist: medium DPS, largest hitbox (easiest to land), medium knockback (60). Best for beginners.
- **Spear** has the best per-hit damage: one-shots skeletons, fewest boss hits. Narrow hitbox (4 px tall) rewards precise aim. Highest knockback (80) for safety.
- **Daggers** have the highest raw DPS but require sustained contact (3 hits per skeleton). Smallest hitbox, lowest knockback (30). Best with boons that scale on hit count (VampiricTouch, CriticalEdge, LifeSteal).

**Boon synergy:** Daggers benefit most from flat damage boons (+1 from SharpenedBlade takes them to 2 DPS = 13.3/s). Spear benefits most from multiplier boons (BerserkersRage takes it to 3.75 = 7.5/s). This creates interesting weapon-boon synergies.

**Verdict:** No changes needed. Weapon balance is sound.

## Boon Impact Analysis

### Offense Boons

| Boon | Rarity | Effect | DPS Impact (Sword baseline) | Notes |
|------|--------|--------|----------------------------|-------|
| SharpenedBlade | Common | +1 flat dmg | +50% (8.57/s) | Strong, scales best with Daggers. Stackable. |
| BerserkersRage | Rare | +25% mult | +25% (7.14/s) | Solid multiplier, scales with flat bonuses. |
| SwiftStrikes | Rare | 0.7x cooldown | +43% (8.16/s) | Very strong, multiplicative with damage. |
| CriticalEdge | Rare | 20% double dmg | +20% avg (6.86/s) | Consistent but not flashy. |
| KillingBlow | Rare | 2 dmg on death | Situational | AOE clear, better in dense rooms. |
| Fury | Legendary | +5%/kill/room | Scales up to +50%+ | Snowballs hard in long fights. |
| ChainLightning | Legendary | Chain to 2 | Up to 3x in groups | Legendary-tier AOE, very powerful. |
| ProjectileSlash | Legendary | Ranged attack | Extra damage source | Safe DPS option. |

**Assessment:** SwiftStrikes (+43%) is the strongest single Rare offense boon. Combined with SharpenedBlade, it pushes Sword to ~12.2/s which is powerful but not gamebreaking given it requires two boon slots. No auto-pick or never-pick issues.

### Defense Boons

| Boon | Rarity | Effect | Effective HP Gain | Notes |
|------|--------|--------|-------------------|-------|
| ToughSkin | Common | +1 max HP | +1 HP (20% of base) | Simple, always useful. Stackable. |
| IronShield | Rare | Block 2/floor | +2 effective HP/floor | Resets per floor, very strong on later floors. |
| LifeSteal | Rare | Heal 15% dealt | Varies with DPS | Better with high-damage weapons. |
| VampiricTouch | Common | 10% heal chance | ~0.57 HP/sec (Sword) | Weaker but Common, so seen more often. |
| Retaliation | Common | 1 dmg to attackers | Situational | Niche, punishes melee enemies. |
| SecondWind | Legendary | Survive lethal | +1 life/floor | Emergency safety net, very strong. |

**Assessment:** VampiricTouch at 10% chance is notably weaker than LifeSteal at 15% guaranteed. However, VampiricTouch is Common (shown 3x more often than Rare) and triggers independently, so it has appropriate power for its rarity tier. No adjustments needed.

### Mobility Boons

| Boon | Rarity | Effect | Notes |
|------|--------|--------|-------|
| SwiftFeet | Common | +20% move speed | Always useful for positioning. Stackable. |
| PhantomDash | Rare | +40% dash distance | Better escape/engage. |
| ShadowStep | Rare | 50% less dash CD | More frequent dashes. |
| DashStrike | Rare | 2 dmg on dash | Offensive dash, rewards aggressive play. |

**Assessment:** Mobility boons are well-differentiated. DashStrike creates an interesting aggressive playstyle. No issues.

### Special Boons

| Boon | Rarity | Effect | Notes |
|------|--------|--------|-------|
| GoldMagnet | Common | +50% gold | Meta-progression accelerator. |
| Lucky | Rare | Better rarity odds | Shifts weights from 60/30/10 to 40/40/20. Snowball boon. |
| TreasureSense | Common | Reveal treasure rooms | Minimap utility, weakest boon for combat. |
| DeathsBargain | Legendary | +3 dmg, 1 max HP | Glass cannon. Sword goes to 14.3/s but dies in one hit. |

**Assessment:** Death's Bargain is the highest-risk/reward boon in the game. With 1 HP, any hit kills unless SecondWind or IronShield is also held. This creates interesting build paths (Death's Bargain + SecondWind + IronShield = glass cannon with safety nets). TreasureSense is the weakest combat boon but provides exploration value.

**Verdict:** No boon changes needed. No auto-picks or never-picks detected.

## Gold Economy Analysis

### Per-Run Gold Income Estimate

Assumptions: ~60% of rooms are combat, player clears all rooms on each floor.

| Floor | Rooms | Combat Rooms | Enemies/Room (avg) | Kill Gold | Clear Gold | Floor Total |
|-------|-------|-------------|-------------------|-----------|------------|-------------|
| 1 | 6-10 | 3-5 | 2-3 | ~14G | ~20G | ~34G |
| 2 | 7-11 | 4-6 | 3-4 | ~22G | ~25G | ~47G |
| 3 | 8-12 | 5-7 | 4-5 | ~32G | ~30G | ~62G |
| 4 | 9-13 | 5-8 | 5-6 | ~42G | ~35G | ~77G |
| 5 | 10-14 | 6-8 | 5-7 + boss | ~55G | ~40G | ~95G |

**Estimated full run total: ~250-350G** (varies by room count and enemy mix)

### Upgrade Cost Analysis

| Upgrade | Cost | Runs to Afford (no deaths) | Priority |
|---------|------|---------------------------|----------|
| Vitality I | 30G | < 1 run | First buy, available mid-run 1 |
| Boon Reroll | 40G | < 1 run | High value, early buy |
| Strength I | 50G | < 1 run | Good early investment |
| Vitality II | 60G | < 1 run | Second run purchase |
| Twin Dash | 80G | ~1 run | Powerful mobility upgrade |
| Boon Reroll+ | 80G | ~1 run | Quality-of-life |
| Strength II | 100G | ~1 run | Mid-game power spike |
| Vitality III | 120G | ~1 run | Late-game tankiness |
| **Total** | **560G** | **~2-3 runs** | |

**Assessment:** Gold pacing is good. First upgrade affordable during or after first run. Full upgrade suite takes 2-3 complete runs. This matches the Hades model of gradual power accumulation across runs. GoldMagnet boon (+50%) can accelerate this meaningfully.

**Verdict:** Gold economy is well-paced. No changes needed.

## Floor Difficulty Scaling

### Encounter Scaling by Floor

Enemy counts scale via `floor_bonus_sk` and `floor_bonus_gh`:
- `floor_bonus_sk = min(floor - 1, 3)` -- skeleton count bonus caps at +3
- `floor_bonus_gh = min(floor - 2, 2)` -- ghosts appear from floor 2, cap at +2

| Floor | Easy Room | Medium Room (W1 + W2) | Hard Room (W1 + W2) |
|-------|-----------|----------------------|---------------------|
| 1 | 2 sk | 2 sk + (1 sk, 1 gh) | (3 sk, 1 gh) + (2 sk, 2 gh) |
| 2 | 3 sk | 3 sk + (1 sk, 1 gh) | (4 sk, 1 gh) + (3 sk, 2 gh) |
| 3 | 4 sk | 4 sk + (2 sk, 2 gh) | (5 sk, 2 gh) + (4 sk, 3 gh) |
| 4 | 5 sk | 5 sk + (2 sk, 2 gh) | (6 sk, 2 gh) + (5 sk, 3 gh) |
| 5 | 5 sk | 5 sk + (2 sk, 2 gh) | (6 sk, 2 gh) + (5 sk, 3 gh) + BOSS |

Note: actual counts are capped by `num_spawn_points` in each room template.

**Assessment:** Scaling is reasonable. Floor 1 is approachable (2-3 enemies per easy room). By floor 3-4, rooms have 6-10 total enemies across waves, which is challenging but manageable with boons. Ghost introduction on floor 2 adds ranged pressure. Boss on floor 5 (20 HP, 2 damage melee, charge + slam + sweep attacks) is a substantial challenge.

**Potential concern:** `enemy_count_mult` and `enemy_hp_mult` in FloorConfig are defined but not yet applied in the encounter system. Currently only `floor_bonus_sk`/`floor_bonus_gh` affect difficulty. These multipliers should be wired in for finer tuning in future phases.

**Verdict:** Floor scaling is functional and reasonably paced. No immediate changes needed.

## Summary

| Category | Status | Notes |
|----------|--------|-------|
| Weapon DPS | Good | 5.71-6.67 range (17% spread), each weapon has clear identity |
| Boon impact | Good | No auto-picks or never-picks, interesting build synergies |
| Gold economy | Good | First upgrade in ~1 run, full set in ~2-3 runs |
| Floor scaling | Good | Gradual ramp, ghosts from floor 2, boss on floor 5 |
| Upgrade pacing | Good | Matches Hades-style gradual power accumulation |

**No values changed.** All gameplay constants are within acceptable ranges for the current phase. Future tuning may adjust:
1. Wire `enemy_count_mult`/`enemy_hp_mult` into encounter generation for finer floor scaling
2. Add HP scaling for enemies on higher floors (currently all enemies have flat HP regardless of floor)
3. Tune boon offering frequency (currently every 2 combat rooms) based on playtest feedback
