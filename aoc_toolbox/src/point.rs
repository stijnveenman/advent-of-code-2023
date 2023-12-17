use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const DOWN: Point = Point { x: 0, y: 1 };
    pub const UP: Point = Point { x: 0, y: -1 };

    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn is_within(&self, lower: &Point, upper: &Point) -> bool {
        self.x >= lower.x && self.y >= lower.y && self.x <= upper.x && self.y <= upper.y
    }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            *self + Point::UP,
            *self + Point::RIGHT,
            *self + Point::DOWN,
            *self + Point::LEFT,
        ]
    }

    pub fn rotate(&self, direction: isize) -> Point {
        Point {
            x: self.y * direction,
            y: self.x * direction,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
