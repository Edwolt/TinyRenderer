use std::fs::File;
use std::io::Read;
use std::ops;

use crate::image::Color;
use crate::image::Image;
use crate::image::Point;

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
        if self.x == 0f64 && self.y == 0f64 && self.z == 0f64 {
            return Vertex {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            };
        }
        self * (1f64 / self.norm())
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

    fn to_point(&self, width: i32, height: i32) -> Point {
        let x = (self.x + 1f64) * (width as f64) / 2f64;
        let y = (self.y + 1f64) * (height as f64) / 2f64;
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl ops::Add for Vertex {
    type Output = Vertex;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vertex {
    type Output = Vertex;
    fn sub(self, other: Vertex) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Vertex {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ops::Mul<f64> for Vertex {
    type Output = Vertex;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Vec<Vertex>>,
}

impl Model {
    pub fn new(path: &str) -> std::io::Result<Model> {
        let mut file = File::open(path)?;

        let mut lines: String = String::new();
        file.read_to_string(&mut lines)?;
        let lines = lines.split('\n');

        let mut model = Model {
            vertices: Vec::new(),
            faces: Vec::new(),
        };

        let mut faces_index: Vec<Vec<isize>> = Vec::new();

        for line in lines {
            let mut data = line.split(" ");
            match data.next() {
                Some("v") => model.vertices.push(Vertex {
                    x: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex must have x coordinate")
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate must be a float"),
                    y: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex must have y coordinate")
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate must be a float"),
                    z: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex must have z coordinate")
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate must be a float"),
                }),
                Some("f") => {
                    faces_index.push(
                        data.map(|index| {
                                index
                                    .split("/")
                                    .next()
                                    .expect("Invalid Wavefront Obj: The Face's vertex index must be a integer")
                                    .parse::<isize>()
                                    .expect("Invalid Wavefront Obj: The Face's vertex index must be a integer")
                            }
                        ).collect::<Vec<isize>>()
                    );
                }
                _ => continue,
            }
        }

        for face in faces_index {
            model.faces.push(
                face.iter()
                    .map(|index| {
                        let index = if *index > 0 {
                            *index as usize
                        } else {
                            let i = (model.vertices.len() as isize) - *index;
                            assert!(i > 0, "Invalid Wavefront Obj: Invalid Vertex index");
                            i as usize
                        };
                        model.vertices[index - 1]
                    })
                    .collect::<Vec<Vertex>>(),
            );
        }

        Ok(model)
    }

    pub fn wireframe_render(&self, image: &mut Image, color: Color) {
        for face in &self.faces {
            let mut v = match face.last() {
                Some(v) => v,
                None => continue,
            };
            for u in face {
                image.line(
                    u.to_point(image.width, image.height),
                    v.to_point(image.width, image.height),
                    color,
                );
                v = u;
            }
        }
    }

    pub fn render(&self, image: &mut Image, light: Vertex, color: Color) {
        for face in &self.faces {
            let u = face[0];
            let v = face[1];
            let w = face[2];
            let normal = (v - u).cross(w - u).normalize();
            let intensity = normal * light;

            if intensity > 0f64 {
                // if intensity < 0 it's
                image.triangle(
                    u.to_point(image.width, image.height),
                    v.to_point(image.width, image.height),
                    w.to_point(image.width, image.height),
                    color * intensity,
                )
            }
        }
    }
}
