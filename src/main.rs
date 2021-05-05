#![allow(dead_code)]

#[macro_use]
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

    
    println!("Opening model and texture\n");
    let texture = Image::load_tga("obj/african_head_diffuse.tga").expect("Can't load texture");
    let model = Model::new("obj/african_head.obj").expect("Can't open model");
    
    {
        let mut image = Image::new(width, height);

        println!("Wireframe");
        println!("> Rendering");
        model.wireframe(&mut image, Color::hex(b"#FFF"));

        println!("> Saving");
        image
            .save_tga("wireframe.tga", true)
            .expect("Can't save the image");
    }
    println!();
    {
        let mut image = Image::new(width, height);
        println!("Render Color");
        println!("> Rendering");
        model.render_color(&mut image, Color::hex(b"#FFF"), light_source);

        println!("> Saving");
        image
            .save_tga("color.tga", true)
            .expect("Can't save the image");
    }
    println!();
    {
        let mut image = Image::new(width, height);

        println!("Render Texture");
        println!("> Rendering");
        model.render_texture(&mut image, &texture, light_source);

        println!("> Saving");
        image
            .save_tga("texture.tga", true)
            .expect("Can't save the image");
    }
    println!();
    {
        let mut image = Image::new(width, height);

        println!("Perspective");
        println!("> Rendering");
        model.render_perspective(&mut image, 3.0, &texture, light_source);

        println!("> Saving");
        image
            .save_tga("perspective.tga", true)
            .expect("Can't save the image");
    }
    println!();

    println!("Images created with success");
}
