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
    pub fn barycentric(
        v: Vector3,
        triangle: (Vector3, Vector3, Vector3),
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
        triangle: (Vector3, Vector3, Vector3),
    ) -> Option<Vector3> {
        let (a, b, c) = triangle;
        match barycentric {
            Some((alpha, beta, gamma)) => Some(a * alpha + b * beta + c * gamma),
            None => None,
        }
    }

    /// Norm of the vector
    pub fn norm(self) -> f64 {
        let Vector3 { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    /// Vector normalized
    pub fn normalize(self) -> Vector3 {
        if self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
        {
            return Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        self / self.norm()
    }

    pub fn normal(u: Vector3, v: Vector3, w: Vector3) -> Vector3 {
        (v - u).cross(w - u).normalize()
    }

    /// Cross product
    pub fn cross(self, other: Vector3) -> Vector3 {
        let Vector3 {
            x: x0,
            y: y0,
            z: z0,
        } = self;

        let Vector3 {
            x: x1,
            y: y1,
            z: z1,
        } = other;

        Vector3 {
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
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Scalar product
impl Mul for Vector3 {
    type Output = f64;
    fn mul(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Product with a scalar
impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/// Product with the inverse of the scalar (u / a := u * (1 / a))
impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
