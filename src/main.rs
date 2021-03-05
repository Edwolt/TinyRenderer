mod image;
use image::Color;
use image::Image;

fn main() {
    let mut image = Image::new(100, 100);
    let red = Color { r: 255, g: 0, b: 0 };

    image.clear(red);
    match image.save("img.bmp") {
        Ok(()) => println!("Imagem criada com sucesso"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
