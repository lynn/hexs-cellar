use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::fmt;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::result::Result;
use std::convert::From;
use std::ascii::AsciiExt;
use rand;
use rand::Rng;
use grid;
use grid::Grid;
use item::{Item};
use geometry::Point;
use tile::{Tile};


// A dungeon level.
pub struct Level {
    pub tiles: Grid<Tile>,
    pub items: HashMap<Point, Item>,
    pub visible: HashSet<Point>,
}

pub type Dungeon = Vec<Level>;


pub enum MapError {
    ShapeError(usize),    // funny map shape at given line
    TileError(usize, u8), // bad tile (u8) on given map (usize)
    StairError(usize),    // not enough stairs on given map
    IoError(io::Error)
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MapError::ShapeError(line) =>
                write!(f, "map error: misshapen map at line {}", line),
            MapError::TileError(whichmap, tile) =>
                write!(f, "map error: unknown tile '{}' on map {}",
                    tile as char, whichmap),
            MapError::StairError(whichmap) =>
                write!(f, "map error: not enough stairs on map {}", whichmap),
            MapError::IoError(ref e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for MapError {
    fn from(e: io::Error) -> Self {
        MapError::IoError(e)
    }
}


// TODO: make early levels easy, handle special case for level 255
pub fn build() -> Result<Dungeon, MapError> {
    let mut schemes = read_maps()?;

    // until we have 255 distinct levels, make do with duplicates
    let distinct_schemes = schemes.len();
    while schemes.len() < 255 {
        let which = rand::thread_rng().gen_range(0, distinct_schemes);
        let duplicate = schemes[which].clone();
        schemes.push(duplicate)
    }

    let mut maps: Vec<Grid<Tile>> = Vec::with_capacity(255);
    for (whichmap, scheme) in (1..).zip(schemes.iter()) {
        let mut map = build_map(whichmap, scheme)?;
        flip_randomly(&mut map);
        maps.push(map)
    }

    rand::thread_rng().shuffle(&mut maps[..]);

    let dungeon = maps.into_iter().map(|map| {
        Level {
            tiles: map,
            items: HashMap::new(),
            visible: HashSet::new()
        }
    }).collect();

    Ok(dungeon)
}

type Lines<'a> = &'a mut Iterator<Item=io::Result<String>>;

fn read_maps() -> Result<Vec<Grid<u8>>, MapError> {
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

fn read_map(linecount: &mut usize, lines: Lines) -> Result<Grid<u8>, MapError> {
    let mut grid = Grid::empty();
    for _ in 0..grid::HEIGHT {
        match lines.next() {
            Some(Ok(ref row)) if row.len() == grid::WIDTH =>
                grid.grid.extend(row.bytes()),
            Some(Err(e)) => return Err(MapError::IoError(e)),
            _ => return Err(MapError::ShapeError(*linecount))
        }
        *linecount = *linecount + 1
    }

    Ok(grid)
}

fn build_map(whichmap: usize, scheme: &Grid<u8>) -> Result<Grid<Tile>, MapError> {
    let mut map = Grid::empty();

    // preliminary pass: figure out where to place stairs
    let stairtotal = scheme.grid.iter().filter(|&&t| t == b'<').count();
    if stairtotal < 2 {
        return Err(MapError::StairError(whichmap))
    }
    let upstairs = rand::thread_rng().gen_range(0, stairtotal);
    let mut downstairs = upstairs;
    while downstairs == upstairs {
        downstairs = rand::thread_rng().gen_range(0, stairtotal)
    }

    // a HashMap to keep track of which tile (floor or wall) to use for
    // each letter A-Z in the map scheme (and the inverse for a-z)
    let mut tilechoices: HashMap<u8, bool> = HashMap::new();

    let mut staircount = 0;

    for &srctile in scheme.grid.iter() {
        map.grid.push(match srctile {
            b'.' => Tile::Floor,
            b'#' => Tile::Wall,
            b'+' => Tile::Door,
            b'<' => {
                let tile = if staircount == upstairs {
                    Tile::StairsUp
                } else if staircount == downstairs {
                    Tile::StairsDown
                } else {
                    Tile::Floor
                };
                staircount += 1;
                tile
            }
            b'A' ... b'Z' | b'a' ... b'z' => {
                let normalized = srctile.to_ascii_uppercase();
                let selection = *tilechoices.entry(normalized).or_insert_with(||
                    rand::thread_rng().gen() );
                // flip selection for lowercase letters
                // TODO: use srctile.is_ascii_uppercase()
                // once ascii_ctype is stable
                if selection == (srctile == normalized) {
                    Tile::Wall
                } else {
                    Tile::Floor
                }
            }
            _ => return Err(MapError::TileError(whichmap, srctile))
        })
    }

    Ok(map)
}

fn flip_randomly<T>(map: &mut Grid<T>) {
    if rand::thread_rng().gen() {
        // flip horizontally
        for row in map.grid.chunks_mut(grid::WIDTH) {
            row.reverse()
        }
    }
    if rand::thread_rng().gen() {
        // flip both horizontally and vertically
        map.grid.reverse()
    }
}
