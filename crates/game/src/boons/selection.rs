use super::{BoonCategory, BoonDef, BoonId, Rarity};

/// Simple seeded pseudo-random number generator (xorshift64).
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

}

/// Rarity weight tables.
fn rarity_weight(rarity: Rarity, lucky: bool) -> u32 {
    match (rarity, lucky) {
        (Rarity::Common, false) => 60,
        (Rarity::Rare, false) => 30,
        (Rarity::Legendary, false) => 10,
        (Rarity::Common, true) => 40,
        (Rarity::Rare, true) => 40,
        (Rarity::Legendary, true) => 20,
    }
}

/// Select 3 boon options for the player to choose from.
///
/// - Weights by rarity (Common 60/30/10, Lucky 40/40/20)
/// - Excludes non-stackable boons the player already has
/// - Tries to include at least 2 different categories
/// - Uses seeded RNG for determinism
pub fn select_boon_options<'a>(
    available: &'a [BoonDef],
    active: &[BoonId],
    lucky: bool,
    seed: u64,
) -> Vec<&'a BoonDef> {
    let mut rng = Rng::new(seed);

    // Filter out non-stackable boons the player already has
    let candidates: Vec<&BoonDef> = available
        .iter()
        .filter(|b| b.stackable || !active.contains(&b.id))
        .collect();

    if candidates.is_empty() {
        return Vec::new();
    }

    if candidates.len() <= 3 {
        return candidates;
    }

    // Build weighted list
    let weights: Vec<u32> = candidates
        .iter()
        .map(|b| rarity_weight(b.rarity, lucky))
        .collect();
    let total_weight: u32 = weights.iter().sum();

    if total_weight == 0 {
        return Vec::new();
    }

    let mut selected: Vec<&BoonDef> = Vec::with_capacity(3);
    let mut used_indices: Vec<usize> = Vec::with_capacity(3);

    // Pick 3 boons using weighted random selection
    for pick in 0..3 {
        // On the third pick, try to ensure category diversity
        let force_different_category = if pick == 2 && selected.len() == 2 {
            let cat0 = selected[0].category;
            let cat1 = selected[1].category;
            if cat0 == cat1 {
                Some(cat0)
            } else {
                None
            }
        } else {
            None
        };

        let chosen = if let Some(exclude_cat) = force_different_category {
            // Try to pick from a different category
            weighted_pick_excluding_category(
                &candidates,
                &weights,
                &used_indices,
                exclude_cat,
                &mut rng,
            )
            .or_else(|| {
                // Fallback: pick any remaining boon
                weighted_pick(&weights, &used_indices, &mut rng)
            })
        } else {
            weighted_pick(&weights, &used_indices, &mut rng)
        };

        if let Some(idx) = chosen {
            selected.push(candidates[idx]);
            used_indices.push(idx);
        }
    }

    selected
}

fn weighted_pick(
    weights: &[u32],
    excluded: &[usize],
    rng: &mut Rng,
) -> Option<usize> {
    let available_weight: u32 = weights
        .iter()
        .enumerate()
        .filter(|(i, _)| !excluded.contains(i))
        .map(|(_, w)| w)
        .sum();

    if available_weight == 0 {
        return None;
    }

    let roll = (rng.next() % available_weight as u64) as u32;
    let mut acc = 0;
    for (i, &w) in weights.iter().enumerate() {
        if excluded.contains(&i) {
            continue;
        }
        acc += w;
        if roll < acc {
            return Some(i);
        }
    }

    // Fallback: return last non-excluded
    weights
        .iter()
        .enumerate()
        .rev()
        .find(|(i, _)| !excluded.contains(i))
        .map(|(i, _)| i)
}

