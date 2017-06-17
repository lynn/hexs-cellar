use rand;
use rand::Rng;
use sprite::{Sprite};
use sprite::Color::*;

#[derive(Copy, Clone)]
pub enum Appearance {
    NoItem = 0x00,
    Crowbar = 0x01,
    VolcanicShard = 0x02,
    Taser = 0x03,
    JellyGun = 0x04,

    // Fruits group (shuffled)
    Lumimelon = 0x05,
    Glowfruit = 0x06,
    Shineapple = 0x07,

    // Pills group (shuffled)
    RoundPill = 0x08,
    TinyPill = 0x09,
    DiamondPill = 0x0a,
    OblongPill = 0x0b,
    SoftPill = 0x0c,
    HexagonalPill = 0x0d,
    WidePill = 0x0e,
    TranslucentPill = 0x0f,

    ThickSweater = 0x10,
    BallisticVest = 0x11,
    DragonScaleMail = 0x12,
    TitaniumNecklace = 0x13,
    RustyNecklace = 0x14,
    CrimsonNecklace = 0x15,
    GlowingNecklace = 0x16,
    UnholyNecklace = 0x17,
    Wand = 0x18,
    Manual = 0x19,
    Guidebook = 0x1a,

    // Devices group (shuffled)
    RedDevice = 0x1b,
    YellowDevice = 0x1c,
    BlueDevice = 0x1d,

    Palantir = 0x1e,
    GoldenPendant = 0x1f,
}

struct AppearanceInfo {
    name: &'static str,
    sprite: Sprite,
}

const WEAPON_CHAR: char = '/';
const FOOD_CHAR: char = '%';
const PILL_CHAR: char = '*';
const ARMOR_CHAR: char = '[';
const NECKLACE_CHAR: char = '"';
const DEVICE_CHAR: char = '&';
const ARTIFACT_CHAR: char = '$';

const APPEARANCE_INFOS: [AppearanceInfo; 32] = [
    AppearanceInfo {name: "(no item)",         sprite: Sprite {character: '?',           color: &[Red]}},
    AppearanceInfo {name: "crowbar",           sprite: Sprite {character: WEAPON_CHAR,   color: &[Teal]}},
    AppearanceInfo {name: "volcanic shard",    sprite: Sprite {character: WEAPON_CHAR,   color: &[Maroon]}},
    AppearanceInfo {name: "taser",             sprite: Sprite {character: WEAPON_CHAR,   color: &[Aqua]}},
    AppearanceInfo {name: "jelly gun",         sprite: Sprite {character: WEAPON_CHAR,   color: &[Lime]}},
    AppearanceInfo {name: "lumimelon",         sprite: Sprite {character: FOOD_CHAR,     color: &[Lime]}},
    AppearanceInfo {name: "glowfruit",         sprite: Sprite {character: FOOD_CHAR,     color: &[Pink]}},
    AppearanceInfo {name: "shineapple",        sprite: Sprite {character: FOOD_CHAR,     color: &[Yellow]}},
    AppearanceInfo {name: "round pill",        sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "tiny pill",         sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "diamond pill",      sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "oblong pill",       sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "soft pill",         sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "hexagonal pill",    sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "wide pill",         sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "translucent pill",  sprite: Sprite {character: PILL_CHAR,     color: &[White]}},
    AppearanceInfo {name: "thick sweater",     sprite: Sprite {character: ARMOR_CHAR,    color: &[Purple]}},
    AppearanceInfo {name: "ballistic vest",    sprite: Sprite {character: ARMOR_CHAR,    color: &[Teal]}},
    AppearanceInfo {name: "dragon scale mail", sprite: Sprite {character: ARMOR_CHAR,    color: &[Green]}},
    AppearanceInfo {name: "titanium necklace", sprite: Sprite {character: NECKLACE_CHAR, color: &[Gray]}},
    AppearanceInfo {name: "rusty necklace",    sprite: Sprite {character: NECKLACE_CHAR, color: &[Brown]}},
    AppearanceInfo {name: "crimson necklace",  sprite: Sprite {character: NECKLACE_CHAR, color: &[Maroon]}},
    AppearanceInfo {name: "glowing necklace",  sprite: Sprite {character: NECKLACE_CHAR, color: &[Yellow]}},
    AppearanceInfo {name: "unholy necklace",   sprite: Sprite {character: NECKLACE_CHAR, color: &[Dark]}},
    AppearanceInfo {name: "wand",              sprite: Sprite {character: DEVICE_CHAR,   color: &[Navy]}},
    AppearanceInfo {name: "manual",            sprite: Sprite {character: DEVICE_CHAR,   color: &[Teal]}},
    AppearanceInfo {name: "guidebook",         sprite: Sprite {character: DEVICE_CHAR,   color: &[Teal]}},
    AppearanceInfo {name: "red device",        sprite: Sprite {character: DEVICE_CHAR,   color: &[Red]}},
    AppearanceInfo {name: "yellow device",     sprite: Sprite {character: DEVICE_CHAR,   color: &[Yellow]}},
    AppearanceInfo {name: "blue device",       sprite: Sprite {character: DEVICE_CHAR,   color: &[Blue]}},
    AppearanceInfo {name: "palantir",          sprite: Sprite {character: ARTIFACT_CHAR, color: &[Blue, Navy, Aqua, Teal]}},
    AppearanceInfo {name: "golden pendant",    sprite: Sprite {character: ARTIFACT_CHAR, color: &[Red, Yellow, Brown, White]}},
];

