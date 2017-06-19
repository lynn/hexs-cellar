#[derive(Copy, Clone)]
pub enum Color {
    Navy, Green, Teal, Maroon,
    Purple, Brown, Gray, Dark,
    Blue, Lime, Aqua, Red,
    Pink, Yellow, White,
}

use self::Color::*;
pub const NAVY: &'static [Color] = &[Navy];
pub const GREEN: &'static [Color] = &[Green];
pub const TEAL: &'static [Color] = &[Teal];
pub const MAROON: &'static [Color] = &[Maroon];
pub const PURPLE: &'static [Color] = &[Purple];
pub const BROWN: &'static [Color] = &[Brown];
pub const GRAY: &'static [Color] = &[Gray];
pub const DARK: &'static [Color] = &[Dark];
pub const BLUE: &'static [Color] = &[Blue];
pub const LIME: &'static [Color] = &[Lime];
pub const AQUA: &'static [Color] = &[Aqua];
pub const RED: &'static [Color] = &[Red];
pub const PINK: &'static [Color] = &[Pink];
pub const YELLOW: &'static [Color] = &[Yellow];
pub const WHITE: &'static [Color] = &[White];
pub const GLITCH: &'static [Color] = &[Yellow, Pink];
pub const GOLD: &'static [Color] = &[Red, Yellow, Brown, White];
pub const SAPPHIRE: &'static [Color] = &[Blue, Navy, Aqua, Teal];

#[derive(Copy, Clone)]
pub struct Sprite {
    pub character: char,
    pub color: &'static [Color],
}

impl Sprite {
    pub fn of_byte(appearance: u8) -> Self {
        Sprite {
            character: ('@' as u8 + (appearance & 0b00011111)) as char,
            color: match appearance >> 5 {
                0b000 => DARK,
                0b001 => BLUE,
                0b010 => LIME,
                0b011 => AQUA,
                0b100 => RED,
                0b101 => PINK,
                0b110 => YELLOW,
                _     => WHITE
            }
        }
    }
}
