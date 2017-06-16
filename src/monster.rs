use sprite::{Sprite};

pub enum Kind {
    Kestrel = 0x0,
    Skeleton = 0x1,
    Troll = 0x2,
    Android = 0x3,
    Jelly = 0x4,
    Salamander = 0x5,
    TinyUFO = 0x6,
    Minotaur = 0x7,
    Glitch = 0x8,
    Witch = 0x9,
    Ghost = 0xa,
    Soldier = 0xb,
    Attractor = 0xc,
    Turret = 0xd,
    Elf = 0xe,
    GoldenDragon = 0xf,
}

pub struct Info {
    appearance: Sprite,
    name: &'static str,
    habitat: (u8, u8),
    max_hp: u8,
}

pub const INFOS: [Info; 16] = [
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'K', color: 0u8},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },

];
