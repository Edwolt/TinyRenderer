#![allow(dead_code)]

#[macro_use]
mod modules;
use modules::{Color, Vertex3};

mod image;
use image::Image;

mod model;
use model::Model;


// Const 
const MODEL: &str = "african_head";

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;

const LIGHT_SOURCE: Vertex3 = Vertex3 {
    x: 0.3,
    y: 0.0,
    z: -1.0,
};

const CAMERA_Z: f64 = 3.0;

/// A function to reduce repeated code
///
/// Save the rendered image to a file and print some things
fn wrap_render<F>(title: &str, path: &str, render: F)
where
    F: FnOnce(&mut Image),
{
    let mut image = Image::new(WIDTH, HEIGHT);

    println!("{}", title);

    println!("> Rendering");
    render(&mut image);

    println!("> Saving");
    image.save_tga(path, true).expect("Can't save the image");

    println!();
}

fn main() {
    // const MODEL: &str = "diablo3_pose";

    println!("Opening model and texture\n");

    let texture = {
        let path = format!("obj/{0}/{0}_diffuse.tga", MODEL);
        Image::load_tga(path.as_str()).expect("Can't load texture")
    };
    let model = {
        let path = format!("obj/{0}/{0}.obj", MODEL);
        Model::new(path.as_str()).expect("Can't open model")
    };

    wrap_render("Wireframe", "wireframe.tga", |image| {
        model.wireframe(image, Color::hex(b"#FFF"))
    });

    wrap_render("Render Color", "color.tga", |image| {
        model.render_color(image, Color::hex(b"#FFF"), LIGHT_SOURCE)
    });

    wrap_render("Render Texture", "texture.tga", |image| {
        model.render_texture(image, &texture, LIGHT_SOURCE)
    });

    wrap_render("Perspective", "perspective.tga", |image| {
        model.render_perspective(image, CAMERA_Z, &texture, LIGHT_SOURCE);
    });

    println!("Images created with success");
}
