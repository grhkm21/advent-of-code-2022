use std::ops::{Add, Sub};

// TODO: Extract this into utils.rs
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn dirs4(&self) -> Vec<Point> {
        DIRS4.iter().map(|d| *self + *d).collect()
    }

    pub fn dirs8(&self) -> Vec<Point> {
        DIRS8.iter().map(|d| *self + *d).collect()
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub const DIRS4: [Point; 4] = [
    Point::new(-1, 0),
    Point::new(1, 0),
    Point::new(0, -1),
    Point::new(0, 1),
];

pub const DIRS8: [Point; 8] = [
    Point::new(-1, 0),
    Point::new(-1, -1),
    Point::new(-1, 1),
    Point::new(0, -1),
    Point::new(0, 1),
    Point::new(1, 0),
    Point::new(1, -1),
    Point::new(1, 1),
];
