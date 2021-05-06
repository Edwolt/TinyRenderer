use std::ops::{Add, Div, Mul, Sub};

use super::{Point, Vertex3};

#[derive(Copy, Clone, Debug)]
pub struct Vertex2 {
    pub x: f64,
    pub y: f64,
}

impl Vertex2 {
    /// Caculate barycentric coordinates of a vertex P using the triangle ΔABC
    pub fn barycentric(
        p: Vertex2,
        triangle: (Vertex2, Vertex2, Vertex2),
    ) -> Option<(f64, f64, f64)> {
        // Barycentric coordinates is the (α, β, 𝛾) where
        // P = α*A + β*B + 𝛾*C and α + β + 𝛾 = 1

        let (a, b, c) = triangle;

        let ab = b - a;
        let ac = c - a;
        let pa = a - p;

        let vec_x = Vertex3 {
            x: ab.x,
            y: ac.x,
            z: pa.x,
        };
        let vec_y = Vertex3 {
            x: ab.y,
            y: ac.y,
            z: pa.y,
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
        triangle: (Vertex2, Vertex2, Vertex2),
    ) -> Option<Vertex2> {
        let (a, b, c) = triangle;
        match barycentric {
            Some((alpha, beta, gamma)) => Some(a * alpha + b * beta + c * gamma),
            None => None,
        }
    }

    pub fn norm(self) -> f64 {
        let Vertex2 { x, y } = self;
        (x * x + y * y).sqrt()
    }

    pub fn normalize(self) -> Vertex2 {
        if self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON {
            Vertex2 { x: 0.0, y: 0.0 }
        } else {
            self / self.norm()
        }
    }

    /// Cross product norm with z = 0
    pub fn cross(self, other: Vertex2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn to_point(self, width: i32, height: i32) -> Point {
        // Vertex2 vary from 0 to 1
        let x = self.x * ((width - 1) as f64);
        let y = self.y * ((height - 1) as f64);
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl Add for Vertex2 {
    type Output = Vertex2;
    fn add(self, other: Vertex2) -> Vertex2 {
        Vertex2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vertex2 {
    type Output = Vertex2;
    fn sub(self, other: Vertex2) -> Vertex2 {
        Vertex2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Scalar product
impl Mul for Vertex2 {
    type Output = f64;
    fn mul(self, other: Vertex2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Product with a scalar
impl Mul<f64> for Vertex2 {
    type Output = Vertex2;
    fn mul(self, other: f64) -> Vertex2 {
        Vertex2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Product with the inverse of the scalar (u / a := u * (1 / a))
impl Div<f64> for Vertex2 {
    type Output = Vertex2;
    fn div(self, other: f64) -> Vertex2 {
        Vertex2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
