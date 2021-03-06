use std::fs::File;
use std::io::Write;

pub struct Point(pub i32, pub i32);

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        let size = (width * height) as usize;
        Image {
            width,
            height,
            pixels: vec![Color { r: 0, g: 0, b: 0 }; size],
        }
    }

    /// Set the value of pixel at (p.x, p.y) to color
    pub fn set(&mut self, &Point(x, y): &Point, color: &Color) {
        self.pixels[(y * self.width + x) as usize] = color.clone();
    }

    /// Set all image's pixels to color
    pub fn clear(&mut self, color: &Color) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = color.clone();
        }
    }

    /// Draw a line from (x0, y0) to (x1, y1)
    pub fn line(&mut self, &Point(x0, y0): &Point, &Point(x1, y1): &Point, color: &Color) {
        // Using Thales Theorem
        // y = (x - x0) * ((y1 - y0) / x1 - x0) + y0
        let q = ((y1 - y0) as f32) / ((x1 - x0) as f32);
        let c = (-x0 as f32) * q + y0 as f32;
        for x in x0..x1 {
            let y = (x as f32) * q + c;
            self.set(&Point(x, y as i32), &color);
        }
        for x in x1..x0 {
            let y = (x as f32) * q + c;
            self.set(&Point(x, y as i32), &color);
        }
    }

    /// Save the image as a bitmap
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        let header_size: u32 = 14;
        let dib_size: u32 = 40;
        let image_size: u32 = (3 * self.width * self.height) as u32;

        // * Header
        // Indentify the file
        file.write_all(b"BM")?; // u8, u8

        // File size
        let size: u32 = header_size + dib_size + image_size;
        file.write_all(&size.to_le_bytes())?; // u32

        // Unused two fields of 2 bytes
        file.write_all(&0u16.to_le_bytes())?; // 0u16
        file.write_all(&0u16.to_le_bytes())?; // 0u16

        // Offset where the image can be found
        let offset: u32 = header_size + dib_size;
        file.write_all(&offset.to_le_bytes())?; // u32

        // * DIB Header (Image Header)
        // DIB file size
        file.write_all(&dib_size.to_le_bytes())?; // u32

        // Width and height
        file.write_all(&self.width.to_le_bytes())?; // i32
        file.write_all(&self.height.to_le_bytes())?; // i32

        // Number of color planes (Must be 1)
        file.write_all(&1u16.to_le_bytes())?; // 1u16

        // Color depth
        file.write_all(&24u16.to_le_bytes())?; // u16

        // Compression method (0 means None)
        file.write_all(&0u32.to_le_bytes())?; // u32

        // Image size (for none compression can be a dummy 0)
        file.write_all(&image_size.to_le_bytes())?; // u32

        // Horizontal and vertical pixel per meter
        // (0 means no preference)
        file.write_all(&0i32.to_le_bytes())?; // i32
        file.write_all(&0i32.to_le_bytes())?; // i32

        // Number of color used
        // 0 means there is no palette
        // otherwise is the number of color in the color table
        file.write_all(&0u32.to_le_bytes())?; // u32

        // Number of important color of the palette
        // Used to draw the image on limited displays
        file.write_all(&0u32.to_le_bytes())?; // u32

        // * Image
        for color in &self.pixels {
            let Color { r, g, b } = color;
            file.write_all(&b.to_le_bytes())?;
            file.write_all(&g.to_le_bytes())?;
            file.write_all(&r.to_le_bytes())?;
        }

        Ok(())
    }
}
