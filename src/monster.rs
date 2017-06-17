use sprite::*;

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
    name: &'static str,
    sprite: Sprite,
    habitat: (u8, u8),
    max_hp: u8,
}

pub const INFOS: [Info; 16] = [
    Info {name: "kestrel",       sprite: Sprite {character: 'K', color: WHITE},  habitat: ( 1,   4), max_hp: 6},
    Info {name: "skeleton",      sprite: Sprite {character: 'Z', color: GRAY},   habitat: ( 1,   5), max_hp: 8},
    Info {name: "troll",         sprite: Sprite {character: 'T', color: BROWN},  habitat: ( 2,   6), max_hp: 20},
    Info {name: "android",       sprite: Sprite {character: 'A', color: TEAL},   habitat: ( 3,   7), max_hp: 15},
    Info {name: "jelly",         sprite: Sprite {character: 'J', color: LIME},   habitat: ( 4,   8), max_hp: 13},
    Info {name: "salamander",    sprite: Sprite {character: 'S', color: RED},    habitat: ( 5,   9), max_hp: 18},
    Info {name: "tiny UFO",      sprite: Sprite {character: 'U', color: AQUA},   habitat: ( 6,  10), max_hp: 16},
    Info {name: "minotaur",      sprite: Sprite {character: 'M', color: MAROON}, habitat: ( 8,  15), max_hp: 40},
    Info {name: "glitch",        sprite: Sprite {character: 'B', color: GLITCH}, habitat: ( 0,   0), max_hp: 15},
    Info {name: "witch",         sprite: Sprite {character: 'W', color: PURPLE}, habitat: (11,  16), max_hp: 24},
    Info {name: "ghost",         sprite: Sprite {character: 'G', color: DARK},   habitat: (13,  19), max_hp: 35},
    Info {name: "soldier",       sprite: Sprite {character: '@', color: BLUE},   habitat: (14,  20), max_hp: 45},
    Info {name: "attractor",     sprite: Sprite {character: '8', color: TEAL},   habitat: (15,  20), max_hp: 50},
    Info {name: "turret",        sprite: Sprite {character: '9', color: DARK},   habitat: (15,  20), max_hp: 60},
    Info {name: "elf",           sprite: Sprite {character: 'E', color: LIME},   habitat: ( 1,   0), max_hp: 40},
    Info {name: "golden dragon", sprite: Sprite {character: 'D', color: GOLD},   habitat: (20, 255), max_hp: 200},
];
