#[derive(Copy, Clone)]
pub enum BitNumber {
    Bit0, Bit1, Bit2, Bit3, Bit4, Bit5, Bit6, Bit7
}

pub fn mask(bit: BitNumber) -> u8 {
    match bit {
        Bit0 => 0b00000001,
        Bit1 => 0b00000010,
        Bit2 => 0b00000100,
        Bit3 => 0b00001000,
        Bit4 => 0b00010000,
        Bit5 => 0b00100000,
        Bit6 => 0b01000000,
        Bit7 => 0b10000000,
    }
}

pub fn get(byte: u8, bit: BitNumber) -> bool {
    byte & mask(bit) != 0u8
}

pub fn flip(byte: u8, bit: BitNumber) -> u8 {
    byte ^ mask(bit)
}
