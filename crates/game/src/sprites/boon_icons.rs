#![allow(dead_code)]

use engine::color::Color;
use engine::sprite::SpriteData;

const N: Option<Color> = None;

// ============================================================================
// OFFENSE BOONS (red tones)
// ============================================================================

// Sharpened Blade colors
const SR: Option<Color> = Some([200, 50, 50]);   // red blade
const SS: Option<Color> = Some([180, 190, 200]);  // silver edge
const SH: Option<Color> = Some([230, 230, 240]);  // silver highlight
const SD: Option<Color> = Some([120, 30, 30]);    // dark red handle

/// Small sword icon — red blade with silver edge. 8x8.
#[rustfmt::skip]
pub static ICON_SHARPENED_BLADE: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N,  N,  N, SH,  N,
    N,  N,  N,  N,  N, SH, SS,  N,
    N,  N,  N,  N, SS, SR,  N,  N,
    N,  N,  N, SR, SR,  N,  N,  N,
    N,  N, SR, SR,  N,  N,  N,  N,
    N, SD, SR,  N,  N,  N,  N,  N,
    SD, SD, SD,  N,  N,  N,  N,  N,
    N, SD,  N,  N,  N,  N,  N,  N,
]);

// Berserker's Rage colors
const BR: Option<Color> = Some([180, 20, 20]);    // dark red
const BL: Option<Color> = Some([230, 50, 30]);    // bright angry red
const BW: Option<Color> = Some([255, 200, 180]);  // highlight

/// Angry fist icon — clenched fist, dark red. 8x8.
#[rustfmt::skip]
pub static ICON_BERSERKERS_RAGE: SpriteData = SpriteData::new(8, 8, &[
    N,  N, BL, BL, BL, BL,  N,  N,
    N, BL, BW, BL, BW, BL, BL,  N,
    N, BL, BL, BL, BL, BL, BL,  N,
    N, BL, BL, BL, BL, BL, BL,  N,
    N,  N, BR, BR, BR, BR,  N,  N,
    N,  N, BR, BR, BR, BR,  N,  N,
    N,  N, BR, BR, BR, BR,  N,  N,
    N,  N,  N, BR, BR,  N,  N,  N,
]);

// Swift Strikes colors
const QR: Option<Color> = Some([220, 80, 40]);    // red-orange
const QO: Option<Color> = Some([255, 140, 50]);   // orange
const QY: Option<Color> = Some([255, 200, 80]);   // yellow tip

/// Double speed arrows — red/orange. 8x8.
#[rustfmt::skip]
pub static ICON_SWIFT_STRIKES: SpriteData = SpriteData::new(8, 8, &[
    N,  N, QY,  N,  N, QY,  N,  N,
    N, QO, QO,  N, QO, QO,  N,  N,
    QR, QO,  N,  N, QO,  N,  N,  N,
    N, QR,  N,  N, QR,  N,  N,  N,
    N, QR,  N,  N, QR,  N,  N,  N,
    N,  N, QR,  N,  N, QR,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Killing Blow colors
const KR: Option<Color> = Some([220, 40, 40]);    // red
const KY: Option<Color> = Some([255, 220, 60]);   // yellow
const KO: Option<Color> = Some([255, 140, 30]);   // orange

/// Explosion symbol — red/yellow starburst. 8x8.
#[rustfmt::skip]
pub static ICON_KILLING_BLOW: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N, KR,  N,  N,  N,  N,
    N, KR,  N, KO,  N, KR,  N,  N,
    N,  N, KO, KY, KO,  N,  N,  N,
    KR, KO, KY, KY, KY, KO, KR,  N,
    N,  N, KO, KY, KO,  N,  N,  N,
    N, KR,  N, KO,  N, KR,  N,  N,
    N,  N,  N, KR,  N,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Chain Lightning colors
const LY: Option<Color> = Some([255, 255, 100]);  // bright yellow
const LW: Option<Color> = Some([255, 255, 220]);  // white-yellow
const LB: Option<Color> = Some([200, 200, 60]);   // dim yellow

/// Lightning bolt — yellow/white zigzag. 8x8.
#[rustfmt::skip]
pub static ICON_CHAIN_LIGHTNING: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N, LW, LY,  N,  N,
    N,  N,  N, LY, LY,  N,  N,  N,
    N,  N, LY, LW,  N,  N,  N,  N,
    N, LY, LW, LY, LY, LY,  N,  N,
    N,  N,  N,  N, LW, LY,  N,  N,
    N,  N,  N, LY, LB,  N,  N,  N,
    N,  N, LB, LY,  N,  N,  N,  N,
    N, LB,  N,  N,  N,  N,  N,  N,
]);

