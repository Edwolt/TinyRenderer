use std::io::BufWriter;
use std::io::Write;
use std::{collections::VecDeque, fs::File};

use crate::modules::{Color, Point, Vertex};

// Using i32 because Point use i32
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Color>,
}

/// Test if the point p is inside triangle v0 v1 v2
fn inside_triangle(p: Point, v0: Point, v1: Point, v2: Point) -> bool {
    let cross1 = (v0 - p).cross(v1 - v0);
    let cross2 = (v1 - p).cross(v2 - v1);
    let cross3 = (v2 - p).cross(v0 - v2);

    let neg = cross1 <= 0 && cross2 <= 0 && cross3 <= 0;
    let pos = cross1 >= 0 && cross2 >= 0 && cross3 >= 0;
    neg || pos
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        let size = (width * height) as usize;
        Image {
            width,
            height,
            pixels: vec![Color::hex(b"#000"); size],
        }
    }

    /// Set the value of pixel at (p.x, p.y) to color
    pub fn set(&mut self, Point { x, y }: Point, color: Color) {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }

    /// Get the color of pixel at (p.x, p.y)
    pub fn get(&self, Point { x, y }: Point) -> Option<Color> {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            Some(self.pixels[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    /// Set all image's pixels to color
    #[allow(dead_code)]
    pub fn clear(&mut self, color: Color) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = color;
        }
    }

    /// Draw the triangle defined by the points v0, v1, v2
    /// filled with color
    #[allow(dead_code)]
    pub fn triangle(&mut self, v0: Point, v1: Point, v2: Point, color: Color) {
        let max_x = v0.x.max(v1.x).max(v2.x);
        let max_y = v0.y.max(v1.y).max(v2.y);
        let min_x = v0.x.min(v1.x).min(v2.x);
        let min_y = v0.y.min(v1.y).min(v2.y);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Point { x, y };
                if inside_triangle(p, v0, v1, v2) {
                    self.set(p, color);
                }
            }
        }
    }

    /// Draw a triangle defined by the vertexs v0, v1, v2
    /// filled with color
    /// using a zbuffer to prevent drawing a hidden triangle over other\
    /// zbuffer length must be image.width * image.height
    /// and be filled with f64::NEG_INFINITY
    pub fn triangle_zbuffer(
        &mut self,
        zbuffer: &mut Vec<f64>,
        v0: Vertex,
        v1: Vertex,
        v2: Vertex,
        color: Color,
    ) {
        let w = self.width as usize;
        let index = |i: usize, j: usize| i * (w as usize) + j;

        // Convert Vertex to points
        let p0 = v0.to_point(self.width, self.height);
        let p1 = v1.to_point(self.width, self.height);
        let p2 = v2.to_point(self.width, self.height);

        let max_x = p0.x.max(p1.x).max(p2.x) + 1;
        let max_y = p0.y.max(p1.y).max(p2.y) + 1;
        let min_x = p0.x.min(p1.x).min(p2.x) - 1;
        let min_y = p0.y.min(p1.y).min(p2.y) - 1;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Point { x, y };
                if inside_triangle(p, p0, p1, p2) {
                    let z = {
                        let normal = (v0 - v1).cross(v0 - v2);
                        let Vertex { x: a, y: b, z: c } = normal;
                        if c == 0.0 {
                            v0.z
                        } else {
                            // let d = -(a * v0.x + b * v0.y + c * v0.z);
                            // let x = (p.x as f64) * 2.0 / (self.width as f64) - 1.0;
                            // let y = (p.y as f64) * 2.0 / (self.width as f64) - 1.0;
                            // (a * x + b * y + d) / (-c)
                            let x = (p.x as f64) * 2.0 / (self.width as f64) - 1.0;
                            let y = (p.y as f64) * 2.0 / (self.width as f64) - 1.0;
                            (a * (v0.x - x) + b * (v0.y - y) + c * v0.z) / c
                        }
                    };
                    let i = index(y as usize, x as usize);
                    if i < zbuffer.len() as usize && zbuffer[i] < z {
                        zbuffer[i] = z;
                        self.set(p, color);
                    }
                }
            }
        }
    }

    /// Draw a line from (x0, y0) to (x1, y1)
    pub fn line(
        &mut self,
        Point { x: x0, y: y0 }: Point,
        Point { x: x1, y: y1 }: Point,
        color: Color,
    ) {
        // This is my implementation of
        // Bresenham’s Line Drawing Algorithm
        // (or at least something close)
        // There's no floating point arithmetic
        // (I coded this trying to understand why the algorithm in
        // the wiki works, but the algorithm in the wiki is simpler)

        // I tried to explain why this works
        // the idea is that e is the distance to the line
        // from our pixel (x_, y_)

        // if Δx > Δy { for x {...} } else { for y {...} }
        let dx = x1 - x0; // Δx
        let dy = y1 - y0; // Δy

        // if height > width, draw walking through y axis
        let flip = dy.abs() > dx.abs();

        if dx == 0 && dy == 0 {
            // Trivial! It's a point because (x0, y0) = (x1, y1)
            self.set(Point { x: x0, y: y0 }, color);
        } else if !flip {
            // |Δx| > |Δy|

            // Define round(x), with x ∈ ℝ as the nearest integer to x

            // We want to draw the line r on the screen

            // r' is the set of pixels of screen
            // that will be draw to make the line

            // for all x' ∈ ℤ that (x', y) ∈ r
            // implies that (x', y') ∈ r' with y' = round(y)

            // a = Δy / Δx
            // a is the line coefficient, then
            // r = {(x, y) | y = a*x + b }

            // a2 = a * 2*Δx = 2*Δy
            // a3 = |a2| = 2*|Δy|
            let a3 = 2 * dy.abs();

            // Notice that dx can't be 0,
            // otherwise flip is true
            // or dy is 0 implying it's a point

            // if points P=(xp, yp) ∈ r and Q=(xq, yq) ∈ r then
            // |xq - xp| = 1 => |yq - yp| = a,
            // ie If x varies 1, then y varies a

            let mut y_ = y0; // y'

            // We will make e always satisfy y = y' + e or y = y' - e
            // e2 = e * 2*Δx

            // e3 = e2, if Δx > 0
            // e3 = -e2, otherwise
            // e3 grows when y' is more distant from y,
            // don't matter if y' > y or y' < y
            let mut e3 = 0;

            // x0 <= x1: drawing from left to right
            // Δx > 0
            for x_ in x0..=x1 {
                // Draw the pixel (x_, y_)
                self.set(Point { x: x_, y: y_ }, color);

                // x will increases 1, then y will increases a
                // Then we need to update e using following
                // line of code:
                // `e = if dy > 0 { e + a } else { e - a }`
                // Using e2 and a2 we gets this:
                // `e2 = if dy > 0 { e2 + a2 } else { e2 - a2 }`
                // Using e3 and a2 we get this:
                e3 += a3;

                // if |e| > 0.5 it means that the value of e is wrong
                // and need to be updated

                // |e| > 0.5 => |e * 2*Δx| > |0.5 * 2*Δx| => |e2| > |Δx|
                // => e3 > |dx|

                // ```
                // if e > 0.5 {
                //     e -= 1.0;
                //     y_ += 1;
                // } else if e < -0.5 {
                //     e += 1.0;
                //     y_ -= 1;
                // }
                // ```

                // ```
                // if e2 > dx {  // `>` Because Δx > 0
                //     e2 -= 2 * dx; // `e -= 1.0`;
                //     y_ += 1;
                // } else if e2 < -dx {  // `<` Because Δx < 0
                //     e2 += 2 * dx; // e += 1.0;
                //     y_ -= 1;
                // }
                // ```

                // e3 > |Δx|
                if e3 > dx {
                    e3 -= 2 * dx; // 2*|Δx|
                    y_ = if dy > 0 { y_ + 1 } else { y_ - 1 };
                }
            }

            let mut y_ = y1;
            let mut e3 = 0;

            // x0 >= x1: drawing from left to right
            // Δx < 0
            for x_ in x1..=x0 {
                self.set(Point { x: x_, y: y_ }, color);

                e3 += a3;

                // e3 > |Δx|
                if e3 > -dx {
                    e3 += 2 * dx; // 2*|Δx|
                    y_ = if dy > 0 { y_ - 1 } else { y_ + 1 };
                }
            }
        } else {
            // The same as logic that !flip, but switching x and y
            // keep self.set(&Point{ x: x_, y: y_ }, ...)

            let a3 = 2 * dx.abs();

            let mut x_ = x0;
            let mut e3 = 0;
            for y_ in y0..=y1 {
                self.set(Point { x: x_, y: y_ }, color);
                e3 += a3;
                if e3 > dy {
                    e3 -= 2 * dy;
                    x_ = if dx > 0 { x_ + 1 } else { x_ - 1 };
                }
            }

            let mut x_ = x1;
            let mut e3 = 0;
            for y_ in y1..=y0 {
                self.set(Point { x: x_, y: y_ }, color);

                e3 += a3;
                if e3 > -dy {
                    e3 += 2 * dy;
                    x_ = if dx > 0 { x_ - 1 } else { x_ + 1 };
                }
            }
        }
    }

    /// Save the image as a bitmap
    pub fn save_bmp(&self, path: &str) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(path)?);

        let header_size: u32 = 14;
        let dib_size: u32 = 40;
        let image_size: u32 = (3 * self.width * self.height) as u32;

        // * Header
        // Indentify the file
        file.write(b"BM")?; // 2 ASCII chars

        // File size
        let size: u32 = header_size + dib_size + image_size;
        file.write_all(&size.to_le_bytes())?; // 4 bytes

        // Unused two fields of 2 bytes (can be 0)
        file.write_all(&0u16.to_le_bytes())?; // 2 bytes
        file.write_all(&0u16.to_le_bytes())?; // 2 bytes

        // Offset where the image can be found
        let offset: u32 = header_size + dib_size;
        file.write_all(&offset.to_le_bytes())?; // 4 bytes

        // * DIB Header (Image Header)
        // DIB file size
        file.write_all(&dib_size.to_le_bytes())?; // 4 bytes

        // Width and height
        file.write_all(&self.width.to_le_bytes())?; // 4 bytes (signed)
        file.write_all(&self.height.to_le_bytes())?; // 4 bytes (signed)

        // Number of color planes (Must be 1)
        file.write_all(&1u16.to_le_bytes())?; // 2 bytes

        // Color depth
        file.write_all(&24u16.to_le_bytes())?; // 2 bytes

        // Compression method (0 means none)
        file.write_all(&0u32.to_le_bytes())?; // 4 bytes

        // Image size (for none compression can be a dummy 0)
        file.write_all(&image_size.to_le_bytes())?; // 4 bytes

        // Horizontal and vertical pixel per meter
        // (0 means no preference)
        file.write_all(&0i32.to_le_bytes())?; // 4 bytes (signed)
        file.write_all(&0i32.to_le_bytes())?; // 4 bytes (signed)

        // Number of color used
        // 0 means there is no palette
        // otherwise is the number of color in the color table
        file.write_all(&0u32.to_le_bytes())?; // 4 bytes

        // Number of important color of the palette
        // Used to draw the image on limited displays
        file.write_all(&0u32.to_le_bytes())?; // 4 bytes

        // * Image
        for &color in &self.pixels {
            file.write_all(&color.to_bgr_bytes())?;
        }

        file.flush()?;
        Ok(())
    }

    /// Save the image as a Truevision TGA file
    pub fn save_tga(&self, path: &str, mut rle: bool) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(path)?);

        // * Header
        // ID length
        file.write_all(&0u8.to_le_bytes())?; // 1 byte

        // Color map type (0 means no color map)
        file.write_all(&0u8.to_le_bytes())?; // 1 byte

        // Image type (Comprenssion and color types)
        let image_type = if rle { 10u8 } else { 2u8 };
        file.write_all(&(image_type).to_le_bytes())?; // 1 byte

        // ** Color map specification
        // Ignored because color map type is 0
        // First entry index
        file.write_all(&0u16.to_le_bytes())?; // 2 bytes

        // Color map length
        file.write_all(&0u16.to_le_bytes())?; // 2 bytes

        // Color map entry size
        file.write_all(&0u8.to_le_bytes())?; // 2 bytes

        // ** Image specification
        // X and Y origin
        file.write_all(&0u16.to_le_bytes())?; // 2 bytes
        file.write_all(&0u16.to_le_bytes())?; // 2 bytes

        // Image width and height
        file.write_all(&(self.width as u16).to_le_bytes())?; // 2 bytes
        file.write_all(&(self.height as u16).to_le_bytes())?; // 2 bytes

        // Pixel depth
        file.write_all(&24u8.to_le_bytes())?; // 1 bytes

        // Image descriptor (0 works fine)
        file.write_all(&0u8.to_le_bytes())?; // 1 byte

        // * Image and color map data
        // Image ID (we set to 0)
        // Color Map (we set to no color map)
        // Image Data

        // My rle algorithm needs at least 2 pixels
        rle = if self.pixels.len() <= 2 { false } else { rle };

        if rle {
            const MAX_COUNT: u8 = 0b01111111;
            const RLE: u8 = 0b10000000; // Mark counter as rle

            let mut raw_packet: VecDeque<Color> = VecDeque::new();
            let mut it = self.pixels.iter();
            let mut counter: usize = 0;

            let mut last: Color = *it.next().unwrap();
            let mut cur: Color = *it.next().unwrap();
            let mut line: i32 = 2;

            'outer_loop: loop {
                loop {
                    // line loop

                    'rle_loop: while last == cur {
                        counter += 1; // Add cur to rle_packet

                        // last = cur; cur = next;
                        cur = match &it.next() {
                            Some(&next) => next,
                            None => {
                                println!("EOF RLE: {} -> {:?}", counter + 1, last);

                                // Write rle_packet
                                while counter > (MAX_COUNT as usize) {
                                    file.write_all(&(MAX_COUNT | RLE).to_le_bytes())?;
                                    file.write_all(&last.to_rgb_bytes())?;
                                    counter -= (MAX_COUNT as usize) + 1;
                                }
                                file.write_all(&(counter as u8 | RLE).to_le_bytes())?;
                                file.write_all(&last.to_rgb_bytes())?;
                                // counter = 0;

                                break 'outer_loop;
                            }
                        };
                        line += 1;

                        if line == self.width {
                            if last == cur {
                                println!("LIN RLE ADD");
                                counter += 1;
                            } else {
                                // Write rle_packet
                                println!("LIN RLE: {} -> {:?}", counter + 1, last);

                                while counter > (MAX_COUNT as usize) {
                                    file.write_all(&(MAX_COUNT | RLE).to_le_bytes())?;
                                    file.write_all(&last.to_rgb_bytes())?;
                                    counter -= (MAX_COUNT as usize) + 1;
                                }
                                file.write_all(&(counter as u8 | RLE).to_le_bytes())?;
                                file.write_all(&last.to_rgb_bytes())?;
                                counter = 0;

                                println!("LAS RAW: {} -> {:?}", counter + 1, cur);
                                // Write cur
                                file.write_all(&0u8.to_le_bytes())?;
                                file.write_all(&cur.to_rgb_bytes())?;
                            }

                            line = 0;
                            break 'rle_loop;
                        }
                    }
                    if counter != 0 {
                        println!("DIF RLE: {} -> {:?}", counter + 1, last);
                        // Write rle_packet
                        while counter > (MAX_COUNT as usize) {
                            file.write_all(&(MAX_COUNT | RLE).to_le_bytes())?;
                            file.write_all(&last.to_rgb_bytes())?;
                            counter -= (MAX_COUNT as usize) + 1;
                        }
                        file.write_all(&(counter as u8 | RLE).to_le_bytes())?;
                        file.write_all(&last.to_rgb_bytes())?;
                        counter = 0;
                    }

                    'raw_loop: while last != cur {
                        raw_packet.push_back(last);

                        // last = cur; cur = next;
                        last = cur;
                        cur = match &it.next() {
                            Some(&next) => next,
                            None => {
                                raw_packet.push_back(cur);

                                // Write raw_packet
                                let mut len = (raw_packet.len() as usize) - 1;
                                println!("EOF RAW: {} -> Vec ...", len + 1);
                                while len > (MAX_COUNT as usize) {
                                    file.write_all(&MAX_COUNT.to_le_bytes())?;
                                    for _i in 0..MAX_COUNT + 1 {
                                        let color = raw_packet.pop_front().unwrap();
                                        file.write_all(&color.to_rgb_bytes())?;
                                    }
                                    len -= (MAX_COUNT as usize) + 1;
                                }
                                len = (raw_packet.len() as usize) - 1;
                                file.write_all(&len.to_le_bytes())?;
                                for &color in &raw_packet {
                                    file.write(&color.to_rgb_bytes())?;
                                }
                                // raw_packet.clear();

                                break 'outer_loop;
                            }
                        };
                        line += 1;

                        if line == self.width {
                            if last != cur {
                                raw_packet.push_back(last);
                                raw_packet.push_back(cur);

                                // last = next; cur = next_of_next;
                                last = match &it.next() {
                                    Some(&next) => next,
                                    None => {
                                        println!("EOF NIL");
                                        break 'outer_loop;
                                    }
                                };
                                cur = match &it.next() {
                                    Some(&next) => next,
                                    None => {
                                        println!("EDF RAW: 1 -> {:?}", last);

                                        file.write_all(&0u8.to_le_bytes())?;
                                        file.write_all(&last.to_rgb_bytes())?;
                                        break 'outer_loop;
                                    }
                                };
                            } else {
                                // Write raw_packet
                                let mut len = (raw_packet.len() as usize) - 1;
                                println!("LIN RAW: {} -> Vec ...", len + 1);
                                while len > (MAX_COUNT as usize) {
                                    file.write_all(&MAX_COUNT.to_le_bytes())?;
                                    for _i in 0..MAX_COUNT + 1 {
                                        let color = raw_packet.pop_front().unwrap();
                                        file.write_all(&color.to_rgb_bytes())?;
                                    }
                                    len -= (MAX_COUNT as usize) + 1;
                                }
                                len = (raw_packet.len() as usize) - 1;
                                file.write_all(&len.to_le_bytes())?;
                                for &color in &raw_packet {
                                    file.write(&color.to_rgb_bytes())?;
                                }
                                raw_packet.clear();

                                // Write last and cur as rle_packet
                                println!("LAS RLE: 2 -> {:?}", last);
                                file.write_all(&(1u8 | RLE).to_le_bytes())?;
                                file.write(&last.to_rgb_bytes())?;
                            }

                            line = 0;
                            break 'raw_loop;
                        }
                    }
                    if !raw_packet.is_empty() {
                        let mut len = (raw_packet.len() as usize) - 1;
                        println!("EQ  RAW: {} -> Vec {:?}", len + 1, raw_packet);
                        println!("MAO: {:?} {:?}", last, cur);
                        while len > (MAX_COUNT as usize) {
                            file.write_all(&MAX_COUNT.to_le_bytes())?;
                            for _i in 0..MAX_COUNT + 1 {
                                let color = raw_packet.pop_front().unwrap();
                                file.write_all(&color.to_rgb_bytes())?;
                            }
                            len -= (MAX_COUNT as usize) + 1;
                        }
                        len = (raw_packet.len() as usize) - 1;
                        file.write_all(&len.to_le_bytes())?;
                        for &color in &raw_packet {
                            file.write(&color.to_rgb_bytes())?;
                        }
                        raw_packet.clear();
                    }
                }
            }
        } else {
            for &color in &self.pixels {
                let Color { r, g, b } = color;
                file.write_all(&[r, g, b])?;
            }
        }

        // * Footer
        // Extension area offset (0 because we won't use extensio area)
        file.write_all(&0u32.to_le_bytes())?; // 4 bytes

        // Developer area offset (0 because we won't use extensio area)
        file.write_all(&0u32.to_le_bytes())?; // 4 bytes

        // Indentify the file
        file.write_all(b"TRUEVISION-XFILE.\0")?;

        file.flush()?;
        Ok(())
    }
}
