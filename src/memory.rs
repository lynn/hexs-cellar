use std::mem::transmute;
use world::World;
use geometry::Point;

// In Hex's Cellar, the player's spells manipulate an u8[40] of bytes that
// affect the world around her. This file gives a "RAM map" for that array.

// Color (3 bits = 8 bright colors) and character (5 bits, offset added to '!')
pub const PLAYER_APPEARANCE: u8 = 0x00;

// Begins a char[15].
pub const PLAYER_NAME: u8 = 0x01;

// Monster data is a (struct {u8, u8, u8})[5].
// So add (n * 3) to these, where 0 <= n <= 4, to get the address for the n-th monster.
pub const MONSTER_FLAGS: u8 = 0x10;
pub const MONSTER_POSITION: u8 = 0x11;
pub const MONSTER_HP: u8 = 0x12;

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

        _ if address >= PLAYER_NAME && address < MONSTER_FLAGS =>
            world.player.name[(address - PLAYER_NAME) as usize],

        _ if address >= MONSTER_FLAGS && address < SPELL_MEMORY =>
            // TODO: implement monsters
            0,

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

        _ => panic!("memory::peek - invalid address")
    }
}


pub fn poke(world: &mut World, address: u8, value: u8) {
    match address {
        PLAYER_APPEARANCE =>
            world.player_appearance_byte = value,

        _ if address >= PLAYER_NAME && address < MONSTER_FLAGS =>
            world.player.name[(address - PLAYER_NAME) as usize] = value,

        _ if address >= MONSTER_FLAGS && address < SPELL_MEMORY =>
            // TODO: implement monsters
            {},

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
