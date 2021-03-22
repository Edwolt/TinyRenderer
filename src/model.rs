use std::fs::File;
use std::io::Read;

use crate::image::Image;
use crate::modules::{Color, Vertex};

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
                    .map(|&index| {
                        let index = if index > 0 {
                            index as usize
                        } else {
                            let i = (model.vertices.len() as isize) - index;
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

    pub fn wireframe(&self, image: &mut Image, color: Color) {
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
                // if intensity < 0 it's behind the scene
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
