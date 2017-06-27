use byte::{self, BitNumber};
use sprite::*;
use world::World;
use player::Player;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Doorway,
    Stairs(Stairs),
    Switch(BitNumber),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Stairs {
    Up, Down
}

impl Tile {
    pub fn sprite(self, world: &World) -> Sprite {
        match self {
            Tile::Floor =>                Sprite {character: '.',  color: GRAY},
            Tile::Wall =>                 Sprite::of_byte(world.wall_appearance_byte, false),
            Tile::Door =>                 Sprite::of_byte(world.door_appearance_byte, false),
            Tile::Doorway =>              Sprite {character: '\'', color: BROWN},
            Tile::Stairs(Stairs::Up) =>   Sprite {character: '<',  color: WHITE},
            Tile::Stairs(Stairs::Down) => Sprite {character: '>',  color: WHITE},
            Tile::Switch(bn) => {
                let on = byte::get(world.player.selected, bn);
                Sprite {character: bn.char(), color: if on {YELLOW} else {NAVY}}
            },
        }
    }

    pub fn is_open(self) -> bool {
        match self {
            Tile::Floor | Tile::Doorway | Tile::Stairs(_) => true,
            Tile::Wall | Tile::Door | Tile::Switch(_) => false
        }
    }
}

impl Stairs {
    pub fn destination(self, player: &Player) -> u8 {
        match self {
            Stairs::Up   => player.depth.wrapping_sub(player.stairs_delta),
            Stairs::Down => player.depth.wrapping_add(player.stairs_delta)
        }
    }

    pub fn flip(self) -> Stairs {
        match self {
            Stairs::Up   => Stairs::Down,
            Stairs::Down => Stairs::Up
        }
    }
}
