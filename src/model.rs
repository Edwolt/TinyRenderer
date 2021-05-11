use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::image::Image;
use crate::modules::{mat, Color, Matrix, Vector2, Vector3};

type Element = (isize, Option<isize>, Option<isize>);
/// Representation of a 3D model loaded from a Wavefront obj
pub struct Model {
    /// (v) vertices of the model
    vertices: Vec<Vector3>,

    /// (vt) vertices in the diffuse texture
    textures: Vec<Vector2>,

    /// (vn) Normals of the vertices
    normals: Vec<Vector3>,

    /// (f) A list of faces that is a list of indexes
    /// (vertex, Option<texture_vertex>, Option<normal>)
    faces: Vec<Vec<Element>>,

    /// The diffuse texture image
    diffuse: Option<Image>,
}

impl Model {
    /// Wireframe Render in orthographic projection
    pub fn render_wireframe(&self, image: &mut Image, color: Color) {
        for face in self.faces() {
            let mut v = match face.last() {
                Some(v) => v.0,
                None => continue,
            };
            for (u, _, _) in face {
                image.line(
                    u.to_image_point(image.width, image.height),
                    v.to_image_point(image.width, image.height),
                    color,
                );
                v = u;
            }
        }
    }

    /// Render a image in orthographic projection
    /// using a color
    pub fn render_color(
        &self,
        image: &mut Image,
        viewport: (Vector3, Vector3),
        color: Color,
        light_source: Vector3,
    ) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];
        let transform = matrix_viewport(viewport.0, viewport.1);

        for face in self.faces() {
            let (u, _, _) = face[0];
            let (v, _, _) = face[1];
            let (w, _, _) = face[2];

            let u = (&transform * u.to_matrix(true)).to_vector3();
            let v = (&transform * v.to_matrix(true)).to_vector3();
            let w = (&transform * w.to_matrix(true)).to_vector3();

            let normal = Vector3::normal(u, v, w);
            let intensity = normal * light_source;
            let draw_color = color.light(intensity);

            image.triangle_zbuffer(&mut zbuffer, (u, v, w), draw_color);
        }
    }

    /// Render a image in orthographic projection
    /// using a diffuse texture image
    pub fn render_texture(
        &self,
        image: &mut Image,
        viewport: (Vector3, Vector3),
        light_source: Vector3,
    ) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];
        let transform = matrix_viewport(viewport.0, viewport.1);

        for face in self.faces() {
            let diffuse = match &self.diffuse {
                Some(image) => image,
                None => panic!("Model have no diffuse texture image"),
            };

            let (u, ut, _) = face[0];
            let (v, vt, _) = face[1];
            let (w, wt, _) = face[2];

            let u = (&transform * u.to_matrix(true)).to_vector3();
            let v = (&transform * v.to_matrix(true)).to_vector3();
            let w = (&transform * w.to_matrix(true)).to_vector3();

            let ut = ut.expect("Model have no texture vertex");
            let vt = vt.expect("Model have no texture vertex");
            let wt = wt.expect("Model have no texture vertex");

            let normal = Vector3::normal(u, v, w);
            let intensity = normal * light_source;

            image.triangle_zbuffer_texture(
                &mut zbuffer,
                &diffuse,
                (u, v, w),
                (ut, vt, wt),
                intensity,
            );
        }
    }

    /// Render a image in pespective projection
    /// using a diffuse texture image
    pub fn render_perspective(
        &self,
        image: &mut Image,
        viewport: (Vector3, Vector3),
        camera_z: f64,
        light_source: Vector3,
    ) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];
        let transform = matrix_viewport(viewport.0, viewport.1) * matrix_perspective(camera_z);

        for face in self.faces() {
            let diffuse = match &self.diffuse {
                Some(image) => image,
                None => panic!("Model have no diffuse texture image"),
            };

            let (u, ut, _) = face[0];
            let (v, vt, _) = face[1];
            let (w, wt, _) = face[2];

            let normal = Vector3::normal(u, v, w);
            let intensity = normal * light_source;

            let u = (&transform * u.to_matrix(true)).to_vector3();
            let v = (&transform * v.to_matrix(true)).to_vector3();
            let w = (&transform * w.to_matrix(true)).to_vector3();

            let ut = ut.expect("Model have no texture vertex");
            let vt = vt.expect("Model have no texture vertex");
            let wt = wt.expect("Model have no texture vertex");

            image.triangle_zbuffer_texture(
                &mut zbuffer,
                &diffuse,
                (u, v, w),
                (ut, vt, wt),
                intensity,
            );
        }
    }

    /// Render a image in orthographic projection
    /// using Gouraud shading
    pub fn render_gouraud_color(
        &self,
        image: &mut Image,
        viewport: (Vector3, Vector3),
        color: Color,
        light_source: Vector3,
    ) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];
        let transform = matrix_viewport(viewport.0, viewport.1);

        for face in self.faces() {
            let (u, _, un) = face[0];
            let (v, _, vn) = face[1];
            let (w, _, wn) = face[2];

            let u = (&transform * u.to_matrix(true)).to_vector3();
            let v = (&transform * v.to_matrix(true)).to_vector3();
            let w = (&transform * w.to_matrix(true)).to_vector3();

            image.triangle_zbuffer_gourad_color(
                &mut zbuffer,
                (u, v, w),
                (un, vn, wn),
                color,
                light_source,
            );
        }
    }

    /// Render a image in pespective projection
    /// using a diffuse texture
    /// and Gouraud shading
    pub fn render_gouraud(
        &self,
        image: &mut Image,
        viewport: (Vector3, Vector3),
        camera_z: f64,
        light_source: Vector3,
    ) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];
        let transform = matrix_viewport(viewport.0, viewport.1) * matrix_perspective(camera_z);

        for face in self.faces() {
            let diffuse = match &self.diffuse {
                Some(image) => image,
                None => panic!("Model have no diffuse texture image"),
            };

            let (u, ut, un) = face[0];
            let (v, vt, vn) = face[1];
            let (w, wt, wn) = face[2];

            let u = (&transform * u.to_matrix(true)).to_vector3();
            let v = (&transform * v.to_matrix(true)).to_vector3();
            let w = (&transform * w.to_matrix(true)).to_vector3();

            let ut = ut.expect("Model have no texture vertex");
            let vt = vt.expect("Model have no texture vertex");
            let wt = wt.expect("Model have no texture vertex");

            image.triangle_zbuffer_gourad_texture(
                &mut zbuffer,
                &diffuse,
                (u, v, w),
                (ut, vt, wt),
                (un, vn, wn),
                light_source,
            );
        }
    }
    /// Render a image in pespective projection
    /// using a diffuse texture
    /// and Gouraud shading
    pub fn render_look_at(
        &self,
        image: &mut Image,
        viewport: (Vector3, Vector3),
        eye: Vector3,
        center: Vector3,
        up: Vector3,
        light_source: Vector3,
    ) {
        let mut zbuffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width * image.height) as usize];
        // Transformation chain: Viewport * Projection * View * Model * v
        let mut model_view = matrix_model_view(eye, center, up);
        let transform =
            matrix_viewport(viewport.0, viewport.1) * matrix_perspective(eye.z) * &model_view;

        dbg!(&model_view);
        model_view.transpose(); // Now it'll be used to convert normals
        dbg!(&model_view);
        for face in self.faces() {
            let diffuse = match &self.diffuse {
                Some(image) => image,
                None => panic!("Model have no diffuse texture image"),
            };

            let (u, ut, un) = face[0];
            let (v, vt, vn) = face[1];
            let (w, wt, wn) = face[2];

            let u = (&transform * u.to_matrix(true)).to_vector3();
            let v = (&transform * v.to_matrix(true)).to_vector3();
            let w = (&transform * w.to_matrix(true)).to_vector3();

            let ut = ut.expect("Model have no texture vertex");
            let vt = vt.expect("Model have no texture vertex");
            let wt = wt.expect("Model have no texture vertex");

            let un = (&model_view * un.to_matrix(false)).to_vector3();
            let vn = (&model_view * vn.to_matrix(false)).to_vector3();
            let wn = (&model_view * wn.to_matrix(false)).to_vector3();
            // dbg!(un, vn, wn);

            image.triangle_zbuffer_gourad_texture(
                &mut zbuffer,
                &diffuse,
                (u, v, w),
                (ut, vt, wt),
                (un, vn, wn),
                light_source,
            );
        }
    }

    /// Calculate the normals of all vertices that isn't calculated yet
    ///
    /// Actually this method calculate the normals of all vertices
    /// and update only the normals that is None
    fn compute_normals(&mut self) {
        // Average[i] is the sum of the normals and count of normals of vertices[i]
        let mut average: Vec<(Vector3, usize)> = vec![
            (
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                0
            );
            self.vertices.len()
        ];

        for vec in &mut self.faces {
            let (u_index, _, _) = vec[0];
            let (v_index, _, _) = vec[1];
            let (w_index, _, _) = vec[2];

            let u = self.vertices[convert_index(u_index, self.vertices.len())];
            let v = self.vertices[convert_index(v_index, self.vertices.len())];
            let w = self.vertices[convert_index(w_index, self.vertices.len())];

            let normal = Vector3::normal(u, v, w);

            for i in 0..vec.len() {
                let (vi, vti, vni) = vec[i];
                let index = convert_index(vi, self.vertices.len());

                let (sum, count) = average[index];
                average[index] = ((sum + normal), count + 1);
                if let Some(vni) = vni {
                    let vni = (convert_index(vni, self.normals.len()) + 1) as isize;
                    vec[i] = (vi, vti, Some(vni));
                }
            }
        }

        for vec in &mut self.faces {
            for i in 0..vec.len() {
                let (vi, vti, vni) = vec[i];
                if vni.is_none() {
                    let v_index = convert_index(vi, self.vertices.len());
                    let (sum, count) = average[v_index];
                    self.normals.push(sum / (count as f64));

                    let vni = Some(self.normals.len() as isize);
                    vec[i] = (vi, vti, vni);
                }
            }
        }
    }

    /// Create a model from a Wavefront obj file
    /// and use the Truevision TGA file in texture_path if it isn't None
    pub fn new(model_path: &str, texture_path: Option<&str>) -> std::io::Result<Model> {
        let diffuse = match texture_path {
            Some(path) => Some(Image::load_tga(path)?),
            None => None,
        };

        let file = BufReader::new(File::open(model_path)?);

        let mut model = Model {
            vertices: Vec::new(),
            faces: Vec::new(),
            textures: Vec::new(),
            normals: Vec::new(),
            diffuse,
        };

        let mut no_computed_normals = false;
        for line in file.lines() {
            let line = line?;
            let mut data = line.split(" ").filter(|string| !string.is_empty());

            match data.next() {
                Some("v") => model.vertices.push({
                    /// Function to reduce repeated code
                    fn v_parse(data: Option<&str>) -> f64 {
                        data.expect(
                            "Invalid Wavefront Obj: Vertex have less than three coordinates",
                        )
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate isn't a float")
                    }

                    Vector3 {
                        x: v_parse(data.next()),
                        y: v_parse(data.next()),
                        z: v_parse(data.next()),
                    }
                }),
                Some("vt") => model.textures.push({
                    /// Function to reduce repeated code
                    fn vt_parse(data: Option<&str>) -> f64 {
                        data.expect(
                            "Invalid Wavefront Obj: Texture Vertex have less than two coordinates",
                        )
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Texture Vertex coordinate isn't a float")
                    }

                    Vector2 {
                        x: vt_parse(data.next()),
                        y: vt_parse(data.next()),
                    }
                }),
                Some("vn") => model.normals.push({
                    /// Function to reduce repeated code
                    fn vn_parse(data: Option<&str>) -> f64 {
                        data.expect(
                            "Invalid Wavefront Obj: Normal have less than three coordinates",
                        )
                        .trim()
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Normal coordinate isn't a float")
                    }

                    Vector3 {
                        x: vn_parse(data.next()),
                        y: vn_parse(data.next()),
                        z: vn_parse(data.next()),
                    }
                    .normalize()
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
                                no_computed_normals = true;
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

        if no_computed_normals {
            model.compute_normals();
        }

        Ok(model)
    }

    /// Iterator of faces that is (vertex, Option<texture_vertex>, normal)
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
    type Item = Vec<(Vector3, Option<Vector2>, Vector3)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.model.faces.len() {
            return None;
        }

        let model = self.model;
        let face = &model.faces[self.index];
        let mut result: Self::Item = Vec::new();

        for element in face {
            let &(vi, vti, vni) = element;

            let v = self.model.vertices[convert_index(vi, model.vertices.len())];
            let vt = match vti {
                Some(vti) => Some(model.textures[convert_index(vti, model.textures.len())]),
                None => None,
            };
            let vn = model.normals[convert_index(vni.unwrap(), model.normals.len())];

            result.push((v, vt, vn));
        }

        self.index += 1;
        Some(result)
    }
}

/// Matrix that deform the coordinate to make a perspective projection
fn matrix_perspective(camera_z: f64) -> Matrix {
    mat![4, 4 =>
        1.0, 0.0, 0.0,           0.0;
        0.0, 1.0, 0.0,           0.0;
        0.0, 0.0, 1.0,           0.0;
        0.0, 0.0, -1.0/camera_z, 1.0;
    ]
}

/// Matrix that change the size and the position of the model
///
/// The model is mapped onto scree cube
/// [position.x, position.x+size.x] * [position.y, position.y+size.y] * [position.z, position.z+size.z]
fn matrix_viewport(position: Vector3, size: Vector3) -> Matrix {
    let Vector3 { x, y, z } = position;
    // w = width, h = height, d = depth
    let Vector3 { x: w, y: h, z: d } = size;

    mat![4, 4 =>
        w / 2.0, 0.0,     0.0,   x + w / 2.0;
        0.0,     h / 2.0, 0.0,   y + h / 2.0;
        0.0,     0.0,     d/2.0, z + d / 2.0;
        0.0,     0.0,     0.0,   1.0        ;
    ]
}

/// Matrix that convert the coordinate to the frame (center, i', j' k')
/// Where eye is a point and the camera is in eye pointing to the center
/// and the vector up is in vertical
///
/// eye is a point, center is a point, up is a vector
fn matrix_model_view(eye: Vector3, center: Vector3, up: Vector3) -> Matrix {
    // The problem is:
    // The origin of the new frame is the point C (center)
    // the point E (eye) is in the z-axis of the frame and
    // the vector u (up) has the x coordinate equals to zero
    // because it's in vertical

    // We want a matrix M that v' = v * M where v is in standard basis
    // and v' is in the basis (i', j', k')
    //
    // We know that
    // | 1 |           | 0 |           | 0 |
    // | 0 | = M * i', | 1 | = M * j', | 0 | = M * k'
    // | 0 |           | 0 |           | 1 |
    //
    // Then M (that is unique because it convert basis) is:
    // | i'x  i'y  i'z |
    // | j'x  j'y  j'z |
    // | k'x  k'y  k'z |
    //
    // For example, i' * M =
    // | i'x  i'y  i'z | | i'x |   | ||i'||  |   | 1 |
    // | j'x  j'y  j'z | | i'y | = | i' * j' | = | 0 |
    // | k'x  k'y  k'z | | i'z |   | i' * k' |   | 0 |
    //
    // To convert the coordinantes of the point P in the frame (O, i, j, k)
    // to P' in the frame (C, i', j', k')
    // first we move c to the origin O then we multiply by M
    // P' = M * (P - C)
    //
    // It's the same that
    // | P'x*r |   | i'x  i'y  i'z  0 | | 1  0  0  -cx |
    // | P'y*r | = | j'x  j'y  j'z  0 | | 0  1  0  -cy |
    // | P'z*r |   | k'x  k'y  k'z  0 | | 0  0  1  -cz |
    // |   r   |   |  0    0    0   1 | | 0  0  0   1  |

    // k' = CE / || CE ||
    // i' = (u ^ k') / ||u ^ k'||
    // j' = k' ^ i'

    let k_ = (eye - center).normalize();
    let i_ = up.cross(k_).normalize();
    let j_ = k_.cross(i_).normalize(); // Don't need to be normalized

    let m = mat![4, 4=>
        i_.x, i_.y, i_.z, 0.0;
        j_.x, j_.y, j_.z, 0.0;
        k_.x, k_.y, k_.z, 0.0;
        0.0,  0.0,  0.0,  1.0;
    ];

    let t = mat![4, 4 =>
        1.0, 0.0, 0.0, -center.x;
        0.0, 1.0, 0.0, -center.y;
        0.0, 0.0, 1.0, -center.z;
        0.0, 0.0, 0.0, 1.0;
    ];

    return m * t; // ModelView
}

/// Convert a isize 1-based index into a usize 0-based index
///
/// The input can be negative with -1 meaning the last, -2 meaning the last but one, ...
fn convert_index(index: isize, max: usize) -> usize {
    if index >= 0 {
        (index - 1) as usize
    } else {
        ((max as isize) + index) as usize
    }
}
