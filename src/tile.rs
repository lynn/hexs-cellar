use byte::{self, BitNumber};
use sprite::*;
use player::Player;

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
    pub fn sprite(self, player: &Player) -> Sprite {
        match self {
            Tile::Floor =>      Sprite {character: '.',  color: GRAY},
            Tile::Wall =>       Sprite {character: '#',  color: TEAL},
            Tile::Door =>       Sprite {character: '+',  color: BROWN},
            Tile::Doorway =>    Sprite {character: '\'', color: BROWN},
            Tile::StairsUp =>   Sprite {character: '<',  color: WHITE},
            Tile::StairsDown => Sprite {character: '>',  color: WHITE},
            Tile::Switch(bn) => {
                let on = byte::get(player.selected, bn);
                Sprite {character: bn.char(), color: if on {YELLOW} else {NAVY}}
            },
        }
    }
}
