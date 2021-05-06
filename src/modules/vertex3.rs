use std::ops::{Add, Div, Mul, Sub};

use super::{mat, Matrix, Point};

#[derive(Copy, Clone, Debug)]
pub struct Vertex3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vertex3 {
    /// Caculate barycentric coordinates of a vertex P using the triangle
    pub fn barycentric(
        p: Vertex3,
        triangle: (Vertex3, Vertex3, Vertex3),
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
        triangle: (Vertex3, Vertex3, Vertex3),
    ) -> Option<Vertex3> {
        let (a, b, c) = triangle;
        match barycentric {
            Some((alpha, beta, gamma)) => Some(a * alpha + b * beta + c * gamma),
            None => None,
        }
    }

    /// Norm of the Vertex
    pub fn norm(self) -> f64 {
        let Vertex3 { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    /// Vertex normalized
    pub fn normalize(self) -> Vertex3 {
        if self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
        {
            return Vertex3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        self / self.norm()
    }

    /// Cross product
    pub fn cross(self, other: Vertex3) -> Vertex3 {
        let Vertex3 {
            x: x0,
            y: y0,
            z: z0,
        } = self;

        let Vertex3 {
            x: x1,
            y: y1,
            z: z1,
        } = other;

        Vertex3 {
            x: y0 * z1 - y1 * z0,
            y: z0 * x1 - x0 * z1,
            z: x0 * y1 - y0 * x1,
        }
    }

    /// Convert to a Point
    pub fn to_point(self, width: i32, height: i32) -> Point {
        // Vertex3 vary from -1 to 1
        let x = (self.x + 1.0) * ((width - 1) as f64) / 2.0;
        let y = (self.y + 1.0) * ((height - 1) as f64) / 2.0;
        Point {
            x: x as i32,
            y: y as i32,
        }
    }

    /// Convert to a Matrix 4x1
    ///
    /// (x, y, z) -> (x, y, z, 1) -> mat![4, 1 => x; y; z; 1]
    pub fn to_matrix(self) -> Matrix {
        mat![4, 1 =>
            self.x;
            self.y;
            self.z;
            1.0;
        ]
    }
}

impl Add for Vertex3 {
    type Output = Vertex3;
    fn add(self, other: Vertex3) -> Vertex3 {
        Vertex3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vertex3 {
    type Output = Vertex3;
    fn sub(self, other: Vertex3) -> Vertex3 {
        Vertex3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Scalar product
impl Mul for Vertex3 {
    type Output = f64;
    fn mul(self, other: Vertex3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Product with a scalar
impl Mul<f64> for Vertex3 {
    type Output = Vertex3;
    fn mul(self, other: f64) -> Vertex3 {
        Vertex3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/// Product with the inverse of the scalar (u / a := u * (1 / a))
impl Div<f64> for Vertex3 {
    type Output = Vertex3;
    fn div(self, other: f64) -> Vertex3 {
        Vertex3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
