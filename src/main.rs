#![allow(dead_code)]

mod modules;
use modules::{Color, Vertex};

mod image;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 500;
    let height = 500;

    // let mut wire = Image::new(width, height);
    let mut img = Image::new(width, height);

    println!("Opening model");
    let model = Model::new("obj/african_head.obj").expect("Can't open model");

    // println!("Wireframe");
    // println!("> Rendering");
    // model.wireframe(&mut wire, Color::hex(b"#FFF"));

    // println!("> Saving");
    // wire.save_tga("wireframe.tga", true)
    //     .expect("Can't save the image");

    println!("Image");
    println!("> Rendering");
    model.render(
        &mut img,
        Vertex {
            x: 0.3,
            y: 0.0,
            z: -1.0,
        },
        Color::hex(b"#FFF"),
    );

    println!("> Saving");
    img.save_tga("image.tga", true)
        .expect("Can't save the image");

    println!("Image created with success");
}
