mod image;
use image::Color;
use image::Image;
use image::Point;

fn main() {
    let mut image = Image::new(100, 100);
    let red = Color { r: 255, g: 0, b: 0 };
    let black = Color { r: 0, g: 0, b: 0 };

    image.clear(&black);
    image.line(&Point { x: 0, y: 0 }, &Point { x: 100, y: 100 }, &red);
    image.line(&Point { x: 13, y: 20 }, &Point { x: 80, y: 40 }, &red);

    match image.save("img.bmp") {
        Ok(()) => println!("Imagem criada com sucesso"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
