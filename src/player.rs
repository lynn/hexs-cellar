use geometry::Point;

pub struct Player {
    position: Point,
    depth: u8,
    name: [u8; 15],
    hp: u8,
    tp: u8,
    xl: u8,
    def: i8,

    // Index with element::Element.
    aptitude: [i8; 4],

    // Must be represented as bytes!
    inventory: [u8; 8],

    appearance_byte: u8,

    // Index with byte::BitNumber.
    spell_memory: [bool; 8],

    // Index with timer::Timer.
    timer: [u8; 4],

    // The address the player's spells will act on.
    selected: u8,

    stairs_delta: u8,
    timer_delta: u8,
    damage_offset: i8,
    text_sync: u8,

    // Interface
    show_ram: bool,
}

impl Player {
    pub fn new(position: Point) -> Player {
        Player {
            position: position,
            depth: 1,
            // TODO naming the player
            name: [97, 98, 99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            hp: 10,
            tp: 3,
            xl: 1,
            def: 0,
            aptitude: [0, 0, 0, 0],
            inventory: [0, 0, 0, 0, 0, 0, 0, 0],
            appearance_byte: 0,
            spell_memory: [false; 8],
            timer: [0; 4],
            selected: 0x00,
            stairs_delta: 1,
            timer_delta: 0xFF,
            damage_offset: 0,
            text_sync: 0,
            show_ram: false,
        }
    }
}
