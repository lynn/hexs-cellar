// In Hex's Cellar, the player's spells manipulate an u8[40] of bytes that
// affect the world around her. This enum gives a "RAM map" for that array.

#[derive(Copy, Clone)]
pub enum Addr {
    // Color (3 bits = 8 bright colors) and character (5 bits, offset added to '!')
    PlayerAppearance = 0x00,

    // Begins a char[15].
    PlayerName = 0x01,

    // Monster data is a (struct {u8, u8, u8})[5].
    // So add (n * 3) to these, where 0 <= n <= 4, to get the address for the n-th monster.
    MonsterFlags = 0x10,
    MonsterPosition = 0x11,
    MonsterHP = 0x12,

    // An 8-bit bitmask (there are eight spells).
    SpellMemory = 0x1f,

    // A 32-bit bitmask (there are thirty-two items).
    Identification = 0x20,

    // Begins a u8[4]: one byte for each timer (poison, haste, charge, protect).
    Timers = 0x24,

    // Begins a u8[8].
    Inventory = 0x28,

    // Same as player appearance.
    DoorAppearance = 0x30,
    WallAppearance = 0x31,
    FloorColor = 0x32,

    // u8 to add/subtract from depth when stairs are used; normally 1.
    StaircaseDelta = 0x33,

    // u8 to add to timers each turn if non-zero; normally 0xff.
    TimerDelta = 0x34,

    // s8 to add to each damage roll; normally 0x00.
    DamageOffset = 0x35,

    // ??? = 0x36,
    // ??? = 0x37,

    // The higher this is, the more text gets screwed up.
    TextSync = 0x38,

    // Player stats.
    PlayerHP = 0x39,
    PlayerTP = 0x3a,
    PlayerXLDef = 0x3b, // hi-bits XL, lo-bits Def
    PlayerPosition = 0x3c,
    PlayerDepth = 0x3d,
    MetalAcidResistance = 0x3e, // hi-bits Metal, lo-bits Acid
    FireElecResistance = 0x3f, // hi-bits Fire, lo-bits Elect
}