// Projectile Slash colors
const PR: Option<Color> = Some([200, 50, 50]);    // red arc
const PC: Option<Color> = Some([60, 200, 220]);   // cyan projectile
const PW: Option<Color> = Some([220, 220, 255]);  // white flash

/// Slash arc with projectile — red/cyan. 8x8.
#[rustfmt::skip]
pub static ICON_PROJECTILE_SLASH: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N,  N, PR, PR,  N,
    N,  N,  N,  N, PR, PW,  N,  N,
    N,  N,  N, PR, PW,  N,  N,  N,
    N,  N, PR, PW,  N,  N,  N,  N,
    N, PR, PW,  N,  N,  N,  N,  N,
    N,  N,  N,  N, PC, PW,  N,  N,
    N,  N,  N, PC, PW, PC,  N,  N,
    N,  N,  N,  N, PC,  N,  N,  N,
]);

// Critical Edge colors
const CR: Option<Color> = Some([220, 40, 40]);    // red
const CW: Option<Color> = Some([255, 255, 255]);  // white
const CY: Option<Color> = Some([255, 220, 100]);  // yellow

/// Exclamation starburst — red/white. 8x8.
#[rustfmt::skip]
pub static ICON_CRITICAL_EDGE: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N, CR,  N,  N,  N,  N,
    N, CR,  N, CW,  N, CR,  N,  N,
    N,  N, CY, CW, CY,  N,  N,  N,
    CR, CW, CW, CY, CW, CW, CR,  N,
    N,  N, CY, CW, CY,  N,  N,  N,
    N, CR,  N, CW,  N, CR,  N,  N,
    N,  N,  N, CR,  N,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Fury colors
const FR: Option<Color> = Some([220, 60, 20]);    // red-orange
const FO: Option<Color> = Some([255, 140, 30]);   // orange
const FY: Option<Color> = Some([255, 200, 60]);   // yellow tip

/// Rising flame / meter — red/orange. 8x8.
#[rustfmt::skip]
pub static ICON_FURY: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N, FY,  N,  N,  N,  N,
    N,  N, FY, FO,  N,  N,  N,  N,
    N,  N, FO, FO, FY,  N,  N,  N,
    N, FO, FR, FO, FO,  N,  N,  N,
    N, FR, FR, FR, FO,  N,  N,  N,
    N, FR, FR, FR, FR,  N,  N,  N,
    FR, FR, FR, FR, FR, FR,  N,  N,
    FR, FR, FR, FR, FR, FR,  N,  N,
]);

// ============================================================================
// DEFENSE BOONS (blue tones)
// ============================================================================

// Tough Skin colors
const TH: Option<Color> = Some([220, 40, 40]);    // red heart
const TB: Option<Color> = Some([60, 120, 200]);   // blue border
const TW: Option<Color> = Some([200, 220, 255]);  // white plus

