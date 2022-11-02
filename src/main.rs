mod modules;
use crate::modules::{Color, Point, Vector3};

mod image;
use crate::image::Image;

mod model;
use crate::model::Model;

// const MODEL: &str = "diablo3_pose";
const MODEL: &str = "african_head";
// const MODEL: &str = "african_head_novn";

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;

const COLOR: Color = Color::hex(b"#dbc6b8");

// (1.0, -1.0, 1.0).normalize()
const LIGHT_SOURCE: Vector3 = Vector3 {
    x: 0.5773502691896258,
    y: -0.5773502691896258,
    z: 0.5773502691896258,
};

const CAMERA: Vector3 = Vector3 {
    x: 1.0,
    y: 1.0,
    z: 3.0,
};

const CENTER: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

const UP: Vector3 = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

const VIEWPORT: (Vector3, Vector3) = (
    Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    Vector3 {
        x: WIDTH as f64,
        y: HEIGHT as f64,
        z: 255.0,
    },
);

/// A function to reduce repeated code
///
/// Save the rendered image to a file and print some things
fn wrap_render<F>(title: &str, path: &str, render: F)
where
    F: FnOnce(Image) -> Image,
{
    println!("{}", title);

    println!("> Rendering");
    let image = render(Image::new(WIDTH, HEIGHT));

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

    let mut zbuffer: Vec<f64> = Vec::new();
    let index = |i: usize| Point {
        x: (i % (WIDTH as usize)) as i32,
        y: (i / (WIDTH as usize)) as i32,
    };

    wrap_render("Wireframe", "wireframe.tga", |image| {
        model.render_wireframe(image, COLOR)
    });

    wrap_render("Triangles", "triangles.tga", |image| {
        model.render_triangles(image, COLOR, LIGHT_SOURCE)
    });

    wrap_render("Render Color", "color.tga", |image| {
        let res = model.render_color(image, VIEWPORT, COLOR, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });
    wrap_render(
        "Render Color - Zbuffer",
        "color_zbuffer.tga",
        |mut image| {
            for (i, &z) in zbuffer.iter().enumerate() {
                image.set(index(i), Color::gray(z as u8))
            }
            image
        },
    );

    wrap_render("Render Color", "color.tga", |image| {
        let res = model.render_color(image, VIEWPORT, COLOR, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });

    wrap_render("Render Texture", "texture.tga", |image| {
        let res = model.render_texture(image, VIEWPORT, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });

    wrap_render("Perspective", "perspective.tga", |image| {
        let res = model.render_perspective(image, VIEWPORT, CAMERA.z, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });
    wrap_render(
        "Perspective - Zbuffer",
        "perspective_zbuffer.tga",
        |mut image| {
            for (i, &z) in zbuffer.iter().enumerate() {
                image.set(index(i), Color::gray(z as u8))
            }
            image
        },
    );

    wrap_render("Gouraud Color", "gouraud_color.tga", |image| {
        let res = model.render_gouraud_color(image, VIEWPORT, COLOR, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });

    wrap_render("Gouraud", "gouraud.tga", |image| {
        let res = model.render_gouraud(image, VIEWPORT, CAMERA.z, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });
    wrap_render("Gouraud - Zbuffer", "gouraud_zbuffer.tga", |mut image| {
        for (i, &z) in zbuffer.iter().enumerate() {
            image.set(index(i), Color::gray(z as u8))
        }
        image
    });

    wrap_render("Look at", "look.tga", |image| {
        let res = model.render_look_at(image, VIEWPORT, CAMERA, CENTER, UP, LIGHT_SOURCE);
        zbuffer = res.1;
        res.0
    });
    wrap_render("Look at - Zbuffer", "look_zbuffer.tga", |mut image| {
        for (i, &z) in zbuffer.iter().enumerate() {
            image.set(index(i), Color::gray(z as u8))
        }
        image
    });

    println!("Images created with success");
}
