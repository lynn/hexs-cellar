use std::mem::transmute;
use world::World;
use geometry::Point;
use sprite::Sprite;
use util;

// In Hex's Cellar, the player's spells manipulate an u8[40] of bytes that
// affect the world around her. This file gives a "RAM map" for that array.

// Color (3 bits = 8 bright colors) and character (5 bits, offset added to '!')
pub const PLAYER_APPEARANCE: u8 = 0x00;

// Begins a char[15].
pub const PLAYER_NAME: u8 = 0x01;

// Monster data is a (struct {u8, u8, u8})[5].
// So add (n * 3) to this, where 0 <= n <= 4, to get the address for the n-th monster,
// then add the offset of which byte you want.
pub const MONSTERS: u8 = 0x10;
pub const MONSTER_FLAGS: u8 = 0;
pub const MONSTER_POSITION: u8 = 1;
pub const MONSTER_HP: u8 = 2;

// An 8-bit bitmask (there are eight spells).
pub const SPELL_MEMORY: u8 = 0x1f;

// A 32-bit bitmask (there are thirty-two items).
pub const IDENTIFICATION: u8 = 0x20;

// Begins a u8[4]: one byte for each timer (poison, haste, charge, protect).
pub const TIMERS: u8 = 0x24;

// Begins a u8[8].
pub const INVENTORY: u8 = 0x28;

// Same as player appearance.
pub const DOOR_APPEARANCE: u8 = 0x30;
pub const WALL_APPEARANCE: u8 = 0x31;
pub const FLOOR_COLOR: u8 = 0x32;

// u8 to add/subtract from depth when stairs are used; normally 1.
pub const STAIRS_DELTA: u8 = 0x33;

// u8 to add to timers each turn if non-zero; normally 0xff.
pub const TIMER_DELTA: u8 = 0x34;

// s8 to add to each damage roll; normally 0x00.
pub const DAMAGE_OFFSET: u8 = 0x35;

// ??? = 0x36;
// ??? = 0x37;

// The higher this is, the more text gets screwed up.
pub const TEXT_SYNC: u8 = 0x38;

// Player stats.
pub const PLAYER_HP: u8 = 0x39;
pub const PLAYER_TP: u8 = 0x3a;
pub const PLAYER_XLDEF: u8 = 0x3b; // hi-bits XL, lo-bits Def
pub const PLAYER_POSITION: u8 = 0x3c;
pub const PLAYER_DEPTH: u8 = 0x3d;
pub const METAL_ACID_RESISTANCE: u8 = 0x3e; // hi-bits Metal, lo-bits Acid
pub const FIRE_ELEC_RESISTANCE: u8 = 0x3f; // hi-bits Fire, lo-bits Elect


pub fn peek(world: &World, address: u8) -> u8 {
    match address {
        PLAYER_APPEARANCE =>
            world.player_appearance_byte,

        _ if address >= PLAYER_NAME && address < MONSTERS =>
            world.player.name[(address - PLAYER_NAME) as usize],

        _ if address >= MONSTERS && address < SPELL_MEMORY => {
            let monster = &world.current_level()
                .monsters[(address - MONSTERS) as usize / 3];
            match (address - MONSTERS) % 3 {
                MONSTER_FLAGS => ((monster.kind as u8) << 4)
                    | ((monster.charged as u8) << 3)
                    | ((monster.vulnerable as u8) << 2)
                    | ((monster.venomous as u8) << 1)
                    | monster.corrupted as u8,
                MONSTER_POSITION => monster.position.as_byte(),
                MONSTER_HP => monster.hp,
                _ => unreachable!()
            }
        },

        _ if address >= SPELL_MEMORY && address < IDENTIFICATION =>
            world.player.spell_memory.iter().enumerate()
                .map(|(index, &known)| (known as u8) << index).sum(),

        _ if address >= IDENTIFICATION && address < TIMERS =>
            // TODO: implement identification
            0,

        _ if address >= TIMERS && address < INVENTORY =>
            world.player.timer[(address - TIMERS) as usize],

        _ if address >= INVENTORY && address < DOOR_APPEARANCE =>
            world.player.inventory.slots[(address - INVENTORY) as usize].byte,

        DOOR_APPEARANCE =>
            world.door_appearance_byte,

        WALL_APPEARANCE =>
            world.wall_appearance_byte,

        FLOOR_COLOR =>
            // TODO: remove floor color
            0,

        STAIRS_DELTA  =>
            world.player.stairs_delta,

        TIMER_DELTA =>
            world.player.timer_delta,

        DAMAGE_OFFSET =>
            unsafe { transmute(world.player.damage_offset) },

        0x36 =>
            // TODO: ???
            0,

        0x37 =>
            // TODO: ???
            0,

        TEXT_SYNC =>
            world.player.text_sync,

        PLAYER_HP =>
            world.player.hp,

        PLAYER_TP =>
            world.player.tp,

        PLAYER_XLDEF =>
            (world.player.xl << 4) | unsafe { transmute::<i8,u8>(world.player.def) },

        PLAYER_POSITION =>
            world.player.position.as_byte(),

        PLAYER_DEPTH =>
            world.player.depth,

        METAL_ACID_RESISTANCE =>
            // TODO: use element names
            unsafe {
                transmute((world.player.aptitude[0] << 4)
                    | (world.player.aptitude[1] & 0x0f))
            },

        FIRE_ELEC_RESISTANCE =>
            // TODO: use element names
            unsafe {
                transmute((world.player.aptitude[2] << 4)
                    | (world.player.aptitude[3] & 0x0f))
            },

        _ => panic!("memory::peek - invalid address {}", address)
    }
}