fn weighted_pick_excluding_category(
    candidates: &[&BoonDef],
    weights: &[u32],
    excluded: &[usize],
    exclude_cat: BoonCategory,
    rng: &mut Rng,
) -> Option<usize> {
    let available_weight: u32 = weights
        .iter()
        .enumerate()
        .filter(|(i, _)| !excluded.contains(i) && candidates[*i].category != exclude_cat)
        .map(|(_, w)| w)
        .sum();

    if available_weight == 0 {
        return None;
    }

    let roll = (rng.next() % available_weight as u64) as u32;
    let mut acc = 0;
    for (i, &w) in weights.iter().enumerate() {
        if excluded.contains(&i) || candidates[i].category == exclude_cat {
            continue;
        }
        acc += w;
        if roll < acc {
            return Some(i);
        }
    }

    // Fallback
    weights
        .iter()
        .enumerate()
        .rev()
        .find(|(i, _)| !excluded.contains(i) && candidates[*i].category != exclude_cat)
        .map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boons::BOON_DEFS;

    #[test]
    fn test_returns_3_options() {
        let result = select_boon_options(BOON_DEFS, &[], false, 42);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_no_duplicate_ids() {
        for seed in 1..100 {
            let result = select_boon_options(BOON_DEFS, &[], false, seed);
            assert_eq!(result.len(), 3);
            let ids: Vec<BoonId> = result.iter().map(|b| b.id).collect();
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    assert_ne!(ids[i], ids[j], "duplicate at seed {seed}");
                }
            }
        }
    }

    #[test]
    fn test_excludes_non_stackable_active() {
        let active = vec![BoonId::BerserkersRage, BoonId::Lucky];
        for seed in 1..50 {
            let result = select_boon_options(BOON_DEFS, &active, false, seed);
            assert_eq!(result.len(), 3);
            for b in &result {
                if !b.stackable {
                    assert!(
                        !active.contains(&b.id),
                        "offered non-stackable active boon {:?} at seed {seed}",
                        b.id
                    );
                }
            }
        }
    }

    #[test]
    fn test_stackable_boons_can_repeat() {
        // SharpenedBlade is stackable, so it should still be offered even if active
        let active = vec![BoonId::SharpenedBlade];
        let mut seen_sharpened = false;
        for seed in 1..200 {
            let result = select_boon_options(BOON_DEFS, &active, false, seed);
            if result.iter().any(|b| b.id == BoonId::SharpenedBlade) {
                seen_sharpened = true;
                break;
            }
        }
        assert!(seen_sharpened, "stackable boon should still be offered");
    }

    #[test]
    fn test_category_diversity() {
        let mut diverse_count = 0;
        let total = 200;
        for seed in 1..=total {
            let result = select_boon_options(BOON_DEFS, &[], false, seed);
            let cats: Vec<BoonCategory> = result.iter().map(|b| b.category).collect();
            let unique: std::collections::HashSet<_> = cats.iter().collect();
            if unique.len() >= 2 {
                diverse_count += 1;
            }
        }
        // At least 80% should have 2+ categories
        assert!(
            diverse_count > total * 80 / 100,
            "only {diverse_count}/{total} had 2+ categories"
        );
    }

    #[test]
    fn test_lucky_shifts_rarity() {
        let mut legendary_normal = 0;
        let mut legendary_lucky = 0;
        let trials = 1000;

        for seed in 1..=trials {
            let result = select_boon_options(BOON_DEFS, &[], false, seed);
            legendary_normal += result
                .iter()
                .filter(|b| b.rarity == Rarity::Legendary)
                .count();

            let result_lucky = select_boon_options(BOON_DEFS, &[], true, seed);
            legendary_lucky += result_lucky
                .iter()
                .filter(|b| b.rarity == Rarity::Legendary)
                .count();
        }

        assert!(
            legendary_lucky > legendary_normal,
            "lucky should produce more legendaries: normal={legendary_normal}, lucky={legendary_lucky}"
        );
    }

    #[test]
    fn test_deterministic_with_same_seed() {
        let r1 = select_boon_options(BOON_DEFS, &[], false, 12345);
        let r2 = select_boon_options(BOON_DEFS, &[], false, 12345);
        let ids1: Vec<BoonId> = r1.iter().map(|b| b.id).collect();
        let ids2: Vec<BoonId> = r2.iter().map(|b| b.id).collect();
        assert_eq!(ids1, ids2);
    }

    #[test]
    fn test_empty_available() {
        let result = select_boon_options(&[], &[], false, 42);
        assert!(result.is_empty());
    }

    #[test]
    fn test_fewer_than_3_available() {
        let small: Vec<BoonDef> = BOON_DEFS.iter().take(2).map(|b| BoonDef {
            id: b.id,
            name: b.name,
            description: b.description,
            rarity: b.rarity,
            category: b.category,
            stackable: b.stackable,
        }).collect();
        let result = select_boon_options(&small, &[], false, 42);
        assert_eq!(result.len(), 2);
    }
}
