use std::collections::{HashMap, HashSet};
use item::{Item};
use point::{Point};
use tile::{Tile};

pub const WIDTH: usize = 19;
pub const HEIGHT: usize = 13;

// A dungeon level.
pub struct Level {
    depth: u8,
    tiles: [Tile; WIDTH * HEIGHT], // In row order.
    items: HashMap<Point, Item>,
    visible: HashSet<Point>,
}

pub type Dungeon = [Level; 256];
