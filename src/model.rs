use std::fs::File;
use std::io::Read;

use crate::image::Image;
use crate::modules::{Color, Vertex, Vertex2};

pub struct Model {
    vertices: Vec<Vertex>,
    textures: Vec<Vertex2>,
    faces: Vec<Vec<(isize, Option<isize>)>>,
}

impl Model {
    /// Wireframe Render
    pub fn wireframe(&self, image: &mut Image, color: Color) {
        for face in self.faces() {
            let mut v = match face.last() {
                Some(v) => v.0,
                None => continue,
            };
            for (u, _) in face {
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
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];

        for face in self.faces() {
            let (u, _) = face[0];
            let (v, _) = face[1];
            let (w, _) = face[2];
            let normal = (w - u).cross(v - u).normalize();
            let intensity = normal * light;

            if intensity > 0.0 {
                // if intensity < 0 it's behind the scene
                image.triangle_zbuffer(&mut zbuffer, u, v, w, color * intensity);
            } else {
                image.triangle_zbuffer(&mut zbuffer, u, v, w, Color::hex(b"#000"));
            }
        }
    }

    pub fn new(path: &str) -> std::io::Result<Model> {
        let mut file = File::open(path)?;

        let mut lines: String = String::new();
        file.read_to_string(&mut lines)?;
        let lines = lines.split('\n');

        let mut model = Model {
            vertices: Vec::new(),
            faces: Vec::new(),
            textures: Vec::new(),
        };

        for line in lines {
            let mut data = line.split(" ").filter(|string| !string.is_empty());
            match data.next() {
                Some("v") => model.vertices.push(Vertex {
                    x: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex must have x coordinate")
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate must be a float"),
                    y: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex must have y coordinate")
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate must be a float"),
                    z: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex must have z coordinate")
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate must be a float"),
                }),
                Some("vt") => model.textures.push(Vertex2 {
                    x: data
                        .next()
                        .expect("Invalid Wavefront Obj: Texture Vertex must have x coordinate")
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Texture Vertex coordinate must be a float"),
                    y: data
                        .next()
                        .expect("Invalid Wavefront Obj: Texture Vertex must have y coordinate")
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Texture Vertex coordinate must be a float"),
                }),
                Some("f") => {
                    model.faces.push(
                        data.map(|element| {
                                let mut element = element.split("/");
                                let vertex_index = element
                                    .next()
                                    .expect("Invalid Wavefront Obj: The Face's vertex index must be a integer")
                                    .trim()
                                    .parse::<isize>()
                                    .expect("Invalid Wavefront Obj: The Face's vertex index must be a integer");

                                let texture_index:Option<isize> = match element.next() {
                                    None => None,
                                    Some(string) => {
                                        Some(string
                                            .trim()
                                            .parse::<isize>()
                                            .expect("Invalid Wavefront Obj: The Face's texture vertex index must be a integer")
                                        )
                                    },
                                };

                                (vertex_index, texture_index)
                            }
                        ).collect::<Vec<(isize, Option<isize>)>>()
                    );
                }
                _ => continue,
            }
        }

        Ok(model)
    }

    pub fn faces(&self) -> FaceIterator {
        FaceIterator {
            model: self,
            index: 0,
        }
    }
}

pub struct FaceIterator<'a> {
    model: &'a Model,
    index: usize,
}

impl<'a> Iterator for FaceIterator<'a> {
    type Item = Vec<(Vertex, Option<Vertex2>)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.model.faces.len() {
            return None;
        }

        let model = self.model;
        let face = &model.faces[self.index];
        let mut result: Self::Item = Vec::new();

        for element in face {
            let &(vertex_index, texture_index) = element;
            let vertex_index = if vertex_index >= 0 {
                vertex_index - 1
            } else {
                model.vertices.len() as isize + vertex_index
            } as usize;
            let vertex = self.model.vertices[vertex_index];

            let texture = match texture_index {
                Some(texture_index) => {
                    let texture_index = if texture_index >= 0 {
                        texture_index - 1
                    } else {
                        model.textures.len() as isize + texture_index
                    } as usize;
                    Some(model.textures[texture_index])
                }
                None => None,
            };

            result.push((vertex, texture));
        }

        self.index += 1;
        Some(result)
    }
}
