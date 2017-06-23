use rand;
use rand::Rng;
use sprite::*;
use util::{coin_flip, random_range};

#[derive(Copy, Clone)]
pub enum Appearance {
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

impl Appearance {
    pub fn from_byte(byte: u8) -> Appearance {
        use self::Appearance::*;
        match byte {
            0x01 => Crowbar,
            0x02 => VolcanicShard,
            0x03 => Taser,
            0x04 => JellyGun,
            0x05 => Lumimelon,
            0x06 => Glowfruit,
            0x07 => Shineapple,
            0x08 => RoundPill,
            0x09 => TinyPill,
            0x0a => DiamondPill,
            0x0b => OblongPill,
            0x0c => SoftPill,
            0x0d => HexagonalPill,
            0x0e => WidePill,
            0x0f => TranslucentPill,
            0x10 => ThickSweater,
            0x11 => BallisticVest,
            0x12 => DragonScaleMail,
            0x13 => TitaniumNecklace,
            0x14 => RustyNecklace,
            0x15 => CrimsonNecklace,
            0x16 => GlowingNecklace,
            0x17 => UnholyNecklace,
            0x18 => Wand,
            0x19 => Manual,
            0x1a => Guidebook,
            0x1b => RedDevice,
            0x1c => YellowDevice,
            0x1d => BlueDevice,
            0x1e => Palantir,
            0x1f => GoldenPendant,
            _    => panic!("Item::Appearance::from_byte - invalid argument")
        }
    }
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

const APPEARANCE_INFOS: [AppearanceInfo; 31] = [
    AppearanceInfo {name: "crowbar",           sprite: Sprite {character: WEAPON_CHAR,   color: TEAL}},
    AppearanceInfo {name: "volcanic shard",    sprite: Sprite {character: WEAPON_CHAR,   color: MAROON}},
    AppearanceInfo {name: "taser",             sprite: Sprite {character: WEAPON_CHAR,   color: AQUA}},
    AppearanceInfo {name: "jelly gun",         sprite: Sprite {character: WEAPON_CHAR,   color: LIME}},
    AppearanceInfo {name: "lumimelon",         sprite: Sprite {character: FOOD_CHAR,     color: LIME}},
    AppearanceInfo {name: "glowfruit",         sprite: Sprite {character: FOOD_CHAR,     color: PINK}},
    AppearanceInfo {name: "shineapple",        sprite: Sprite {character: FOOD_CHAR,     color: YELLOW}},
    AppearanceInfo {name: "round pill",        sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "tiny pill",         sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "diamond pill",      sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "oblong pill",       sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "soft pill",         sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "hexagonal pill",    sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "wide pill",         sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "translucent pill",  sprite: Sprite {character: PILL_CHAR,     color: WHITE}},
    AppearanceInfo {name: "thick sweater",     sprite: Sprite {character: ARMOR_CHAR,    color: PURPLE}},
    AppearanceInfo {name: "ballistic vest",    sprite: Sprite {character: ARMOR_CHAR,    color: TEAL}},
    AppearanceInfo {name: "dragon scale mail", sprite: Sprite {character: ARMOR_CHAR,    color: GREEN}},
    AppearanceInfo {name: "titanium necklace", sprite: Sprite {character: NECKLACE_CHAR, color: GRAY}},
    AppearanceInfo {name: "rusty necklace",    sprite: Sprite {character: NECKLACE_CHAR, color: BROWN}},
    AppearanceInfo {name: "crimson necklace",  sprite: Sprite {character: NECKLACE_CHAR, color: MAROON}},
    AppearanceInfo {name: "glowing necklace",  sprite: Sprite {character: NECKLACE_CHAR, color: YELLOW}},
    AppearanceInfo {name: "unholy necklace",   sprite: Sprite {character: NECKLACE_CHAR, color: DARK}},
    AppearanceInfo {name: "wand",              sprite: Sprite {character: DEVICE_CHAR,   color: NAVY}},
    AppearanceInfo {name: "manual",            sprite: Sprite {character: DEVICE_CHAR,   color: TEAL}},
    AppearanceInfo {name: "guidebook",         sprite: Sprite {character: DEVICE_CHAR,   color: TEAL}},
    AppearanceInfo {name: "red device",        sprite: Sprite {character: DEVICE_CHAR,   color: RED}},
    AppearanceInfo {name: "yellow device",     sprite: Sprite {character: DEVICE_CHAR,   color: YELLOW}},
    AppearanceInfo {name: "blue device",       sprite: Sprite {character: DEVICE_CHAR,   color: BLUE}},
    AppearanceInfo {name: "palantir",          sprite: Sprite {character: ARTIFACT_CHAR, color: SAPPHIRE}},
    AppearanceInfo {name: "golden pendant",    sprite: Sprite {character: ARTIFACT_CHAR, color: GOLD}},
];

impl Appearance {
    // Unidentified item names.
    pub fn name(self) -> &'static str {
        APPEARANCE_INFOS[self as usize - 1].name
    }

