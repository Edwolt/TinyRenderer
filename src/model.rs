use std::fs::File;
use std::io::Read;

#[derive(Clone)]
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
                        .expect("Invalid Wavefront Obj: Vertex should have x coordinate")
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate should be a float"),
                    y: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex should have y coordinate")
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate should be a float"),
                    z: data
                        .next()
                        .expect("Invalid Wavefront Obj: Vertex should have z coordinate")
                        .parse::<f64>()
                        .expect("Invalid Wavefront Obj: Vertex coordinate should be a float"),
                }),
                Some("f") => {
                    faces_index.push(Vec::new());
                    for i in data {
                        faces_index.last_mut().unwrap().push(
                            i.split("/")
                                .next()
                                .expect("Invalid Wavefront Obj: Face should have a Vertex ")
                                .parse::<isize>()
                                .expect("Invalid Wavefront Obj: The Face's vertex index sohuld be a integer"),
                        );
                    }
                }
                _ => continue,
            }
        }

        for face in faces_index {
            model.faces.push(Vec::new());
            for index in face {
                model.faces.last_mut().unwrap().push(
                    model.vertices[if index > 0 {
                        index as usize
                    } else {
                        assert!((model.vertices.len() as isize) - index > 0);
                        ((model.vertices.len() as isize) - index) as usize
                    } - 1]
                        .clone(),
                )
            }
        }

        Ok(model)
    }
}
