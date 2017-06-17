use byte::BitNumber;
use sprite::*;

#[derive(Copy, Clone)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Doorway,
    StairsUp,
    StairsDown,
    Switch(BitNumber, bool),
}

impl Tile {
    pub fn sprite(self) -> Sprite {
        match self {
            Tile::Floor =>      Sprite {character: '.',      color: GRAY},
            Tile::Wall =>       Sprite {character: '#',      color: TEAL},
            Tile::Door =>       Sprite {character: '+',      color: BROWN},
            Tile::Doorway =>    Sprite {character: '\'',     color: BROWN},
            Tile::StairsUp =>   Sprite {character: '<',      color: WHITE},
            Tile::StairsDown => Sprite {character: '>',      color: WHITE},
            Tile::Switch(n, on) =>
                Sprite {character: n.char(), color: if on {YELLOW} else {NAVY}},
        }
    }
}
