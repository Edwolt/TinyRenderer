use std::fs::File;
use std::io::Write;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl ops::Mul for Point {
    type Output = i32;
    fn mul(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

impl Point {
    const fn cross(&self, other: Point) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Receive a hexadecimal color value and return a Color
    /// The value must be follow one of this formats: #HHH or #HHHHHH
    /// If it is a invalid value, return black
    pub const fn hex(value: &[u8]) -> Color {
        /// Convert a u8 ascii char to the hexadecimal equivalent
        /// If it isn't a valid u8 ascii char return 0
        const fn char_to_hex(val: u8) -> u8 {
            if b'0' <= val && val <= b'9' {
                val - b'0'
            } else if b'a' <= val && val <= b'f' {
                val - b'a' + 10
            } else if b'A' <= val && val <= b'F' {
                val - b'A' + 10
            } else {
                0
            }
        }

        match value {
            &[b'#', r, g, b] => {
                let r = char_to_hex(r);
                let g = char_to_hex(g);
                let b = char_to_hex(b);
                Color {
                    r: (r << 4) + r,
                    g: (g << 4) + g,
                    b: (b << 4) + b,
                }
            }
            &[b'#', r1, r0, g1, g0, b1, b0] => {
                let r1 = char_to_hex(r1);
                let r0 = char_to_hex(r0);
                let g1 = char_to_hex(g1);
                let g0 = char_to_hex(g0);
                let b1 = char_to_hex(b1);
                let b0 = char_to_hex(b0);
                Color {
                    r: (r1 << 4) + r0,
                    g: (g1 << 4) + g0,
                    b: (b1 << 4) + b0,
                }
            }
            _ => Color { r: 0, g: 0, b: 0 },
        }
    }
}

#[test]
fn test_color() {
    let c = Color::hex(b"#097d71");
    if c.r != 9 || c.g != 125 || c.b != 113 {
        panic!("{} = {:?}", "#097d71", c);
    }
    let c = Color::hex(b"#F97D71");
    if c.r != 249 || c.g != 125 || c.b != 113 {
        panic!("{} = {:?}", "#F97d71", c);
    }
    let c = Color::hex(b"#F5a");
    if c.r != 255 || c.g != 85 || c.b != 170 {
        panic!("{} = {:?}", "#F5a", c);
    }
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
            pixels: vec![Color::hex(b"#000"); size],
        }
    }

    /// Set the value of pixel at (p.x, p.y) to color
    pub fn set(&mut self, Point { x, y }: Point, color: Color) {
        if 0 <= x && x < self.width && 0 <= y && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }

    /// Set all image's pixels to color
    #[allow(dead_code)]
    pub fn clear(&mut self, color: Color) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = color;
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

    pub fn triangle(&mut self, v0: Point, v1: Point, v2: Point, color: Color) {
        const fn max(a: i32, b: i32) -> i32 {
            if a > b {
                a
            } else {
                b
            }
        }
        const fn min(a: i32, b: i32) -> i32 {
            if a < b {
                a
            } else {
                b
            }
        }

        fn inside(p: Point, v0: Point, v1: Point, v2: Point) -> bool {
            let cross1 = (v0 - p).cross(v1 - v0);
            let cross2 = (v1 - p).cross(v2 - v1);
            let cross3 = (v2 - p).cross(v0 - v2);

            let neg = cross1 < 0 && cross2 < 0 && cross3 < 0;
            let pos = cross1 > 0 && cross2 > 0 && cross3 > 0;
            neg || pos
        }

        let max_x = max(max(v0.x, v1.x), v2.x);
        let max_y = max(max(v0.y, v1.y), v2.y);
        let min_x = min(min(v0.x, v1.x), v2.x);
        let min_y = min(min(v0.y, v1.y), v2.y);

        for x in min_x..max_x {
            for y in min_y..max_y {
                let p = Point { x, y };
                if inside(p, v0, v1, v2) {
                    self.set(p, color);
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
