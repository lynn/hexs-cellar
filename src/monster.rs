use sprite::{Sprite};
use sprite::Color::*;

#[derive(Copy, Clone)]
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
        appearance: Sprite {character: 'K', color: &[White]},
        name: "kestrel",
        habitat: (1, 4),
        max_hp: 6,
    },
    Info {
        appearance: Sprite {character: 'Z', color: &[Gray]},
        name: "skeleton",
        habitat: (1, 5),
        max_hp: 8,
    },
    Info {
        appearance: Sprite {character: 'T', color: &[Brown]},
        name: "troll",
        habitat: (2, 6),
        max_hp: 20,
    },
    Info {
        appearance: Sprite {character: 'A', color: &[Teal]},
        name: "android",
        habitat: (3, 7),
        max_hp: 15,
    },
    Info {
        appearance: Sprite {character: 'J', color: &[Lime]},
        name: "jelly",
        habitat: (4, 8),
        max_hp: 13,
    },
    Info {
        appearance: Sprite {character: 'S', color: &[Red]},
        name: "salamander",
        habitat: (5, 9),
        max_hp: 18,
    },
    Info {
        appearance: Sprite {character: 'U', color: &[Aqua]},
        name: "tiny UFO",
        habitat: (6, 10),
        max_hp: 16,
    },
    Info {
        appearance: Sprite {character: 'M', color: &[Maroon]},
        name: "minotaur",
        habitat: (8, 15),
        max_hp: 40,
    },
    Info {
        appearance: Sprite {character: 'B', color: &[Pink]},
        name: "glitch",
        habitat: (0, 0),
        max_hp: 15,
    },
    Info {
        appearance: Sprite {character: 'W', color: &[Purple]},
        name: "witch",
        habitat: (11, 16),
        max_hp: 24,
    },
    Info {
        appearance: Sprite {character: 'G', color: &[Dark]},
        name: "ghost",
        habitat: (13, 19),
        max_hp: 35,
    },
    Info {
        appearance: Sprite {character: '@', color: &[Blue]},
        name: "soldier",
        habitat: (14, 20),
        max_hp: 45,
    },
    Info {
        appearance: Sprite {character: '8', color: &[Teal]},
        name: "attractor",
        habitat: (15, 20),
        max_hp: 50,
    },
    Info {
        appearance: Sprite {character: '9', color: &[Dark]},
        name: "turret",
        habitat: (15, 20),
        max_hp: 60,
    },
    Info {
        appearance: Sprite {character: 'E', color: &[Lime]},
        name: "elf",
        habitat: (1, 0), // don't generate
        max_hp: 40,
    },
    Info {
        appearance: Sprite {character: 'D', color: &[Yellow]},
        name: "golden dragon",
        habitat: (20, 255),
        max_hp: 200,
    },

];
