use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Cross product
    pub const fn cross(self, other: Point) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Product with a scalar
impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Scalar product
impl Mul for Point {
    type Output = i32;
    fn mul(self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y
    }
}
