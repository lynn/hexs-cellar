use grid::WIDTH;

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn of_byte(index: u8) -> Point {
        Point(index as i32 % WIDTH as i32, index as i32 / WIDTH as i32)
    }
    pub fn as_byte(self) -> u8 {
        let Point(x, y) = self;
        y as u8 * WIDTH as u8 + x as u8
    }
}
