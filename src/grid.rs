use std::ops::{Index, IndexMut};
use point::Point;

pub const WIDTH: usize = 19;
pub const HEIGHT: usize = 13;

pub struct Grid<T> {
    pub grid: Vec<T>, // In row order.
}

impl<T> Grid<T> {
    pub fn empty() -> Grid<T> {
        Grid {
            grid: Vec::with_capacity(WIDTH * HEIGHT)
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.grid[y * WIDTH + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.grid[y * WIDTH + x]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, Point(x, y): Point) -> &T {
        &self[(x as usize, y as usize)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, Point(x, y): Point) -> &mut T {
        &mut self[(x as usize, y as usize)]
    }
}
