use std::collections::{HashMap, HashSet};
use item::{Item};

pub const WIDTH: usize = 19;
pub const HEIGHT: usize = 13;

#[derive(Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub enum Tile {
    Floor,
    Wall,
    Door,
    Doorway,
    StairsUp,
    StairsDown,
    Switch(u8),
}

// A dungeon level. The terminology is from ZZT...
pub struct Board {
    depth: u8,
    tiles: [Tile; WIDTH * HEIGHT], // In row order.
    items: HashMap<Point, Item>,
    visible: HashSet<Point>,
}
