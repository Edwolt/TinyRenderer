use std::ops;

use super::Point;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // w: f64,
}

impl Vertex {
    pub fn norm(self) -> f64 {
        let Vertex { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(self) -> Vertex {
        if self.x == 0.0 && self.y == 0.0 && self.z == 0.0 {
            return Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        self * (1.0 / self.norm())
    }

    pub fn cross(self, other: Vertex) -> Vertex {
        let Vertex {
            x: x0,
            y: y0,
            z: z0,
        } = self;

        let Vertex {
            x: x1,
            y: y1,
            z: z1,
        } = other;

        Vertex {
            x: y0 * z1 - y1 * z0,
            y: z0 * x1 - x0 * z1,
            z: x0 * y1 - y0 * x1,
        }
    }

    pub fn to_point(self, width: i32, height: i32) -> Point {
        let x = (self.x + 1.0) * ((width - 1) as f64) / 2.0;
        let y = (self.y + 1.0) * ((height - 1) as f64) / 2.0;
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl ops::Add for Vertex {
    type Output = Vertex;
    fn add(self, other: Vertex) -> Vertex {
        Vertex {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vertex {
    type Output = Vertex;
    fn sub(self, other: Vertex) -> Vertex {
        Vertex {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Scalar product
impl ops::Mul for Vertex {
    type Output = f64;
    fn mul(self, other: Vertex) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// Product with a scalar
impl ops::Mul<f64> for Vertex {
    type Output = Vertex;
    fn mul(self, other: f64) -> Vertex {
        Vertex {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
