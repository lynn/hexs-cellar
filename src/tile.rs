use byte::BitNumber;

#[derive(Copy, Clone)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Doorway,
    StairsUp,
    StairsDown,
    Switch(BitNumber),
}
