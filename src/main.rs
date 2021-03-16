mod image;
use image::Color;
use image::Image;

mod model;
use model::Model;

fn main() {
    let width = 1000;
    let height = 1000;
    let mut image = Image::new(width, height);
    image.clear(Color { r: 0, g: 0, b: 0 });

    let model = Model::new("obj/african_head.obj").expect("Can't open model");

    model.wireframe_render(
        &mut image,
        Color {
            r: 255,
            g: 255,
            b: 255,
        },
    );

    image.save("img.bmp").expect("Can't save the image");
    println!("Image created with success");
}
