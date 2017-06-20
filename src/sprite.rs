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

pub const HIDDEN: Sprite = Sprite {
    character: ' ',
    color: DARK
};

impl Sprite {
    pub fn of_byte(appearance: u8, bright: bool) -> Self {
        Sprite {
            character: ('!' as u8 + (appearance & 0b00011111)) as char,
            color: match (appearance >> 5, bright) {
                (0b000, false) => DARK,
                (0b001, false) => NAVY,
                (0b010, false) => GREEN,
                (0b011, false) => TEAL,
                (0b100, false) => MAROON,
                (0b101, false) => PURPLE,
                (0b110, false) => BROWN,
                (0b111, false) => GRAY,
                (0b000, true)  => DARK,
                (0b001, true)  => BLUE,
                (0b010, true)  => LIME,
                (0b011, true)  => AQUA,
                (0b100, true)  => RED,
                (0b101, true)  => PINK,
                (0b110, true)  => YELLOW,
                _              => WHITE,
            }
        }
    }

    pub fn darken(self, shade: bool) -> Self {
        if shade {
            Sprite {
                character: self.character,
                color: DARK
            }
        } else {
            self
        }
    }
}
