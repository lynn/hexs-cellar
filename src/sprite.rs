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
