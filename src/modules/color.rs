#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Receive a hexadecimal color value and return a Color
    /// The value must be follow one of this formats: #HHH or #HHHHHH
    /// If it is a invalid value, return black
    pub const fn hex(value: &[u8]) -> Self {
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
                Self {
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
                Self {
                    r: (r1 << 4) + r0,
                    g: (g1 << 4) + g0,
                    b: (b1 << 4) + b0,
                }
            }
            _ => Self { r: 0, g: 0, b: 0 },
        }
    }

    pub const fn gray(value: u8) -> Self {
        Self {
            r: value,
            g: value,
            b: value,
        }
    }

    /// Return a color with the intensity changed
    pub fn light(self, intensity: f64) -> Self {
        if intensity > 0.0 {
            Self {
                r: ((self.r as f64) * intensity) as u8,
                g: ((self.g as f64) * intensity) as u8,
                b: ((self.b as f64) * intensity) as u8,
            }
        } else {
            Self::hex(b"#000")
        }
    }

    /// Convert color to bytes
    pub fn to_bytes(self) -> [u8; 3] {
        [self.b, self.g, self.r]
    }

    /// Convert bytes to color
    pub fn from_bytes(buffer: [u8; 3]) -> Self {
        let [b, g, r] = buffer;
        Self { r, g, b }
    }
}