    pub fn sprite(self) -> Sprite {
        APPEARANCE_INFOS[self as usize - 1].sprite
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
// 0x00 is mapped to None; items are mapped to Some<Kind>.
pub type AppearanceMap = [Option<Kind>; 0x20];

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

#[derive(Clone, Copy)]
pub struct Item {
    // 5 high bits
    appearance: Appearance,

    // 3 low bits
    enchanted: bool,
    // equipped: bool, -- property of InventorySlot
    cursed: bool,
}

impl Item {
    pub fn spawn() -> Item {
        Item {
            appearance: Appearance::from_byte(random_range(0x01..0x20)),
            enchanted: coin_flip(),
            cursed: coin_flip(),
        }
    }

    pub fn sprite(&self) -> Sprite {
        self.appearance.sprite()
    }

    pub fn name(&self) -> &'static str {
        self.appearance.name()
    }
}

#[derive(Copy, Clone)]
pub struct InventorySlot {
    pub byte: u8
}

const APPEARANCE_MASK: u8 = 0b11111000;
const ENCHANT_MASK: u8    = 0b00000100;
const EQUIP_MASK: u8      = 0b00000010;
const CURSE_MASK: u8      = 0b00000001;

impl InventorySlot {
    pub fn empty() -> InventorySlot {
        InventorySlot { byte: 0 }
    }

    pub fn filled(item: Item) -> InventorySlot {
        InventorySlot {
            byte: ((item.appearance as u8) << 3)
                | (if item.enchanted {ENCHANT_MASK} else {0})
                | (if item.cursed    {CURSE_MASK}   else {0})
        }
    }

    pub fn is_empty(self) -> bool {
        self.byte & APPEARANCE_MASK == 0
    }

    pub fn get_item(self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            Some(Item {
                appearance: Appearance::from_byte((self.byte & APPEARANCE_MASK) >> 3),
                enchanted:  self.byte & ENCHANT_MASK != 0,
                cursed:     self.byte & CURSE_MASK != 0
            })
        }
    }

    pub fn is_cursed(self) -> bool {
        self.byte & CURSE_MASK != 0
    }

    pub fn is_enchanted(self) -> bool {
        self.byte & ENCHANT_MASK != 0
    }

    pub fn is_equipped(self) -> bool {
        self.byte & EQUIP_MASK != 0
    }

    pub fn equip(self) -> InventorySlot {
        InventorySlot { byte: self.byte | EQUIP_MASK }
    }

    pub fn unequip(self) -> InventorySlot {
        InventorySlot { byte: self.byte & !EQUIP_MASK }
    }
}


pub struct Inventory {
    pub slots: [InventorySlot; 8]
}

impl Inventory {
    pub fn empty() -> Inventory {
        Inventory { slots: [InventorySlot::empty(); 8] }
    }

    // try to insert an item into the inventory using the first available slot;
    // fails if there are no uncursed empty slots
    pub fn insert(&mut self, item: Item) -> bool {
        for slot in self.slots.iter_mut() {
            if slot.is_empty() && !slot.is_cursed() {
                // note: intentionally clears `equipped` bit
                *slot = InventorySlot::filled(item);
                return true
            }
        }
        false
    }
}
