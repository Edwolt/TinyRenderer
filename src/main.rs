mod image;
use image::Color;
use image::Image;
use image::Point;

const BLACK: Color = Color {
    r: 000,
    g: 000,
    b: 000,
};
const GREEN: Color = Color {
    r: 000,
    g: 255,
    b: 000,
};
const RED: Color = Color {
    r: 255,
    g: 000,
    b: 000,
};
const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 000,
};
const BLUE: Color = Color {
    r: 000,
    g: 000,
    b: 255,
};
const CYAN: Color = Color {
    r: 000,
    g: 255,
    b: 255,
};
const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

fn main() {
    let mut image = Image::new(100, 100);

    image.clear(&BLACK);
    image.line(&Point(0, 0), &Point(99, 99), &WHITE);
    image.line(&Point(0, 0), &Point(0, 99), &GREEN);
    image.line(&Point(0, 0), &Point(99, 0), &GREEN);
    // image.line(&Point(100, 100), &Point(100, 0), &GREEN);
    // image.line(&Point(100, 100), &Point(100, 0), &GREEN);

    image.line(&Point(13, 20), &Point(80, 40), &BLUE);
    image.line(&Point(20, 13), &Point(40, 80), &RED);

    image.line(&Point(80, 45), &Point(13, 25), &CYAN);
    image.line(&Point(45, 80), &Point(25, 13), &YELLOW);

    match image.save("img.bmp") {
        Ok(()) => println!("Imagem criada com sucesso"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
