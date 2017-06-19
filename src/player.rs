use dungeon::Dungeon;
use geometry::Point;
use grid;
use tile::Tile;
use item;

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

    pub appearance_byte: u8,
    pub door_appearance_byte: u8,
    pub wall_appearance_byte: u8,

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

    pub item_appearance_map: item::AppearanceMap,
}

impl Player {
    // enters first level automatically
    pub fn new(dungeon: &Dungeon) -> Player {
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
            appearance_byte: 0b11111111, // white @
            door_appearance_byte: 0b11001010, // brown +
            wall_appearance_byte: 0b01100010, // teal #
            spell_memory: [false; 8],
            timer: [0; 4],
            selected: 0x00,
            stairs_delta: 1,
            timer_delta: 0xFF,
            damage_offset: 0,
            text_sync: 0,
            show_ram: false,
            item_appearance_map: item::random_appearance_map(),
        };
        player.enter_level(&dungeon, 1, Tile::StairsUp);
        player
    }

    // enter a level: put the player on the appropriate stairs.
    // TODO: special case for level 0
    fn enter_level(&mut self, dungeon: &Dungeon, depth: u8, entry: Tile) {
        self.depth = depth;

        for tile_position in grid::RECTANGLE {
            if dungeon[depth as usize - 1].tiles[tile_position] == entry {
                self.position = tile_position;
                break
            }
        }

    }
}
