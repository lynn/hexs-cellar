use std::collections::HashSet;
use byte;
use dungeon::{Dungeon, Level};
use geometry::Point;
use grid;
use tile::Tile;
use fov;
use log::Log;

pub struct Player {
    pub position: Point,
    pub depth: u8,
    pub name: [u8; 15],
    pub hp: u8,
    pub tp: u8,
    pub xl: u8,
    pub def: i8,

    // Index with element::Element.
    pub aptitude: [i8; 4],

    // Must be represented as bytes!
    pub inventory: [u8; 8],

    // Index with byte::BitNumber.
    pub spell_memory: [bool; 8],

    // Index with timer::Timer.
    pub timer: [u8; 4],

    // The address the player's spells will act on.
    pub selected: u8,

    pub stairs_delta: u8,
    pub timer_delta: u8,
    pub damage_offset: i8,
    pub text_sync: u8,

    // Interface
    pub show_ram: bool,

    pub visible: HashSet<Point>
}

impl Player {
    // enters first level automatically
    pub fn new(dungeon: &mut Dungeon) -> Player {
        let mut player = Player {
            position: Point(-1, -1),
            depth: 1,
            // TODO naming the player
            name: [97, 98, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            hp: 10,
            tp: 3,
            xl: 1,
            def: 0,
            aptitude: [0, 0, 0, 0],
            inventory: [0, 0, 0, 0, 0, 0, 0, 0],
            spell_memory: [false; 8],
            timer: [0; 4],
            selected: 0x00,
            stairs_delta: 1,
            timer_delta: 0xFF,
            damage_offset: 0,
            text_sync: 0,
            show_ram: false,
            visible: HashSet::new()
        };
        player.enter_level(dungeon, 1, Tile::StairsUp);
        player
    }

    pub fn current_level<'a>(&self, dungeon: &'a Dungeon) -> &'a Level {
        &dungeon[self.depth as usize - 1]
    }

    pub fn current_level_mut<'a>(&self, dungeon: &'a mut Dungeon) -> &'a mut Level {
        &mut dungeon[self.depth as usize - 1]
    }

    // enter a level: put the player on the appropriate stairs.
    fn enter_level(&mut self, dungeon: &mut Dungeon, depth: u8, entry: Tile) {
        if depth == 0 {
            // TODO: special case for level 0
        } else {
            self.depth = depth;

            let mut level = self.current_level_mut(dungeon);

            for tile_position in grid::RECTANGLE {
                if level.tiles[tile_position] == entry {
                    self.position = tile_position;
                    break
                }
            }

            self.update_visibility(&mut level)
        }

    }

    // TODO: factor these better if they end up growing
    pub fn try_stairs_up(&mut self, dungeon: &mut Dungeon) {
        if self.current_level(dungeon).tiles[self.position] == Tile::StairsUp {
            let new_depth = self.depth - 1;
            self.enter_level(dungeon, new_depth, Tile::StairsDown)
        }
    }
    pub fn try_stairs_down(&mut self, dungeon: &mut Dungeon) {
        if self.current_level(dungeon).tiles[self.position] == Tile::StairsDown {
            let new_depth = self.depth + 1;
            self.enter_level(dungeon, new_depth, Tile::StairsUp)
        }
    }

    // try to walk in given direction.
    // returns whether this consumes a turn
    pub fn step(&mut self, log: &mut Log, dungeon: &mut Dungeon, direction: Point) -> bool {
        let mut level = self.current_level_mut(dungeon);
        let new_position = self.position + direction;

        // don't let the player step out of bounds
        if !grid::RECTANGLE.contains(new_position) { return false }

        // TODO: check for monsters
        match level.tiles[new_position] {
            Tile::Wall => false,
            Tile::Floor | Tile::Doorway | Tile::StairsUp | Tile::StairsDown => {
                self.position = new_position;
                self.update_visibility(&mut level);
                // TODO: look at floor
                true
            },
            Tile::Door => {
                level.tiles[new_position] = Tile::Doorway;
                log.tell(String::from("You open the door."));
                self.update_visibility(&mut level);
                true
            },
            Tile::Switch(bn) => {
                self.selected = byte::flip(self.selected, bn);
                // TODO: log message for switch flip
                true
            }
        }
    }

    fn update_visibility(&mut self, level: &mut Level) {
        self.visible = fov::calculate(level, self.position);
        level.known_tiles.extend(&self.visible)
    }
}
