use std::ops::{Add, Div, Mul, Sub};

use super::{Point, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// Caculate barycentric coordinates of a vector v using the triangle ΔABC
    pub fn barycentric(
        v: Vector2,
        triangle: (Vector2, Vector2, Vector2),
    ) -> Option<(f64, f64, f64)> {
        // Barycentric coordinates is the (α, β, 𝛾) where
        // P = α*A + β*B + 𝛾*C and α + β + 𝛾 = 1

        let (a, b, c) = triangle;

        let ab = b - a;
        let ac = c - a;
        let va = a - v;

        let vec_x = Vector3 {
            x: ab.x,
            y: ac.x,
            z: va.x,
        };
        let vec_y = Vector3 {
            x: ab.y,
            y: ac.y,
            z: va.y,
        };

        let Vector3 { x: u, y: v, z } = vec_x.cross(vec_y);

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
        triangle: (Vector2, Vector2, Vector2),
    ) -> Option<Vector2> {
        let (a, b, c) = triangle;
        match barycentric {
            Some((alpha, beta, gamma)) => Some(a * alpha + b * beta + c * gamma),
            None => None,
        }
    }

    /// Norm of the Vertex
    pub fn norm(self) -> f64 {
        let Vector2 { x, y } = self;
        (x * x + y * y).sqrt()
    }

    /// Vertex normalized
    pub fn normalize(self) -> Vector2 {
        if self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON {
            Vector2 { x: 0.0, y: 0.0 }
        } else {
            self / self.norm()
        }
    }

    /// Cross product norm with z = 0
    pub fn cross(self, other: Vector2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Convert a Vector2 with x, y ∈ [0, 1]
    /// to a point in the image
    /// with x ∈ [0, width]
    /// and  y ∈ [0, height]
    pub fn to_texture_point(self, width: i32, height: i32) -> Point {
        // texture_vertex vary from 0 to 1
        let x = self.x * ((width - 1) as f64);
        let y = self.y * ((height - 1) as f64);
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Scalar product
impl Mul for Vector2 {
    type Output = f64;
    fn mul(self, other: Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Product with a scalar
impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, other: f64) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Product with the inverse of the scalar (u / a means u * (1 / a))
impl Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, other: f64) -> Vector2 {
        Vector2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
