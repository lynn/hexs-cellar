#[derive(Copy, Clone)]
pub enum Appearance {
    NoItem = 0x00,
    Crowbar = 0x01,
    VolcanicShard = 0x02,
    Taser = 0x03,
    JellyGun = 0x04,
    Lumimelon = 0x05,
    Glowfruit = 0x06,
    Shineapple = 0x07,
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
    WandOfDeath = 0x18,
    Manual = 0x19,
    Guidebook = 0x1a,
    Corruptor = 0x1b,
    Offsetter = 0x1c,
    Copier = 0x1d,
    Palantir = 0x1e,
    GoldenPendant = 0x1f,
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

// NoItem is mapped to None; other items are mapped to Some<Kind>.
type AppearanceMap = [Option<Kind>; 0x20];

pub struct Item {
    // 5 high bits
    kind: Kind,

    // 3 low bits
    enchanted: bool,
    equipped: bool,
    cursed: bool,
}

impl Item {
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