impl Appearance {
    // Unidentified item names.
    pub fn name(self) -> &'static str {
        APPEARANCE_INFOS[self as usize].name
    }

    pub fn sprite(self) -> Sprite {
        APPEARANCE_INFOS[self as usize].sprite
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    Crowbar,
    VolcanicShard,
    Taser,
    JellyGun,
    FullHPFruit,
    FullTPFruit,
    CancellationFruit,
    ChargePill,
    XPUpPill,
    HastePill,
    IdentifyPill,
    XPDownPill,
    PoisonPill,
    ProtectPill,
    TormentPill,
    ThickSweater,
    BallisticVest,
    DragonScaleMail,
    TitaniumNecklace,
    RustyNecklace,
    CrimsonNecklace,
    GlowingNecklace,
    UnholyNecklace,
    WandOfDeath,
    Manual,
    Guidebook,
    Corruptor,
    Offsetter,
    Copier,
    Palantir,
    GoldenPendant,
}

// A map from Appearances (0x00 through 0x1f) to Kinds.
// NoItem is mapped to None; other items are mapped to Some<Kind>.
type AppearanceMap = [Option<Kind>; 0x20];

// Make a random appearance map by shuffling the unidentified fruits, pills, and devices.
pub fn random_appearance_map() -> AppearanceMap {
    use item::Kind::*;

    // Shuffle the kinds that will correspond to randomized item descriptions.
    let mut fruits: [Kind; 3] = [
        FullHPFruit, FullTPFruit, CancellationFruit
    ];
    rand::thread_rng().shuffle(&mut fruits);

    let mut pills: [Kind; 8] = [
        ChargePill, XPUpPill, HastePill, IdentifyPill,
        XPDownPill, PoisonPill, ProtectPill, TormentPill
    ];
    rand::thread_rng().shuffle(&mut pills);

    let mut devices: [Kind; 3] = [
        Corruptor, Offsetter, Copier
    ];
    rand::thread_rng().shuffle(&mut devices);

    // Build a map.
    [
        None,
        Some(Crowbar),
        Some(VolcanicShard),
        Some(Taser),
        Some(JellyGun),
        Some(fruits[0]),
        Some(fruits[1]),
        Some(fruits[2]),
        Some(pills[0]),
        Some(pills[1]),
        Some(pills[2]),
        Some(pills[3]),
        Some(pills[4]),
        Some(pills[5]),
        Some(pills[6]),
        Some(pills[7]),
        Some(ThickSweater),
        Some(BallisticVest),
        Some(DragonScaleMail),
        Some(TitaniumNecklace),
        Some(RustyNecklace),
        Some(CrimsonNecklace),
        Some(GlowingNecklace),
        Some(UnholyNecklace),
        Some(WandOfDeath),
        Some(devices[0]),
        Some(devices[1]),
        Some(devices[2]),
        Some(Offsetter),
        Some(Copier),
        Some(Palantir),
        Some(GoldenPendant),
    ]
}

pub struct Item {
    // 5 high bits
    kind: Kind,

    // 3 low bits
    enchanted: bool,
    equipped: bool,
    cursed: bool,
}

impl Item {
    // An inventory byte with its five high bits cleared represents an
    // empty inventory slot; `from_byte` returns None for such bytes.
    pub fn from_byte(byte: u8, appearance_map: &AppearanceMap) -> Option<Item> {
        appearance_map[byte as usize >> 3].map(|kind| Item {
            kind: kind,
            enchanted: byte & 0b100 > 0,
            equipped: byte & 0b010 > 0,
            cursed: byte & 0b001 > 0,
        })
    }

    pub fn to_byte(&self, appearance_map: &AppearanceMap) -> u8 {
        for n in 0..19 {
            if appearance_map[n] == Some(self.kind) {
                return (n << 3) as u8
                    + (if self.enchanted {0b100} else {0})
                    + (if self.equipped {0b010} else {0})
                    + (if self.cursed {0b001} else {0});
            }
        }
        panic!("Invalid AppearanceMap!");
    }
}
