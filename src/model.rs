use std::fs::File;
use std::io::Read;

use crate::image::Image;
use crate::modules::{Color, Matrix, Vertex2, Vertex3};

type Element = (isize, Option<isize>, Option<isize>);
pub struct Model {
    vertices: Vec<Vertex3>,
    textures: Vec<Vertex2>,
    normal: Vec<Vertex3>,
    faces: Vec<Vec<Element>>,
}

impl Model {
    /// Wireframe Render in orthographic projection
    pub fn wireframe(&self, image: &mut Image, color: Color) {
        for face in self.faces() {
            let mut v = match face.last() {
                Some(v) => v.0,
                None => continue,
            };
            for (u, _, _) in face {
                image.line(
                    u.to_point(image.width, image.height),
                    v.to_point(image.width, image.height),
                    color,
                );
                v = u;
            }
        }
    }

    /// Render a image in orthographic projection
    /// using a color
    pub fn render_color(&self, image: &mut Image, color: Color, light: Vertex3) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];

        for face in self.faces() {
            let (u, _, _) = face[0];
            let (v, _, _) = face[1];
            let (w, _, _) = face[2];

            let normal = (w - u).cross(v - u).normalize();
            let intensity = normal * light;
            let draw_color = color.light(intensity);

            image.triangle_zbuffer(&mut zbuffer, (u, v, w), draw_color);
        }
    }

    /// Render a image in orthographic projection
    /// using a diffuse texture image
    pub fn render_texture(&self, image: &mut Image, texture: &Image, light: Vertex3) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];

        for face in self.faces() {
            let (u, ut, _) = face[0];
            let (v, vt, _) = face[1];
            let (w, wt, _) = face[2];

            let ut = ut.expect("Model have no texture vertex");
            let vt = vt.expect("Model have no texture vertex");
            let wt = wt.expect("Model have no texture vertex");

            let normal = (w - u).cross(v - u).normalize();
            let intensity = normal * light;

            image.triangle_zbuffer_texture(
                &mut zbuffer,
                &texture,
                intensity,
                (u, v, w),
                (ut, vt, wt),
            );
        }
    }

    fn compute_normal(&mut self) {
        // TODO
        self.normal.push(Vertex3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        for vec in &mut self.faces {
            for i in 0..vec.len() {
                let (v, vt, vn) = vec[i];
                if vn.is_none() {
                    vec[i] = (v, vt, Some(self.normal.len() as isize));
                }
            }
        }
    }

    /// Render a image in pespective projection
    /// using a diffuse texture image
    pub fn render_perspective(
        &self,
        image: &mut Image,
        camera_z: f64,
        texture: &Image,
        light: Vertex3,
    ) {
        fn vertex3_to_matrix(Vertex3 { x, y, z }: Vertex3) -> Matrix {
            mat![4, 1 =>
                x;
                y;
                z;
                1.0;
            ]
        }

        fn matrix_to_vertex3(matrix: Matrix) -> Vertex3 {
            let w = matrix.get(3, 0);
            Vertex3 {
                x: matrix.get(0, 0) / w,
                y: matrix.get(1, 0) / w,
                z: matrix.get(2, 0) / w,
            }
        }

        let transform = mat![4, 4 =>
            1.0, 0.0, 0.0,           0.0;
            0.0, 1.0, 0.0,           0.0;
            0.0, 0.0, 1.0,           0.0;
            0.0, 0.0, -1.0/camera_z, 1.0;
        ];

        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];

        for face in self.faces() {
            let (u, ut, _) = face[0];
            let (v, vt, _) = face[1];
            let (w, wt, _) = face[2];

            let ut = ut.expect("Model have no texture vertex");
            let vt = vt.expect("Model have no texture vertex");
            let wt = wt.expect("Model have no texture vertex");

            let up = matrix_to_vertex3(&transform * &vertex3_to_matrix(u));
            let vp = matrix_to_vertex3(&transform * &vertex3_to_matrix(v));
            let wp = matrix_to_vertex3(&transform * &vertex3_to_matrix(w));

            let normal = (w - u).cross(v - u).normalize();
            let intensity = normal * light;

            image.triangle_zbuffer_texture(
                &mut zbuffer,
                &texture,
                intensity,
                (up, vp, wp),
                (ut, vt, wt),
            );
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
            normal: Vec::new(),
        };

        let mut no_computed_normal = false;
        for line in lines {
            let mut data = line.split(" ").filter(|string| !string.is_empty());

            match data.next() {
                Some("v") => model.vertices.push({
                    /// A function to reduce repeated code
                    fn v_parse(data: Option<&str>) -> f64 {
                        data.expect(
                            "Invalid Wavefront Obj: Vertex have less than three coordinates",
                        )
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate isn't a float")
                    }

                    Vertex3 {
                        x: v_parse(data.next()),
                        y: v_parse(data.next()),
                        z: v_parse(data.next()),
                    }
                }),
                Some("vt") => model.textures.push({
                    /// A function to reduce repeated code
                    fn vt_parse(data: Option<&str>) -> f64 {
                        data.expect(
                            "Invalid Wavefront Obj: Texture Vertex have less than two coordinates",
                        )
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Texture Vertex coordinate isn't a float")
                    }

                    Vertex2 {
                        x: vt_parse(data.next()),
                        y: vt_parse(data.next()),
                    }
                }),
                Some("vn") => model.normal.push({
                    /// A function to reduce repeated code
                    fn vn_parse(data: Option<&str>) -> f64 {
                        data.expect(
                            "Invalid Wavefront Obj: Normal have less than three coordinates",
                        )
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Normal coordinate isn't a float")
                    }

                    Vertex3 {
                        x: vn_parse(data.next()),
                        y: vn_parse(data.next()),
                        z: vn_parse(data.next()),
                    }
                }),
                Some("f") => {
                    let mut face: Vec<Element> = Vec::new();
                    for element in data {
                        let mut element = element.split("/").filter(|string| !string.is_empty());
                        let vertex_index = element
                            .next()
                            .expect("Invalid Wavefront Obj: no face vertex index")
                            .trim()
                            .parse::<isize>()
                            .expect(
                                "Invalid Wavefront Obj: The face vertex index isn't an integer",
                            );

                        let texture_index = match element.next() {
                            Some(string) => {
                                Some(string
                                    .trim()
                                    .parse::<isize>()
                                    .expect(
                                        "Invalid Wavefront Obj: The face texture vertex index isn't an integer"
                                    )
                                )
                            }
                            None => None,
                        };

                        let normal_index = match element.next() {
                            Some(string) => {
                                Some(string
                                    .trim()
                                    .parse::<isize>()
                                    .expect(
                                        "Invalid Wavefront Obj: The face normal vertex index isn't an integer"
                                    )
                                )
                            }
                            None => {
                                no_computed_normal = true;
                                None
                            },
                        };

                        face.push((vertex_index, texture_index, normal_index));
                    }
                    model.faces.push(face);
                }
                _ => continue,
            }
        }

        if no_computed_normal {
            model.compute_normal();
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
    type Item = Vec<(Vertex3, Option<Vertex2>, Vertex3)>;
    fn next(&mut self) -> Option<Self::Item> {
        /// Convert a isize 1-based index into a usize 0-based index
        ///
        /// The input can be negative with -1 meaning the last, -2 meaning the last but one, ...
        fn convert(index: isize, max: usize) -> usize {
            if index >= 0 {
                (index - 1) as usize
            } else {
                ((max as isize) + index) as usize
            }
        }

        if self.index >= self.model.faces.len() {
            return None;
        }

        let model = self.model;
        let face = &model.faces[self.index];
        let mut result: Self::Item = Vec::new();

        for element in face {
            let &(vertex_index, texture_index, normal_index) = element;
            // Vertex
            let vertex = {
                let vertex_index = convert(vertex_index, model.vertices.len());
                self.model.vertices[vertex_index]
            };

            // Texture
            let texture = match texture_index {
                Some(texture_index) => {
                    let texture_index = convert(texture_index, model.textures.len());
                    Some(model.textures[texture_index])
                }
                None => None,
            };

            // Normal
            let normal = {
                let normal_index = convert(normal_index.unwrap(), model.normal.len());
                model.normal[normal_index]
            };

            result.push((vertex, texture, normal));
        }

        self.index += 1;
        Some(result)
    }
}
