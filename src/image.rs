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
        // I tried to explain why this works
        // the idea is that e is the distance to the line
        // from our pixel (x_, y_)

        // if Δx > Δy { for x {...} } else { for y {...} }
        let dx = x1 - x0; // Δx
        let dy = y1 - y0; // Δy
        let flip = dy.abs() > dx.abs();

        if dx == 0 && dy == 0 {
            // Trivial! It's a point because (x0, y0) = (x1, y1)
            self.set(&Point(x0, y0), color);
        } else if !flip {
            // |Δx| > |Δy|

            // Define round(x), with x ∈ ℝ as the nearest integer to x

            // We want to draw the line r on the screen

            // r' is the set of pixels of screen
            // that will be draw to make the line

            // for all x' ∈ ℤ that (x', y) ∈ r
            // implies that (x', y') ∈ r' with y' = round(y)

            // a is the line coefficient, then
            // r = {(x, y) | y = a*x + b }
            let a = (dy as f32) / (dx as f32);
            // Notice that dx can't be 0,
            // otherwise flip is true
            // or dy is 0 creating a point

            // if points P=(xp, yp) ∈ r and Q=(xq, yq) ∈ r then
            // |xq - xp| = 1 => |yq - yp| = a,
            // ie If x varies 1, then y varies a

            let mut y_ = y0; // y'

            // We will make y = y' + e
            let mut e = 0.0;

            // x0 <= x1: drawing from left to right
            for x_ in x0..=x1 {
                // Draw the pixel (x_, y_)
                self.set(&Point(x_, y_), &color);

                // x will increases 1, then y will increases a
                e = if dy > 0 { e + a } else { e - a };

                // if |e| > 0.5 it means that the value of e is wrong
                // and need to be updated

                if e > 0.5 {
                    e -= 1.0;
                    y_ += 1;
                } else if e < -0.5 {
                    e += 1.0;
                    y_ -= 1;
                }
            }

            let mut y_ = y1;
            let mut e = 0.0;

            // x0 >= x1: drawing from left to right
            for x_ in x1..=x0 {
                self.set(&Point(x_, y_), &color);

                e = if dy < 0 { e + a } else { e - a };

                if e > 0.5 {
                    e -= 1.0;
                    y_ += 1;
                } else if e < -0.5 {
                    e += 1.0;
                    y_ -= 1;
                }
            }
        } else {
            // The same as !flip, but switching x and y
            // Making it work like the image was traposed
            // keep self.set(&Point(x_, y_), ...)

            let a = (dx as f32) / (dy as f32);
            let mut x_ = x0;
            let mut e = 0.0;
            for y_ in y0..=y1 {
                self.set(&Point(x_, y_), &color);

                e = if dx > 0 { e + a } else { e - a };

                if e > 0.5 {
                    e -= 1.0;
                    x_ += 1;
                } else if e < -0.5 {
                    e += 1.0;
                    x_ -= 1;
                }
            }

            let mut x_ = x1;
            let mut e = 0.0;

            for y_ in y1..=y0 {
                self.set(&Point(x_, y_), &color);

                e = if dx < 0 { e + a } else { e - a };

                if e > 0.5 {
                    e -= 1.0;
                    x_ += 1;
                } else if e < -0.5 {
                    e += 1.0;
                    x_ -= 1;
                }
            }
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
