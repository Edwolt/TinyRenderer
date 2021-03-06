mod image;
use image::Color;
use image::Image;
use image::Point;

fn main() {
    let mut image = Image::new(100, 100);
    let red = Color { r: 255, g: 0, b: 0 };
    let blue = Color { r: 0, g: 0, b: 255 };
    let white = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let black = Color { r: 0, g: 0, b: 0 };

    image.clear(&black);
    image.line(&Point { x: 0, y: 0 }, &Point { x: 100, y: 100 }, &blue);
    image.line(&Point { x: 13, y: 20 }, &Point { x: 80, y: 40 }, &white);
    image.line(&Point { x: 20, y: 13 }, &Point { x: 40, y: 80 }, &red);
    image.line(&Point { x: 85, y: 45 }, &Point { x: 17, y: 25 }, &red);

    match image.save("img.bmp") {
        Ok(()) => println!("Imagem criada com sucesso"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
