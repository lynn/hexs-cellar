use byte::{self, BitNumber};
use sprite::*;
use world::World;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Doorway,
    StairsUp,
    StairsDown,
    Switch(BitNumber),
}

impl Tile {
    pub fn sprite(self, world: &World) -> Sprite {
        match self {
            Tile::Floor =>      Sprite {character: '.',  color: GRAY},
            Tile::Wall =>       Sprite::of_byte(world.wall_appearance_byte, false),
            Tile::Door =>       Sprite::of_byte(world.door_appearance_byte, false),
            Tile::Doorway =>    Sprite {character: '\'', color: BROWN},
            Tile::StairsUp =>   Sprite {character: '<',  color: WHITE},
            Tile::StairsDown => Sprite {character: '>',  color: WHITE},
            Tile::Switch(bn) => {
                let on = byte::get(world.player.selected, bn);
                Sprite {character: bn.char(), color: if on {YELLOW} else {NAVY}}
            },
        }
    }

    pub fn permits_sight(self) -> bool {
        match self {
            Tile::Floor | Tile::Doorway | Tile::StairsUp | Tile::StairsDown => true,
            Tile::Wall | Tile::Door | Tile::Switch(_) => false
        }
    }
}