pub fn poke(world: &mut World, address: u8, value: u8) {
    match address {
        PLAYER_APPEARANCE => {
            let old_sprite = Sprite::of_byte(world.player_appearance_byte, true);
            let new_sprite = Sprite::of_byte(value, true);
            report_player_appearance_change(world, old_sprite, new_sprite);
            world.player_appearance_byte = value;
        },

        _ if address >= PLAYER_NAME && address < MONSTERS =>
            world.player.name[(address - PLAYER_NAME) as usize] = value,

        _ if address >= MONSTERS && address < SPELL_MEMORY => {
            let monster = &mut world.current_level_mut()
                .monsters[(address - MONSTERS) as usize / 3];
            match (address - MONSTERS) % 3 {
                MONSTER_FLAGS => {
                    monster.kind = unsafe { transmute(value >> 4) };
                    monster.charged    = value & 0b1000 != 0;
                    monster.vulnerable = value & 0b0100 != 0;
                    monster.venomous   = value & 0b0010 != 0;
                    monster.corrupted  = value & 0b0001 != 0;
                },
                MONSTER_POSITION => monster.position = Point::of_byte(value),
                MONSTER_HP => monster.hp = value,
                _ => unreachable!()
            }
        },

        _ if address >= SPELL_MEMORY && address < IDENTIFICATION =>
            for (index, known) in world.player.spell_memory.iter_mut().enumerate() {
                *known = value & (1 << index) != 0
            },

        _ if address >= IDENTIFICATION && address < TIMERS =>
            // TODO: implement identification
            {},

        _ if address >= TIMERS && address < INVENTORY =>
            world.player.timer[(address - TIMERS) as usize] = value,

        _ if address >= INVENTORY && address < DOOR_APPEARANCE =>
            world.player.inventory.slots[(address - INVENTORY) as usize].byte = value,

        DOOR_APPEARANCE =>
            world.door_appearance_byte = value,

        WALL_APPEARANCE =>
            world.wall_appearance_byte = value,

        FLOOR_COLOR =>
            // TODO: remove floor color
            {},

        STAIRS_DELTA  =>
            world.player.stairs_delta = value,

        TIMER_DELTA =>
            world.player.timer_delta = value,

        DAMAGE_OFFSET =>
            world.player.damage_offset = unsafe { transmute(value) },

        0x36 =>
            // TODO: ???
            {},

        0x37 =>
            // TODO: ???
            {},

        TEXT_SYNC =>
            world.player.text_sync = value,

        PLAYER_HP =>
            world.player.hp = value,

        PLAYER_TP =>
            world.player.tp = value,

        PLAYER_XLDEF => {
            world.player.xl = value >> 4;
            world.player.def = upcast_i4(value)
        },

        PLAYER_POSITION =>
            world.player.position = Point::of_byte(value),

        PLAYER_DEPTH =>
            world.player.depth = value,

        METAL_ACID_RESISTANCE => {
            // TODO: use element names
            // note: transmute before shift for sign-extension
            world.player.aptitude[0] = unsafe { transmute::<u8, i8>(value) } >> 4;
            world.player.aptitude[1] = upcast_i4(value)
        },

        FIRE_ELEC_RESISTANCE => {
            // TODO: use element names
            // note: transmute before shift for sign-extension
            world.player.aptitude[2] = unsafe { transmute::<u8, i8>(value) } >> 4;
            world.player.aptitude[3] = upcast_i4(value)
        },

        _ => panic!("memory::poke - invalid address")
    }
}

// pretend the low four bits of our u8 argument are an "i4" and sign-extend to i8
fn upcast_i4(the_i4: u8) -> i8 {
    (unsafe { transmute::<u8, i8>(the_i4) } << 4) >> 4
}


fn report_player_appearance_change(world: &mut World, old: Sprite, new: Sprite) {
    let new_color_name = util::color_name(new.color[0]);
    let new_char_name = util::punctuation_name(new.character);
    if old.character != new.character && old.color != new.color {
        world.log.tell(format!("You turn into {} {}!", util::a_or_an(new_color_name), new_char_name));
    } else if old.character != new.character {
        world.log.tell(format!("You turn into {}!", util::a_or_an(new_char_name)));
    } else if old.color != new.color {
        world.log.tell(format!("You turn {}!", new_color_name));
    }
}

const CP437: &'static [char; 256] = &[
    ' ', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼',
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼',
    ' ', '!', '"', '#', '$', '%', '&', '\'','(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\',']', '^', '_',
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '',
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ',
];

pub fn player_name(world: &World) -> String {
    let mut i: u8 = PLAYER_NAME;
    let mut name = String::new();
    while i < 0x40 {
        let c = peek(world, i);
        if c == 0 { break };
        name.push(CP437[c as usize]);
        i += 1;
    }
    return name;
}
