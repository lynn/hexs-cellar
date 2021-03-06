use byte::BitNumber;
use byte::BitNumber::*;

pub struct Spell {
    name: &'static str,
    description: &'static str,
    bit: BitNumber,
}

pub const ONE: Spell = Spell {
    name: "ONE",
    description: "Writes 0x01 to the selected address.",
    bit: Bit0,
};

pub const CLO: Spell = Spell {
    name: "CLO",
    description: "Clear the leftmost 1 bit of the target value.",
    bit: Bit1,
};

pub const INC: Spell = Spell {
    name: "INC",
    description: "Increment the target value, wrapping from 0xFF to 0x00 on overflow.",
    bit: Bit2,
};

pub const CPN: Spell = Spell {
    name: "CPN",
    description: "Set the target value to that of the byte after it in memory, cycling from 0x3F back to 0x00.",
    bit: Bit3,
};

pub const A9D: Spell = Spell {
    name: "A9D",
    description: "Add 0x9D to the target value, wrapping on overflow. In decimal, this is 157 (unsigned) or -99 (signed).",
    bit: Bit4,
};

pub const REV: Spell = Spell {
    name: "REV",
    description: "Reverses the bits of the target value.",
    bit: Bit5,
};

pub const WLN: Spell = Spell {
    name: "WLN",
    description: "Rewrite the lower nibble of the target value freely.",
    bit: Bit6,
};

pub const WHN: Spell = Spell {
    name: "WHN",
    description: "Rewrite the higher nibble of the target value freely.",
    bit: Bit7,
};