/// Heart with + sign — blue/red. 8x8.
#[rustfmt::skip]
pub static ICON_TOUGH_SKIN: SpriteData = SpriteData::new(8, 8, &[
    N, TH, TH, N,  TH, TH,  N,  N,
    TH, TH, TH, TH, TH, TH, TH,  N,
    TH, TH, TW, TW, TW, TH, TH,  N,
    TH, TH, TW, TW, TW, TH, TH,  N,
    N,  TH, TH, TW, TH, TH,  N,  N,
    N,  N,  TB, TH, TB,  N,  N,  N,
    N,  N,  N,  TB,  N,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Iron Shield colors
const IB: Option<Color> = Some([70, 100, 180]);   // blue body
const IS: Option<Color> = Some([160, 170, 190]);  // silver trim
const IH: Option<Color> = Some([200, 210, 230]);  // highlight
const ID: Option<Color> = Some([40, 60, 120]);    // dark blue

/// Shield shape — blue/silver. 8x8.
#[rustfmt::skip]
pub static ICON_IRON_SHIELD: SpriteData = SpriteData::new(8, 8, &[
    N,  IS, IS, IS, IS, IS,  N,  N,
    IS, IH, IB, IB, IB, IH, IS,  N,
    IS, IB, IB, IH, IB, IB, IS,  N,
    IS, IB, IB, IH, IB, IB, IS,  N,
    IS, IB, IB, IH, IB, IB, IS,  N,
    N,  IS, IB, IB, IB, IS,  N,  N,
    N,  N,  ID, IB, ID,  N,  N,  N,
    N,  N,  N,  ID,  N,  N,  N,  N,
]);

// Life Steal colors
const VP: Option<Color> = Some([150, 40, 160]);   // purple
const VR: Option<Color> = Some([200, 40, 50]);    // red
const VW: Option<Color> = Some([220, 180, 220]);  // light purple

/// Fang/heart — purple/red. 8x8.
#[rustfmt::skip]
pub static ICON_LIFE_STEAL: SpriteData = SpriteData::new(8, 8, &[
    N,  VR, VR,  N, VR, VR,  N,  N,
    VR, VR, VR, VR, VR, VR, VR,  N,
    VR, VR, VR, VR, VR, VR, VR,  N,
    N,  VP, VR, VR, VR, VP,  N,  N,
    N,  N,  VP, VR, VP,  N,  N,  N,
    N, VP,  N,  VP,  N, VP,  N,  N,
    VP, VW,  N,  N,  N, VW, VP,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Vampiric Touch colors
const VA: Option<Color> = Some([140, 40, 160]);   // purple hand
const VG: Option<Color> = Some([80, 200, 80]);    // green sparkle
const VL: Option<Color> = Some([180, 140, 200]);  // light purple

/// Hand with sparkle — purple/green. 8x8.
#[rustfmt::skip]
pub static ICON_VAMPIRIC_TOUCH: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N,  N,  N, VG,  N,
    N, VA, VA, VA, VA,  N,  N, VG,
    N, VA, VL, VL, VL, VA,  N,  N,
    N, VA, VL, VA, VL, VA,  N,  N,
    N, VA, VL, VL, VL, VA, VG,  N,
    N,  N, VA, VA, VA,  N,  N,  N,
    N,  N,  N, VA,  N,  N,  N,  N,
    N,  N, VA, VA, VA,  N,  N,  N,
]);

// Retaliation colors
const RB: Option<Color> = Some([80, 120, 200]);   // blue base
const RW: Option<Color> = Some([220, 230, 255]);  // white spike
const RD: Option<Color> = Some([50, 70, 140]);    // dark blue

/// Thorns/spikes radiating — blue/white. 8x8.
#[rustfmt::skip]
pub static ICON_RETALIATION: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N, RW,  N,  N,  N,  N,
    N, RW,  N, RB,  N, RW,  N,  N,
    N,  N, RB, RB, RB,  N,  N,  N,
    RW, RB, RB, RD, RB, RB, RW,  N,
    N,  N, RB, RB, RB,  N,  N,  N,
    N, RW,  N, RB,  N, RW,  N,  N,
    N,  N,  N, RW,  N,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Second Wind colors
const WB: Option<Color> = Some([100, 160, 230]);  // blue
const WW: Option<Color> = Some([200, 220, 255]);  // white
const WL: Option<Color> = Some([150, 190, 240]);  // light blue

/// Wind swirl — blue/white. 8x8.
#[rustfmt::skip]
pub static ICON_SECOND_WIND: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N, WW, WW, WW,  N,  N,
    N,  N, WW, WL,  N,  N, WW,  N,
    N, WB,  N,  N,  N,  N, WL,  N,
    WB, WL, WB,  N,  N, WB,  N,  N,
    N,  N,  N, WB,  N,  N, WB,  N,
    N,  N,  N,  N, WL,  N, WL,  N,
    N,  N,  N, WB, WL, WB,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// ============================================================================
// MOBILITY BOONS (green tones)
// ============================================================================

// Swift Feet colors
const MG: Option<Color> = Some([60, 180, 60]);    // green
const MD: Option<Color> = Some([30, 120, 30]);    // dark green
const MH: Option<Color> = Some([120, 220, 120]);  // highlight green

/// Boot/foot icon — green. 8x8.
#[rustfmt::skip]
pub static ICON_SWIFT_FEET: SpriteData = SpriteData::new(8, 8, &[
    N,  N, MG, MG,  N,  N,  N,  N,
    N,  N, MG, MH,  N,  N,  N,  N,
    N,  N, MG, MG,  N,  N,  N,  N,
    N,  N, MD, MG,  N,  N,  N,  N,
    N,  N, MD, MG, MG,  N,  N,  N,
    N,  N, MD, MG, MG, MG,  N,  N,
    N, MD, MG, MG, MG, MG, MG,  N,
    N, MD, MD, MD, MD, MD, MD,  N,
]);

// Phantom Dash colors
const PG: Option<Color> = Some([60, 200, 180]);   // cyan-green
const PD: Option<Color> = Some([30, 120, 100]);   // dark teal
const PH: Option<Color> = Some([140, 240, 220]);  // highlight

/// Dash trail — green/cyan streaks. 8x8.
#[rustfmt::skip]
pub static ICON_PHANTOM_DASH: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N,  N,  N,  N,  N,
    N,  N,  N,  N,  N, PH, PG,  N,
    N,  N,  N,  N, PG, PG, PH,  N,
    PD, PD, PG, PG, PG, PH,  N,  N,
    PD, PD, PG, PG, PG, PH,  N,  N,
    N,  N,  N,  N, PG, PG, PH,  N,
    N,  N,  N,  N,  N, PH, PG,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Shadow Step colors
const SG: Option<Color> = Some([30, 80, 30]);     // dark green
const SK: Option<Color> = Some([15, 15, 15]);     // near-black
const SL: Option<Color> = Some([50, 130, 50]);    // dim green outline

/// Shadow figure — dark green/black silhouette. 8x8.
#[rustfmt::skip]
pub static ICON_SHADOW_STEP: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N, SL, SL,  N,  N,  N,
    N,  N, SL, SK, SK, SL,  N,  N,
    N,  N,  N, SK, SK,  N,  N,  N,
    N,  N, SK, SK, SK, SK,  N,  N,
    N, SG, SK, SG, SG, SK, SG,  N,
    N,  N,  N, SK, SK,  N,  N,  N,
    N,  N, SK,  N,  N, SK,  N,  N,
    N, SG, SG,  N,  N, SG, SG,  N,
]);

