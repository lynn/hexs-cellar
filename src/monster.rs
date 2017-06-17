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
    name: &'static str,
    sprite: Sprite,
    habitat: (u8, u8),
    max_hp: u8,
}

pub const INFOS: [Info; 16] = [
    Info {name: "kestrel",       sprite: Sprite {character: 'K', color: &[White]},  habitat: ( 1,   4), max_hp: 6},
    Info {name: "skeleton",      sprite: Sprite {character: 'Z', color: &[Gray]},   habitat: ( 1,   5), max_hp: 8},
    Info {name: "troll",         sprite: Sprite {character: 'T', color: &[Brown]},  habitat: ( 2,   6), max_hp: 20},
    Info {name: "android",       sprite: Sprite {character: 'A', color: &[Teal]},   habitat: ( 3,   7), max_hp: 15},
    Info {name: "jelly",         sprite: Sprite {character: 'J', color: &[Lime]},   habitat: ( 4,   8), max_hp: 13},
    Info {name: "salamander",    sprite: Sprite {character: 'S', color: &[Red]},    habitat: ( 5,   9), max_hp: 18},
    Info {name: "tiny UFO",      sprite: Sprite {character: 'U', color: &[Aqua]},   habitat: ( 6,  10), max_hp: 16},
    Info {name: "minotaur",      sprite: Sprite {character: 'M', color: &[Maroon]}, habitat: ( 8,  15), max_hp: 40},
    Info {name: "glitch",        sprite: Sprite {character: 'B', color: &[Pink]},   habitat: ( 0,   0), max_hp: 15},
    Info {name: "witch",         sprite: Sprite {character: 'W', color: &[Purple]}, habitat: (11,  16), max_hp: 24},
    Info {name: "ghost",         sprite: Sprite {character: 'G', color: &[Dark]},   habitat: (13,  19), max_hp: 35},
    Info {name: "soldier",       sprite: Sprite {character: '@', color: &[Blue]},   habitat: (14,  20), max_hp: 45},
    Info {name: "attractor",     sprite: Sprite {character: '8', color: &[Teal]},   habitat: (15,  20), max_hp: 50},
    Info {name: "turret",        sprite: Sprite {character: '9', color: &[Dark]},   habitat: (15,  20), max_hp: 60},
    Info {name: "elf",           sprite: Sprite {character: 'E', color: &[Lime]},   habitat: ( 1,   0), max_hp: 40},
    Info {name: "golden dragon", sprite: Sprite {character: 'D', color: &[Yellow]}, habitat: (20, 255), max_hp: 200},
];
