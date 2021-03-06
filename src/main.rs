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
    image.line(&Point(0, 0), &Point(100, 100), &blue);
    image.line(&Point(13, 20), &Point(80, 40), &white);
    image.line(&Point(20, 13), &Point(40, 80), &red);
    image.line(&Point(85, 45), &Point(17, 25), &red);

    match image.save("img.bmp") {
        Ok(()) => println!("Imagem criada com sucesso"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