// Dash Strike colors
const DG: Option<Color> = Some([60, 180, 60]);    // green boot
const DR: Option<Color> = Some([220, 60, 40]);    // red impact
const DY: Option<Color> = Some([255, 220, 80]);   // yellow star

/// Boot with impact star — green/red. 8x8.
#[rustfmt::skip]
pub static ICON_DASH_STRIKE: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N,  N, DY,  N,  N,
    N,  N, DG, DG,  N, DR, DY,  N,
    N,  N, DG, DG, DR, DY, DR,  N,
    N,  N, DG, DG, DY, DR,  N,  N,
    N,  N, DG, DG, DR,  N,  N,  N,
    N, DG, DG, DG, DG, DG,  N,  N,
    DG, DG, DG, DG, DG, DG,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// ============================================================================
// SPECIAL BOONS (gold/yellow tones)
// ============================================================================

// Gold Magnet colors
const GG: Option<Color> = Some([255, 200, 50]);   // gold
const GD: Option<Color> = Some([180, 140, 30]);   // dark gold
const GA: Option<Color> = Some([130, 130, 140]);  // gray metal

/// Magnet with gold glow — gold/gray. 8x8.
#[rustfmt::skip]
pub static ICON_GOLD_MAGNET: SpriteData = SpriteData::new(8, 8, &[
    N,  GA, GA, GA, GA, GA,  N,  N,
    GA, GG, GA,  N,  GA, GG, GA,  N,
    GA, GG, GA,  N,  GA, GG, GA,  N,
    GA, GG,  N,  N,  N,  GG, GA,  N,
    N,  GG, GD,  N,  GD, GG,  N,  N,
    N,  N,  GG, GD, GG,  N,  N,  N,
    N,  N,  N,  GG,  N,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Lucky colors
const UG: Option<Color> = Some([255, 210, 60]);   // gold
const UE: Option<Color> = Some([80, 180, 60]);    // green leaf
const UH: Option<Color> = Some([120, 220, 80]);   // highlight green

/// Four-leaf clover — gold/green. 8x8.
#[rustfmt::skip]
pub static ICON_LUCKY: SpriteData = SpriteData::new(8, 8, &[
    N,  N, UE, UE,  N, UE, UE,  N,
    N, UE, UH, UE, UE, UH, UE,  N,
    N, UE, UE, UE, UE, UE, UE,  N,
    N,  N, UE, UG, UG, UE,  N,  N,
    N, UE, UE, UG, UG, UE, UE,  N,
    N, UE, UH, UE, UE, UH, UE,  N,
    N,  N, UE, UE, UE, UE,  N,  N,
    N,  N,  N, UE,  N,  N,  N,  N,
]);

// Treasure Sense colors
const TG: Option<Color> = Some([255, 200, 50]);   // gold
const TE: Option<Color> = Some([60, 100, 180]);   // blue
const TI: Option<Color> = Some([255, 240, 120]);  // bright gold iris

/// Eye/compass icon — gold/blue. 8x8.
#[rustfmt::skip]
pub static ICON_TREASURE_SENSE: SpriteData = SpriteData::new(8, 8, &[
    N,  N,  N,  N,  N,  N,  N,  N,
    N,  N, TG, TG, TG,  N,  N,  N,
    N, TG, TE, TE, TE, TG,  N,  N,
    TG, TE, TE, TI, TE, TE, TG,  N,
    TG, TE, TI, TG, TI, TE, TG,  N,
    N, TG, TE, TE, TE, TG,  N,  N,
    N,  N, TG, TG, TG,  N,  N,  N,
    N,  N,  N,  N,  N,  N,  N,  N,
]);

// Death's Bargain colors
const DK: Option<Color> = Some([200, 180, 50]);   // gold
const DD: Option<Color> = Some([120, 20, 20]);    // dark red
const DW: Option<Color> = Some([240, 220, 140]);  // pale gold

/// Skull icon — gold/dark red. 8x8.
#[rustfmt::skip]
pub static ICON_DEATHS_BARGAIN: SpriteData = SpriteData::new(8, 8, &[
    N,  N, DK, DK, DK, DK,  N,  N,
    N, DK, DW, DK, DK, DW, DK,  N,
    N, DK, DK, DK, DK, DK, DK,  N,
    N, DK, DD, DK, DD, DK, DK,  N,
    N, DK, DD, DK, DD, DK, DK,  N,
    N,  N, DK, DK, DK, DK,  N,  N,
    N,  N, DK, DD, DD, DK,  N,  N,
    N,  N,  N, DK, DK,  N,  N,  N,
]);
