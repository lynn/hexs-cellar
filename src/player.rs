use std::collections::HashSet;
use byte;
use byte::BitNumber;
use dungeon::{Dungeon, Level};
use geometry::Point;
use grid;
use tile::{Tile, Stairs};
use fov;
use log::Log;
use util::a_or_an;
use item::{Inventory, InventorySlot};
use speech;

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

    // Index with byte::BitNumber.
    pub inventory: Inventory,

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
    pub fn new(log: &mut Log, dungeon: &mut Dungeon) -> Player {
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
            inventory: Inventory::empty(),
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
        log.tell(speech::intro_line());
        player.enter_level(log, dungeon, 1, Stairs::Up);
        player
    }

    pub fn current_level<'a>(&self, dungeon: &'a Dungeon) -> &'a Level {
        &dungeon[self.depth as usize - 1]
    }

    pub fn current_level_mut<'a>(&self, dungeon: &'a mut Dungeon) -> &'a mut Level {
        &mut dungeon[self.depth as usize - 1]
    }

    // enter a level: put the player on the appropriate stairs.
    fn enter_level(&mut self, log: &mut Log, dungeon: &mut Dungeon, depth: u8, entry: Stairs) {
        if depth == 0 {
            // TODO: special case for level 0
        } else {
            self.depth = depth;

            let mut level = self.current_level_mut(dungeon);

            for tile_position in grid::RECTANGLE {
                if level.tiles[tile_position] == Tile::Stairs(entry) {
                    self.position = tile_position;
                    break
                }
            }

            self.update_visibility(&mut level);
            self.look_at_floor(log, level)
        }
    }

    pub fn try_stairs(&mut self, log: &mut Log, dungeon: &mut Dungeon, stairs: Stairs) {
        if self.current_level(dungeon).tiles[self.position] == Tile::Stairs(stairs) {
            match stairs {
                Stairs::Up   => log.tell("You go up the stairs."),
                Stairs::Down => log.tell("You go down the stairs.")
            };
            let destination = stairs.destination(self);
            self.enter_level(log, dungeon, destination, stairs.flip())
        }
    }

    // try to walk in given direction.
    // returns whether this consumes a turn
    pub fn step(&mut self, log: &mut Log, dungeon: &mut Dungeon, direction: Point) -> bool {
        let mut level = self.current_level_mut(dungeon);
        let new_position = self.position + direction;

        // don't let the player step out of bounds
        if !grid::RECTANGLE.contains(new_position) { return false }

        if let Some(monster) = level.monster_at_mut(new_position) {
            // attack
            log.tell(format!("You hit the {}.", monster.name()));
            monster.hp = 0;
            return true
        }

        match level.tiles[new_position] {
            Tile::Wall => false,
            Tile::Floor | Tile::Doorway | Tile::Stairs(_) => {
                self.position = new_position;
                self.update_visibility(&mut level);
                self.look_at_floor(log, level);
                true
            },
            Tile::Door => {
                level.tiles[new_position] = Tile::Doorway;
                log.tell("You open the door.");
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

    fn look_at_floor(&self, log: &mut Log, level: &Level) {
        if let Some(item) = level.items.get(&self.position) {
            log.tell(format!("You see here {}.", a_or_an(item.name())));
        }

        if let Tile::Stairs(stairs) = level.tiles[self.position] {
            let direction = match stairs {
                Stairs::Up   => "up",
                Stairs::Down => "down",
            };
            let destination = stairs.destination(self);

            if destination == 0 {
                log.tell(format!("There is a staircase leading {} out of the cellar here.", direction))
            } else {
                log.tell(format!("There is a staircase {} to level {} here.", direction, destination))
            }
        }
    }


    // try to pick up an item from the floor; returns if a turn was consumed
    pub fn pick_up_item(&mut self, log: &mut Log, dungeon: &mut Dungeon) -> bool {
        use std::collections::hash_map::Entry::*;

        let mut level = self.current_level_mut(dungeon);

        if let Occupied(floor_item) = level.items.entry(self.position) {
            let item = *floor_item.get();
            if self.inventory.insert(item) {
                floor_item.remove();
                log.tell(format!("You pick up the {}.", item.name()));
                true
            } else {
                log.tell("Your inventory is full!");
                false
            }
        } else {
            log.tell("There is no item here.");
            false
        }
    }

    // try to drop an item to the floor; returns if a turn was consumed
    pub fn drop_item(&mut self, log: &mut Log,
        dungeon: &mut Dungeon, index: BitNumber) -> bool
    {
        use rand::Rng;
        use rand::thread_rng;
        use std::collections::hash_map::Entry::*;

        // TODO: special handling for equipped items?
        let slot = self.inventory.slots[index as usize];
        let item = match slot.get_item() {
            Some(item) => {
                if slot.is_cursed() {
                    log.tell(format!("You can't drop the cursed {}!", item.name()));
                    return false
                } else {
                    item
                }
            },
            None => {
                log.tell("You don't have that item!");
                return false
            }
        };

        let mut level = self.current_level_mut(dungeon);

        // find a nearby floor tile to put the item on.
        // this allows for items to spill over a single tile for convenience,
        // but not travel long distances or leave the player's field of view.
        // we first prefer the player's current tile, then tiles one step away
        // in an orthogonal direction, then finally diagonal directions
        let mut directions = [
            Point(0, 0),
            Point(0, 1), Point(1, 0),  Point(0, -1), Point(-1, 0),
            Point(1, 1), Point(-1, 1), Point(1, -1), Point(-1, -1)
        ];
        thread_rng().shuffle(&mut directions[1..5]);
        thread_rng().shuffle(&mut directions[5..9]);

        for &direction in directions.iter() {
            let position = self.position + direction;

            let tile = level.tiles[position];

            if tile == Tile::Floor || tile == Tile::Doorway {
                if let Vacant(floor) = level.items.entry(position) {
                    self.inventory.slots[index as usize] = InventorySlot::empty();
                    log.tell(format!("You drop the {}.", item.name()));
                    floor.insert(item);
                    return true
                }
            }
        }

        // nowhere nearby to place item
        log.tell("No room on floor!");
        false
    }

    // try to use an item in inventory; returns if a turn was consumed
    pub fn use_item(&mut self, log: &mut Log,
        dungeon: &mut Dungeon, index: BitNumber) -> bool
    {
        let slot = self.inventory.slots[index as usize];
        let item = match slot.get_item() {
            Some(item) => item,
            None => {
                log.tell("You don't have that item!");
                return false
            }
        };

        if item.is_consumable() {
            log.tell("[Item consumed. Do something here.]");
            self.inventory.slots[index as usize] = InventorySlot::empty();
            return true
        }

        if let Some(body_part) = item.equipment_slot() {
            log.tell(format!("[Item is {:?} equipment. Try to equip/unequip.]", body_part));
            return false
        }

        log.tell("[Not a consumable or equipment. Do something here.]");
        false
    }
}
