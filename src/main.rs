#![allow(dead_code)]

mod modules;
use modules::{Color, Vertex};

mod image;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 1024;
    let height = 1024;

    let light_source = Vertex {
        x: 0.3,
        y: 0.0,
        z: -1.0,
    };
    let mut wire = Image::new(width, height);
    let mut image = Image::new(width, height);
    let mut render_color = Image::new(width, height);
    let texture = Image::load_tga("obj/african_head_diffuse.tga").expect("Can't load texture");

    println!("Opening model");
    let model = Model::new("obj/african_head.obj").expect("Can't open model");

    println!("Wireframe");
    println!("> Rendering");
    model.wireframe(&mut wire, Color::hex(b"#FFF"));

    println!("> Saving");
    wire.save_tga("wireframe.tga", true)
        .expect("Can't save the image");

    println!("Render Color");
    println!("> Rendering");
    model.render_color(&mut render_color, Color::hex(b"#FFF"), light_source);

    println!("> Saving");
    render_color
        .save_tga("color.tga", true)
        .expect("Can't save the image");

    println!("Image");
    println!("> Rendering");
    model.render_texture(&mut image, &texture, light_source);

    println!("> Saving");
    image
        .save_tga("image.tga", true)
        .expect("Can't save the image");

    println!("Image created with success");
}
