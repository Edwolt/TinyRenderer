mod image;
use image::Color;
use image::Image;
use image::Point;

mod model;
use model::Model;

fn main() {
    let width = 200;
    let height = 200;
    let mut image = Image::new(width, height);
    image.clear(Color::hex(b"#000"));

    // let model = Model::new("obj/african_head.obj").expect("Can't open model");
    // model.wireframe_render(&mut image, Color::hex(b"#FFF"));

    image.triangle(
        Point { x: 10, y: 70 },
        Point { x: 50, y: 160 },
        Point { x: 70, y: 80 },
        Color::hex(b"#F00"),
    );
    image.triangle(
        Point { x: 180, y: 50 },
        Point { x: 150, y: 1 },
        Point { x: 70, y: 180 },
        Color::hex(b"#FFF"),
    );
    image.triangle(
        Point { x: 180, y: 150 },
        Point { x: 120, y: 160 },
        Point { x: 130, y: 180 },
        Color::hex(b"#0F0"),
    );

    image.save("img.bmp").expect("Can't save the image");
    println!("Image created with success");
}
