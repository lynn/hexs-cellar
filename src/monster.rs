use sprite::*;
use std::mem;
use util;
use geometry::*;
use rand::{Rng, thread_rng};
use grid;
use dungeon::Level;
use world::World;
use speech;

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

#[derive(Copy, Clone)]
pub struct Monster {
    pub kind: Kind,
    pub charged:    bool, // dam*2
    pub vulnerable: bool, // def=0
    pub venomous:   bool, // poisons
    pub corrupted:  bool, // dam*2, flips bits
    pub position: Point,
    pub hp: u8,

    alert: bool
}

impl Monster {
    pub fn null() -> Monster {
        Monster {
            kind: Kind::Kestrel, // 0
            charged:    false,
            vulnerable: false,
            venomous:   false,
            corrupted:  false,
            position: Point::of_byte(0),
            hp: 0,

            alert: false
        }
    }

    pub fn generate(depth: u8, position: Point) -> Monster {
        let infos = &INFOS;
        let (kind, info) = util::pick((0..16u8).zip(infos)
            .filter(|&(_, ref info)| habitable(info, depth)));

        Monster {
            kind: unsafe { mem::transmute(kind) },
            charged:    false,
            vulnerable: false,
            venomous:   false,
            corrupted:  false,
            position: position,
            hp: info.max_hp,

            alert: false
        }
    }

    pub fn sprite(&self) -> Sprite {
        INFOS[self.kind as usize].sprite
    }

    pub fn name(&self) -> &'static str {
        INFOS[self.kind as usize].name
    }

    pub fn alive(&self) -> bool {
        self.hp > 0
    }
}

fn habitable(info: &Info, depth: u8) -> bool {
    let (low, high) = info.habitat;
    low <= depth && depth <= high
}


pub fn take_turns(world: &mut World) {
    let player = &world.player;
    let mut level = player.current_level_mut(&mut world.dungeon);

    // filter to get only living monsters;
    // score monsters by distance to player so that further monsters won't get
    // stuck behind closer monsters that haven't moved.
    let mut turn_order: Vec<(usize, (i32, i32))> =
        level.monsters.iter().enumerate().flat_map(|(i, m)|
            if m.alive() {
                Some( (i, (m.position.cheby_dist(player.position),
                           m.position.taxi_dist(player.position))) )
            } else {
                None
            }).collect();
    turn_order.sort_by_key(|&(_, score)| score);

    for (monster_index, _) in turn_order {
        if level.monsters[monster_index].alert {

            if level.monsters[monster_index].position.cheby_dist(player.position) == 1 {
                // TODO: attack
            } else if player.visible.contains(&level.monsters[monster_index].position) {
                approach(level, monster_index, player.position);
            }

        } else {

            let mut monster = &mut level.monsters[monster_index];

            if player.visible.contains(&monster.position) {
                // TODO: alert stuff nearby? Maybe some monsters are loud, and some aren't -- the shout_lines should make it obvious.
                world.log.tell(speech::shout_line(monster.kind));
                monster.alert = true;
            }

        }
    }
}


fn approach(level: &mut Level, monster_index: usize, target: Point) -> bool {
    let current = level.monsters[monster_index].position;
    let current_cheby_dist = current.cheby_dist(target);
    let current_taxi_dist  = current.taxi_dist(target);

    // scoring function for points we could step to -- lower is better
    let score = |p: Point|
        (p.cheby_dist(target) - current_cheby_dist,
         p.taxi_dist(target)  - current_taxi_dist);

    // filter to get only the points that move us closer in some way
    let mut choices: Vec<((i32, i32), Point)> =
        Rectangle::point(current).grow(1).into_iter().flat_map(|p| {
            let p_score = score(p);
            if p_score < (0, 0) {Some((p_score, p))} else {None}
        }).collect();
    // we use a stable sort, so shuffling will randomize the order of points
    // with the same score.
    thread_rng().shuffle(&mut choices[..]);
    // try points with better scores first
    choices.sort_by_key(|&(score, _)| score);

    for (_, point) in choices {
        if grid::RECTANGLE.contains(point)
            && level.tiles[point].is_open()
            && level.monster_at(point).is_none()
        {
            level.monsters[monster_index].position = point;
            return true
        }
    }

    false
}
