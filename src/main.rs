mod image;
use image::Color;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 500;
    let height = 500;

    let mut wire = Image::new(width, height);
    let mut image = Image::new(width, height);

    let model = Model::new("obj/african_head.obj").expect("Can't open model");
    model.wireframe_render(&mut wire, Color::hex(b"#FFF"));
    model.render(&mut image);

    image.save("image.bmp").expect("Can't save the image");
    wire.save("wire.bmp").expect("Can't save the image");
    println!("Image created with success");
}
