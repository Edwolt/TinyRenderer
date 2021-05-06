#![allow(dead_code)]

mod modules;
use modules::{Color, Vector3};

mod image;
use image::Image;

mod model;
use model::Model;

// const MODEL: &str = "diablo3_pose";
const MODEL: &str = "african_head";
// const MODEL: &str = "african_head_novn";

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;

// const LIGHT_SOURCE: Vector3 = Vector3 {
//     x: 1.0,
//     y: -1.0,
//     z: 1.0,
// }.normalize();
const LIGHT_SOURCE: Vector3 = Vector3 {
    x: 0.5773502691896258,
    y: -0.5773502691896258,
    z: 0.5773502691896258,
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
    println!("Opening model and texture\n");

    let model = {
        let image_path_string = format!("obj/{0}/{0}.obj", MODEL);
        let texture_path_string = format!("obj/{0}/{0}_diffuse.tga", MODEL);

        let image_path = image_path_string.as_str();
        let texture_path = texture_path_string.as_str();

        Model::new(image_path, Some(texture_path)).expect("Can't open model")
    };

    wrap_render("Wireframe", "wireframe.tga", |image| {
        model.render_wireframe(image, Color::hex(b"#FFF"))
    });

    wrap_render("Render Color", "color.tga", |image| {
        model.render_color(image, Color::hex(b"#FFF"), LIGHT_SOURCE)
    });

    wrap_render("Render Texture", "texture.tga", |image| {
        model.render_texture(image, LIGHT_SOURCE)
    });

    wrap_render("Perspective", "perspective.tga", |image| {
        model.render_perspective(image, CAMERA_Z, LIGHT_SOURCE);
    });

    wrap_render("Gouraud Color", "gouraud_color.tga", |image| {
        model.render_gouraud_color(image, Color::hex(b"#FFF"), LIGHT_SOURCE);
    });

    wrap_render("Gouraud", "gouraud.tga", |image| {
        model.render_gouraud(image, CAMERA_Z, LIGHT_SOURCE);
    });

    println!("Images created with success");
}
