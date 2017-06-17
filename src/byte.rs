#[derive(Copy, Clone)]
pub enum BitNumber {
    Bit0, Bit1, Bit2, Bit3, Bit4, Bit5, Bit6, Bit7
}

impl BitNumber {
    pub fn mask(self) -> u8 {
        match self {
            BitNumber::Bit0 => 0b00000001,
            BitNumber::Bit1 => 0b00000010,
            BitNumber::Bit2 => 0b00000100,
            BitNumber::Bit3 => 0b00001000,
            BitNumber::Bit4 => 0b00010000,
            BitNumber::Bit5 => 0b00100000,
            BitNumber::Bit6 => 0b01000000,
            BitNumber::Bit7 => 0b10000000,
        }
    }

    pub fn char(self) -> char {
        match self {
            BitNumber::Bit0 => '0',
            BitNumber::Bit1 => '1',
            BitNumber::Bit2 => '2',
            BitNumber::Bit3 => '3',
            BitNumber::Bit4 => '4',
            BitNumber::Bit5 => '5',
            BitNumber::Bit6 => '6',
            BitNumber::Bit7 => '7',
        }
    }
}


pub fn get(byte: u8, bit: BitNumber) -> bool {
    byte & bit.mask() != 0u8
}

pub fn flip(byte: u8, bit: BitNumber) -> u8 {
    byte ^ bit.mask()
}
