mod modules;
use modules::{Color, Vertex};

mod image;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 500;
    let height = 500;

    let mut wire = Image::new(width, height);
    let mut image = Image::new(width, height);

    println!("Opening model");
    let model = Model::new("obj/african_head.obj").expect("Can't open model");

    println!("Wireframe");
    println!("> Rendering");
    model.wireframe(&mut wire, Color::hex(b"#FFF"));

    println!("> Saving");
    wire.save("wireframe.bmp").expect("Can't save the image");

    println!("Image");
    println!("> Rendering");
    model.render(
        &mut image,
        Vertex {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Color::hex(b"#FFF"),
    );

    println!("> Saving");
    image.save("image.bmp").expect("Can't save the image");

    println!("Image created with success");
}
