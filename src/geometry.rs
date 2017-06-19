use std::cmp::{min, max};
use std::ops::{Add, Mul, Div, Sub, Neg};
use grid::WIDTH;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn of_byte(index: u8) -> Point {
        Point(index as i32 % WIDTH as i32, index as i32 / WIDTH as i32)
    }
    pub fn as_byte(self) -> u8 {
        let Point(x, y) = self;
        y as u8 * WIDTH as u8 + x as u8
    }
    pub fn cheby_norm(self) -> i32 {
        let Point(x, y) = self;
        max(x.abs(), y.abs())
    }
    pub fn cheby_dist(self, other: Point) -> i32 {
        (self - other).cheby_norm()
    }
    pub fn taxi_norm(self) -> i32 {
        let Point(x, y) = self;
        x.abs() + y.abs()
    }
    pub fn taxi_dist(self, other: Point) -> i32 {
        (self - other).taxi_norm()
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        match (self, other) {
            (Point(x1, y1), Point(x2, y2)) => Point(x1 + x2, y1 + y2)
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, scale: i32) -> Point {
        match self {
            Point(x, y) => Point(scale * x, scale * y)
        }
    }
}

impl Div<i32> for Point {
    type Output = Point;
    fn div(self, scale: i32) -> Point {
        match self {
            Point(x, y) => Point(x / scale, y / scale)
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        match self {
            Point(x, y) => Point(-x, -y)
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        self + (-other)
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle(pub Point, pub Point);

impl Rectangle{
    pub fn correct(self) -> Rectangle{
        let Rectangle(Point(x1, y1), Point(x2, y2)) = self;
        Rectangle(
            Point(min(x1, x2), min(y1, y2)),
            Point(max(x1, x2), max(y1, y2)) )
    }

    pub fn grow(self, r: i32) -> Rectangle{
        Rectangle(self.0 - Point(r, r), self.1 + Point(r, r))
    }

    pub fn shrink(self, r: i32) -> Rectangle{
        self.grow(-r)
    }

    pub fn width(self) -> i32 {
        let Rectangle(Point(x1, _), Point(x2, _)) = self;
        x2 - x1 + 1
    }

    pub fn height(self) -> i32 {
        let Rectangle(Point(_, y1), Point(_, y2)) = self;
        y2 - y1 + 1
    }

    pub fn area(self) -> i32 {
        self.width() * self.height()
    }

    pub fn contains(self, point: Point) -> bool {
        let Rectangle(Point(x1, y1), Point(x2, y2)) = self;
        let Point(x, y) = point;
        // TODO: use inclusive range syntax syntax once stable
        x1 <= x && x <= x2 && y1 <= y && y <= y2
    }
}

pub struct RectangleIter {
    start: Point,
    width: i32,
    end_index: i32,
    current_index: i32
}

impl Iterator for RectangleIter {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.current_index < self.end_index {
            let current_point = self.start + Point(
                self.current_index % self.width,
                self.current_index / self.width );
            self.current_index += 1;
            Some(current_point)
        } else {
            None
        }
    }
}

impl IntoIterator for Rectangle {
    type Item = Point;
    type IntoIter = RectangleIter;
    fn into_iter(self) -> RectangleIter {
        let Rectangle(start, _) = self;
        RectangleIter {
            start: start,
            width: self.width(),
            end_index: self.area(),
            current_index: 0
        }
    }
}
