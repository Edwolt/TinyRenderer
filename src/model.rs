use std::fs::File;
use std::io::Read;

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
        fn vertex_to_point(&Vertex { x, y, z: _z }: &Vertex, width: i32, height: i32) -> Point {
            let x = (x + 1f64) * (width as f64) / 2f64;
            let y = (y + 1f64) * (height as f64) / 2f64;
            Point {
                x: x as i32,
                y: y as i32,
            }
        }

        for face in &self.faces {
            let mut v = match face.last() {
                Some(v) => v,
                None => continue,
            };
            for u in face {
                image.line(
                    vertex_to_point(u, image.width, image.height),
                    vertex_to_point(v, image.width, image.height),
                    color,
                );
                v = u;
            }
        }
    }
}
