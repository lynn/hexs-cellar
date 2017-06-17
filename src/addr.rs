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
