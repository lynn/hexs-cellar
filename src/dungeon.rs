use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::fmt;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::result::Result;
use std::convert::From;
use std::ascii::AsciiExt;
use std::iter::Iterator;
use std::iter::FromIterator;
use rand;
use rand::Rng;
use byte::BitNumber;
use geometry::Point;
use grid;
use grid::Grid;
use item::{Item};
use sprite::Sprite;
use tile::{Tile, Stairs};
use util::{coin_flip, random_range, random_range_two, sample};
use world::World;
use monster::Monster;


// A dungeon level.
pub struct Level {
    pub tiles: Grid<Tile>,
    pub items: HashMap<Point, Item>,
    pub known_tiles: HashSet<Point>,
    pub monsters: [Monster; 5]
}

impl Level {
    pub fn monster_at(&self, position: Point) -> Option<&Monster> {
        self.monsters.iter().filter(|m| m.position == position && m.alive()).nth(0)
    }

    pub fn monster_at_mut(&mut self, position: Point) -> Option<&mut Monster> {
        self.monsters.iter_mut().filter(|m| m.position == position && m.alive()).nth(0)
    }

    pub fn sprite_at(&self, position: Point, world: &World) -> Sprite {
        if position == world.player.position {
            Sprite::of_byte(world.player_appearance_byte, true)
        } else if let Some(monster) = self.monster_at(position) {
            monster.sprite()
        } else if let Some(item) = self.items.get(&position) {
            item.sprite()
        } else {
            self.tiles[position].sprite(world)
        }
    }
}

pub type Dungeon = Vec<Level>;


pub enum MapError {
    ShapeError(usize),    // funny map shape at given line
    TileError(usize, u8), // bad tile (u8) on given map (usize)
    StairError(usize),    // not enough stairs on given map
    SwitchError(usize),   // not enough switches on given map (need 2)
    IoError(io::Error)
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MapError::ShapeError(line) =>
                write!(f, "map error: misshapen map at line {}", line),
            MapError::TileError(which_map, tile) =>
                write!(f, "map error: unknown tile '{}' on map {}",
                    tile as char, which_map),
            MapError::StairError(which_map) =>
                write!(f, "map error: not enough stairs on map {}", which_map),
            MapError::SwitchError(which_map) =>
                write!(f, "map error: not enough switches on map {}", which_map),
            MapError::IoError(ref e) => e.fmt(f)
        }
    }
}

impl From<io::Error> for MapError {
    fn from(e: io::Error) -> Self {
        MapError::IoError(e)
    }
}


fn spawn_items(map: &Grid<Tile>) -> HashMap<Point, Item> {
    let floors = grid::RECTANGLE.into_iter().filter(|p| map[*p] == Tile::Floor);
    let locations = sample(floors, 5);
    HashMap::from_iter(locations.iter().map(|p| (*p, Item::spawn())))
}

fn spawn_monsters(depth: u8, map: &Grid<Tile>) -> [Monster; 5] {
    let mut monsters = [Monster::null(); 5];

    // give the player some breathing room when going downstairs the first time
    let upstairs = grid::RECTANGLE.into_iter().filter(|p|
        map[*p] == Tile::Stairs(Stairs::Up)).nth(0).unwrap();
    let floors = grid::RECTANGLE.into_iter().filter(|p|
        map[*p] == Tile::Floor && p.cheby_dist(upstairs) > 1);

    for (position, monster) in sample(floors, 5).into_iter().zip(&mut monsters) {
        *monster = Monster::generate(depth, position)
    }

    monsters
}


// TODO: make early levels easy, handle special case for level 255
pub fn build() -> Result<Dungeon, MapError> {
    let mut schemes = read_maps()?;

    // until we have 255 distinct levels, make do with duplicates
    let distinct_schemes = schemes.len();
    while schemes.len() < 255 {
        let which = random_range(0..distinct_schemes);
        let duplicate = schemes[which].clone();
        schemes.push(duplicate)
    }

    let mut maps: Vec<Grid<Tile>> = Vec::with_capacity(255);
    for (which_map, scheme) in (1..).zip(schemes.iter()) {
        let mut map = build_map(which_map, scheme)?;
        flip_randomly(&mut map);
        maps.push(map)
    }

    rand::thread_rng().shuffle(&mut maps[..]);

    let dungeon = (1..).zip(maps).map(|(depth, map)| {
        Level {
            items: spawn_items(&map),
            monsters: spawn_monsters(depth as u8, &map),
            tiles: map,
            known_tiles: HashSet::new()
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

fn build_map(which_map: usize, scheme: &Grid<u8>) -> Result<Grid<Tile>, MapError> {
    let mut map = Grid::empty();

    // preliminary pass: figure out where to place stairs
    let stair_count = scheme.grid.iter().filter(|&&t| t == b'<').count();
    if stair_count < 2 {
        return Err(MapError::StairError(which_map))
    }
    let (upstairs, downstairs) = random_range_two(0..stair_count);

    // same for switches
    let switch_count = scheme.grid.iter().filter(|&&t| t == b'1').count();
    if switch_count < 2 {
        return Err(MapError::SwitchError(which_map))
    }
    let (switch1, switch2) = random_range_two(0..switch_count);

    // a HashMap to keep track of which tile (floor or wall) to use for
    // each letter A-Z in the map scheme (and the inverse for a-z)
    let mut tile_choices: HashMap<u8, bool> = HashMap::new();

    let mut stair_count = 0;
    let mut switch_count = 0;

    for &byte in scheme.grid.iter() {
        map.grid.push(match byte {
            b'.' => Tile::Floor,
            b'#' => Tile::Wall,
            b'+' => Tile::Door,
            b'<' => {
                let tile = if stair_count == upstairs {
                    Tile::Stairs(Stairs::Up)
                } else if stair_count == downstairs {
                    Tile::Stairs(Stairs::Down)
                } else {
                    Tile::Floor
                };
                stair_count += 1;
                tile
            }
            b'1' => {
                let tile = if switch_count == switch1 || switch_count == switch2 {
                    Tile::Switch(BitNumber::from_number(random_range(0..6)))
                } else {
                    Tile::Wall
                };
                switch_count += 1;
                tile
            }
            b'A' ... b'Z' | b'a' ... b'z' => {
                let normalized = byte.to_ascii_uppercase();
                let selection = *tile_choices.entry(normalized).or_insert_with(coin_flip);
                // flip selection for lowercase letters
                // TODO: use byte.is_ascii_uppercase()
                // once ascii_ctype is stable
                if selection == (byte == normalized) {
                    Tile::Wall
                } else {
                    Tile::Floor
                }
            }
            _ => return Err(MapError::TileError(which_map, byte))
        })
    }

    Ok(map)
}

fn flip_randomly<T>(map: &mut Grid<T>) {
    if coin_flip() {
        // flip horizontally
        for row in map.grid.chunks_mut(grid::WIDTH) {
            row.reverse()
        }
    }
    if coin_flip() {
        // flip both horizontally and vertically
        map.grid.reverse()
    }
}
