mod image;
use image::Color;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 500;
    let height = 500;
    let mut image = Image::new(width, height);
    image.clear(Color::hex(b"#000"));

    let model = Model::new("obj/african_head.obj").expect("Can't open model");

    model.wireframe_render(&mut image, Color::hex(b"#FFF"));

    image.save("img.bmp").expect("Can't save the image");
    println!("Image created with success");
}
