mod modules;
use modules::{Color, Point, Vertex};

mod image;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 4;
    let height = 4;
    let mut img = Image::new(width, height);
    img.set(Point { y: 0, x: 0 }, Color::hex(b"#F00"));
    img.set(Point { y: 1, x: 0 }, Color::hex(b"#F00"));
    img.set(Point { y: 2, x: 0 }, Color::hex(b"#F00"));
    img.set(Point { y: 3, x: 0 }, Color::hex(b"#F00"));
    img.set(Point { y: 0, x: 1 }, Color::hex(b"#0F0"));
    img.set(Point { y: 1, x: 1 }, Color::hex(b"#0F0"));
    img.set(Point { y: 2, x: 1 }, Color::hex(b"#0F0"));
    img.set(Point { y: 3, x: 1 }, Color::hex(b"#0F0"));
    img.set(Point { y: 0, x: 2 }, Color::hex(b"#00F"));
    img.set(Point { y: 1, x: 2 }, Color::hex(b"#00F"));
    img.set(Point { y: 2, x: 2 }, Color::hex(b"#00F"));
    img.set(Point { y: 3, x: 2 }, Color::hex(b"#00F"));
    img.save_tga("lalala.tga", true);

    // let width = 100;
    // let height = 100;


    // let mut wire = Image::new(width, height);
    // let mut img = Image::new(width, height);

    // println!("Opening model");
    // let model = Model::new("obj/african_head.obj").expect("Can't open model");

    // println!("Wireframe");
    // println!("> Rendering");
    // model.wireframe(&mut wire, Color::hex(b"#FFF"));

    // println!("> Saving");
    // wire.save_bmp("wireframe.bmp")
    //     .expect("Can't save the image");

    // wire.save_tga("wireframe.tga", false)
    //     .expect("Can't save the image");
    // wire.save_tga("wireframe_rle.tga", true)
    //     .expect("Can't save the image");

    // println!("Image");
    // println!("> Rendering");
    // model.render(
    //     &mut img,
    //     Vertex {
    //         x: 0.0,
    //         y: 0.0,
    //         z: -1.0,
    //     },
    //     Color::hex(b"#FFF"),
    // );

    // println!("> Saving");
    // img.save_bmp("image.bmp").expect("Can't save the image");

    // img.save_tga("image.tga", true)
    //     .expect("Can't save the image");

    // println!("Image created with success");
}
