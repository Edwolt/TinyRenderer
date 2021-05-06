use std::ops::{Add, Div, Mul, Sub};

use super::Vertex3;

// Using i32 instead of u32 make signed calculation simpler
// For exmaple in the cross product
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Caculate barycentric coordinates of a point P using the triangle
    pub fn barycentric(p: Point, triangle: (Point, Point, Point)) -> Option<(f64, f64, f64)> {
        // Barycentric coordinates is the (α, β, 𝛾) where
        // P = α*A + β*B + 𝛾*C and α + β + 𝛾 = 1

        let (a, b, c) = triangle;

        let ab = b - a;
        let ac = c - a;
        let pa = a - p;

        let vec_x = Vertex3 {
            x: ab.x as f64,
            y: ac.x as f64,
            z: pa.x as f64,
        };
        let vec_y = Vertex3 {
            x: ab.y as f64,
            y: ac.y as f64,
            z: pa.y as f64,
        };

        let Vertex3 { x: u, y: v, z } = vec_x.cross(vec_y);

        // z can't be zero
        if z.abs() < f64::EPSILON {
            None
        } else {
            let u = u / z;
            let v = v / z;
            Some((1.0 - u - v, u, v))
        }
    }

    /// Linear interpolation
    pub fn lerp(
        barycentric: Option<(f64, f64, f64)>,
        triangle: (Point, Point, Point),
    ) -> Option<Point> {
        let (a, b, c) = triangle;
        match barycentric {
            Some((alpha, beta, gamma)) => Some(Point {
                x: ((a.x as f64) * alpha + (b.x as f64) * beta + (c.x as f64) * gamma) as i32,
                y: ((a.y as f64) * alpha + (b.y as f64) * beta + (c.y as f64) * gamma) as i32,
            }),
            None => None,
        }
    }

    /// Cross product norm with z = 0
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

/// Scalar product
impl Mul for Point {
    type Output = i32;
    fn mul(self, other: Point) -> i32 {
        self.x * other.x + self.y * other.y
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

/// Product with a scalar
impl Div<i32> for Point {
    type Output = Point;
    fn div(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
