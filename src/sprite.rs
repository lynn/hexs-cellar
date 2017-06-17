#[derive(Copy, Clone)]
pub enum Color {
    Navy, Green, Teal, Maroon,
    Purple, Brown, Gray, Dark,
    Blue, Lime, Aqua, Red,
    Pink, Yellow, White,
}

#[derive(Copy, Clone)]
pub struct Sprite {
    pub character: char,
    pub color: &'static [Color],
}
