use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::fmt;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::result::Result;
use std::convert::From;
use grid::Grid;
use item::{Item};
use point::{Point};
use tile::{Tile};


// A dungeon level.
pub struct Level {
    depth: u8,
    tiles: Grid<Tile>,
    items: HashMap<Point, Item>,
    visible: HashSet<Point>,
}

pub type Dungeon = [Level; 256];


pub enum MapError {
    ShapeError(usize),     // funny map shape at given line
    ParseError(usize, u8), // bad tile (u8) at line (usize)
    IoError(io::Error)
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MapError::ShapeError(line) =>
                write!(f, "map error: misshapen map at line {}", line),
            MapError::ParseError(line, tile) =>
                write!(f, "map error: unknown tile '{}' at line {}",
                    tile as char, line),
            MapError::IoError(ref e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for MapError {
    fn from(e: io::Error) -> Self {
        MapError::IoError(e)
    }
}

type Lines<'a> = &'a mut Iterator<Item=io::Result<String>>;

pub fn read_maps() -> Result<Vec<Grid<u8>>, MapError> {
    let file = File::open("data/maps.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut maps: Vec<Grid<u8>> = Vec::with_capacity(255);

    let mut linecount: usize = 1;
    loop {
        if lines.next().is_none() {
            return Ok(maps)
        }
        linecount += 1;
        maps.push(read_map(&mut linecount, &mut lines)?);
    }
}

fn read_map(pos: &mut usize, lines: Lines) -> Result<Grid<u8>, MapError> {
    let mut grid = Grid::filled(0);
    for row in 0..13 {
        match lines.next() {
            Some(Ok(ref line)) if line.len() == 19 =>
                for (col, tile) in line.bytes().enumerate() {
                    grid[(col, row)] = tile
                },
            Some(Err(e)) => return Err(MapError::IoError(e)),
            _ => return Err(MapError::ShapeError(*pos))
        }
        *pos = *pos + 1
    }
    Ok(grid)
}
