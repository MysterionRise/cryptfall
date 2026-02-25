pub mod effects;
pub mod selection;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BoonId {
    // Offense
    SharpenedBlade,
    BerserkersRage,
    SwiftStrikes,
    KillingBlow,
    ChainLightning,
    ProjectileSlash,
    CriticalEdge,
    Fury,
    // Defense
    ToughSkin,
    IronShield,
    LifeSteal,
    VampiricTouch,
    Retaliation,
    SecondWind,
    // Mobility
    SwiftFeet,
    PhantomDash,
    ShadowStep,
    DashStrike,
    // Special
    GoldMagnet,
    Lucky,
    TreasureSense,
    DeathsBargain,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BoonCategory {
    Offense,
    Defense,
    Mobility,
    Special,
}

pub struct BoonDef {
    pub id: BoonId,
    pub name: &'static str,
    #[allow(dead_code)] // Will be rendered in boon select card descriptions
    pub description: &'static str,
    pub rarity: Rarity,
    pub category: BoonCategory,
    pub stackable: bool,
}

pub const BOON_DEFS: &[BoonDef] = &[
    // --- Offense ---
    BoonDef {
        id: BoonId::SharpenedBlade,
        name: "Sharpened Blade",
        description: "+1 attack damage",
        rarity: Rarity::Common,
        category: BoonCategory::Offense,
        stackable: true,
    },
    BoonDef {
        id: BoonId::BerserkersRage,
        name: "Berserker's Rage",
        description: "+25% damage multiplier",
        rarity: Rarity::Rare,
        category: BoonCategory::Offense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::SwiftStrikes,
        name: "Swift Strikes",
        description: "30% faster attack speed",
        rarity: Rarity::Rare,
        category: BoonCategory::Offense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::KillingBlow,
        name: "Killing Blow",
        description: "Enemies explode on death for 2 damage",
        rarity: Rarity::Rare,
        category: BoonCategory::Offense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::ChainLightning,
        name: "Chain Lightning",
        description: "Attacks chain to 2 nearby enemies",
        rarity: Rarity::Legendary,
        category: BoonCategory::Offense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::ProjectileSlash,
        name: "Projectile Slash",
        description: "Attacks launch a projectile",
        rarity: Rarity::Legendary,
        category: BoonCategory::Offense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::CriticalEdge,
        name: "Critical Edge",
        description: "20% chance for double damage",
        rarity: Rarity::Rare,
        category: BoonCategory::Offense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::Fury,
        name: "Fury",
        description: "+5% damage per kill this room",
        rarity: Rarity::Legendary,
        category: BoonCategory::Offense,
        stackable: false,
    },
    // --- Defense ---
    BoonDef {
        id: BoonId::ToughSkin,
        name: "Tough Skin",
        description: "+1 max HP",
        rarity: Rarity::Common,
        category: BoonCategory::Defense,
        stackable: true,
    },
    BoonDef {
        id: BoonId::IronShield,
        name: "Iron Shield",
        description: "Block 2 hits per floor",
        rarity: Rarity::Rare,
        category: BoonCategory::Defense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::LifeSteal,
        name: "Life Steal",
        description: "Heal 15% of damage dealt",
        rarity: Rarity::Rare,
        category: BoonCategory::Defense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::VampiricTouch,
        name: "Vampiric Touch",
        description: "10% chance to heal on hit",
        rarity: Rarity::Common,
        category: BoonCategory::Defense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::Retaliation,
        name: "Retaliation",
        description: "Deal 1 damage to attackers",
        rarity: Rarity::Common,
        category: BoonCategory::Defense,
        stackable: false,
    },
    BoonDef {
        id: BoonId::SecondWind,
        name: "Second Wind",
        description: "Survive a killing blow once per floor",
        rarity: Rarity::Legendary,
        category: BoonCategory::Defense,
        stackable: false,
    },
    // --- Mobility ---
    BoonDef {
        id: BoonId::SwiftFeet,
        name: "Swift Feet",
        description: "+20% move speed",
        rarity: Rarity::Common,
        category: BoonCategory::Mobility,
        stackable: true,
    },
    BoonDef {
        id: BoonId::PhantomDash,
        name: "Phantom Dash",
        description: "+40% dash distance",
        rarity: Rarity::Rare,
        category: BoonCategory::Mobility,
        stackable: false,
    },
    BoonDef {
        id: BoonId::ShadowStep,
        name: "Shadow Step",
        description: "50% reduced dash cooldown",
        rarity: Rarity::Rare,
        category: BoonCategory::Mobility,
        stackable: false,
    },
    BoonDef {
        id: BoonId::DashStrike,
        name: "Dash Strike",
        description: "Deal 2 damage on dash",
        rarity: Rarity::Rare,
        category: BoonCategory::Mobility,
        stackable: false,
    },
    // --- Special ---
    BoonDef {
        id: BoonId::GoldMagnet,
        name: "Gold Magnet",
        description: "+50% gold earned",
        rarity: Rarity::Common,
        category: BoonCategory::Special,
        stackable: false,
    },
    BoonDef {
        id: BoonId::Lucky,
        name: "Lucky",
        description: "Better boon rarity odds",
        rarity: Rarity::Rare,
        category: BoonCategory::Special,
        stackable: false,
    },
    BoonDef {
        id: BoonId::TreasureSense,
        name: "Treasure Sense",
        description: "Reveal treasure rooms on minimap",
        rarity: Rarity::Common,
        category: BoonCategory::Special,
        stackable: false,
    },
    BoonDef {
        id: BoonId::DeathsBargain,
        name: "Death's Bargain",
        description: "+3 damage but reduce to 1 max HP",
        rarity: Rarity::Legendary,
        category: BoonCategory::Special,
        stackable: false,
    },
];

pub fn boon_def(id: BoonId) -> &'static BoonDef {
    BOON_DEFS.iter().find(|b| b.id == id).expect("missing boon definition")
}
