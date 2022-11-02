use std::ops::{Add, Div, Mul, Sub};

use super::{mat, Matrix, Point};

/// Represents a Vector or a Point with 3 Real coordinates
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Caculate barycentric coordinates of a vector v using the triangle
    pub fn barycentric(p: Self, triangle: (Self, Self, Self)) -> Option<(f64, f64, f64)> {
        // Barycentric coordinates is the (α, β, 𝛾) where
        // P = α*A + β*B + 𝛾*C and α + β + 𝛾 = 1

        let (a, b, c) = triangle;

        let ab = b - a;
        let ac = c - a;
        let pa = a - p;

        let vec_x = Self {
            x: ab.x,
            y: ac.x,
            z: pa.x,
        };
        let vec_y = Self {
            x: ab.y,
            y: ac.y,
            z: pa.y,
        };

        let Self { x: u, y: v, z } = vec_x.cross(vec_y);

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
        triangle: (Self, Self, Self),
    ) -> Option<Self> {
        let (a, b, c) = triangle;
        match barycentric {
            Some((alpha, beta, gamma)) => Some(a * alpha + b * beta + c * gamma),
            None => None,
        }
    }

    /// Norm of the vector
    pub fn norm(self) -> f64 {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    /// Vector normalized
    pub fn normalize(self) -> Self {
        if self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
        {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        self / self.norm()
    }

    pub fn normal(u: Self, v: Self, w: Self) -> Self {
        (v - u).cross(w - u).normalize()
    }

    /// Cross product
    pub fn cross(self, other: Self) -> Self {
        let Self {
            x: x0,
            y: y0,
            z: z0,
        } = self;

        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = other;

        Self {
            x: y0 * z1 - y1 * z0,
            y: z0 * x1 - x0 * z1,
            z: x0 * y1 - y0 * x1,
        }
    }

    /// Convert a Vector3 { x, y, z } to a Point { x, y }
    pub fn to_point(self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    /// Convert a Vector3 with x, y ∈ [-1, 1]
    /// to a point in the image
    /// with x ∈ [0, width]
    /// and  y ∈ [0, height]
    pub fn to_image_point(self, width: i32, height: i32) -> Point {
        let x = (self.x + 1.0) * ((width - 1) as f64) / 2.0;
        let y = (self.y + 1.0) * ((height - 1) as f64) / 2.0;
        Point {
            x: x as i32,
            y: y as i32,
        }
    }

    /// Convert a point represented by a Vector3
    /// to a Matrix 4x1
    ///
    /// If it's represent a point:
    /// <pre>
    ///              | x |
    /// (x, y, z) -> | y |
    ///              | z |
    ///              | 1 |
    /// </pre>
    ///
    /// <pre>
    /// if it's represent a vector
    ///              | x |
    /// (x, y, z) -> | y |
    ///              | z |
    ///              | 0 |
    /// </pre>
    pub fn to_matrix(self, is_a_point: bool) -> Matrix {
        let w = if is_a_point { 1.0 } else { 0.0 };
        mat![4, 1 =>
            self.x;
            self.y;
            self.z;
            w;
        ]
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Scalar product
impl Mul for Vector3 {
    type Output = f64;
    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Product with a scalar
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/// Product with the inverse of the scalar (u / a := u * (1 / a))
impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
