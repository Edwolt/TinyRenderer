mod image;
use image::Color;
use image::Image;
use image::Point;

mod model;
use model::Model;
use model::Vertex;

const BLACK: Color = Color {
    r: 000,
    g: 000,
    b: 000,
};
// const GREEN: Color = Color {
//     r: 000,
//     g: 255,
//     b: 000,
// };
// const RED: Color = Color {
//     r: 255,
//     g: 000,
//     b: 000,
// };
// const YELLOW: Color = Color {
//     r: 255,
//     g: 255,
//     b: 000,
// };
// const BLUE: Color = Color {
//     r: 000,
//     g: 000,
//     b: 255,
// };
// const CYAN: Color = Color {
//     r: 000,
//     g: 255,
//     b: 255,
// };
const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

fn vertex_to_point(vertex: Vertex, width: i32, height: i32) -> Point {
    let x = (vertex.x + 1f64) * (width as f64) / 2f64;
    let y = (vertex.y + 1f64) * (height as f64) / 2f64;
    Point {
        x: x as i32,
        y: y as i32,
    }
}

fn main() {
    let width = 1000;
    let height = 1000;
    let mut image = Image::new(width, height);

    let model = Model::new("obj/african_head.obj").expect("Can't open model");

    image.clear(BLACK);
    for face in &model.faces {
        let mut v = match face.last() {
            Some(v) => v,
            None => continue,
        };
        for u in face {
            image.line(
                vertex_to_point(*v, width, height),
                vertex_to_point(*u, width, height),
                WHITE
            );
            v = u;
        }
    }

    // image.clear(BLACK);
    // image.line(Point{x:0, y:0}, Point{x:99, y:99}, WHITE);
    // image.line(Point{x:0, y:0}, Point{x:0, y:99}, GREEN);
    // image.line(Point{x:0, y:0}, Point{x:99, y:0}, GREEN);
    // image.line(Point{x:100, y:100}, Point{x:100, y:0}, GREEN);
    // image.line(Point{x:100, y:100}, Point{x:100, y:0}, GREEN);

    // image.line(Point{x:13, y:20}, Point{x:80, y:40}, BLUE);
    // image.line(Point{x:20, y:30}, Point{x:80, y:20}, BLUE);
    // image.line(Point{x:20, y:13}, Point{x:40, y:80}, RED);

    // image.line(Point{x:80, y:45}, Point{x:13, y:25}, CYAN);
    // image.line(Point{x:45, y:80}, Point{x:25, y:13}, YELLOW);

    image.save("img.bmp").expect("Can't save the image");
    println!("Image created with success");
}
