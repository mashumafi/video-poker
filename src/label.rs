use crate::display::Display;
use crate::font::Font;
use crate::pixel::Pixel;

const LABEL_CAPACITY: usize = 64;
const SHADOW_COLOR: Pixel = Pixel::rgba(0x2B, 0x2B, 0x2B, 100);

pub struct Label {
    chars: [u8; LABEL_CAPACITY],
    count: usize,
}

impl Label {
    pub const fn empty() -> Self {
        Self {
            chars: [0; LABEL_CAPACITY],
            count: 0,
        }
    }

    pub fn render(
        &self,
        display: &mut Display,
        font: &Font,
        x: i32,
        y: i32,
        scale: i32,
        color: Pixel,
    ) {
        if let Some(bytes) = self.chars.get(0..self.count) {
            font.render_bytes(display, bytes, x - scale, y - scale, scale, SHADOW_COLOR);
            font.render_bytes(display, bytes, x, y, scale, color);
        }
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn push_byte(&mut self, b: u8) {
        if let Some(char_ref) = self.chars.get_mut(self.count) {
            *char_ref = b;
            self.count += 1;
        }
    }

    pub fn push_bytes(&mut self, bs: &[u8]) {
        for b in bs {
            self.push_byte(*b);
        }
    }

    pub fn push_usize(&mut self, mut x: usize) {
        let saved_count = self.count;

        if x == 0 {
            self.push_byte(b'0');
        } else {
            while x > 0 && self.count < LABEL_CAPACITY {
                self.push_byte((x % 10) as u8 + b'0');
                x /= 10;
            }

            if x > 0 {
                // x does not fit into the Label rolling back and quitting
                self.count = saved_count;
                return;
            }
        }

        if let Some(chars) = self.chars.get_mut(saved_count..self.count) {
            chars.reverse();
        }
    }

    fn push_decimals(&mut self, mut value: usize) {
        let mut width = 9;

        // remove trailing zeros
        while value % 10 == 0 && width > 0 {
            value /= 10;
            width -= 1;
        }

        self.push_byte(b'.');
        self.push_usize(value);
    }

    pub fn push_f64(&mut self, mut value: f64) {
        if value < 0.0 {
            value = -value;
            self.push_byte(b'-');
        }

        if value.is_infinite() {
            self.push_bytes(b"inf");
            return;
        }

        let mut integral_part = 0_u32;
        let mut decimal_part = 0_u32;
        let mut exponent = 0_i16;

        split_float(value, &mut integral_part, &mut decimal_part, &mut exponent);

        self.push_usize(integral_part as usize);
        if decimal_part != 0 {
            self.push_decimals(decimal_part as usize);
        }

        if exponent < 0 {
            self.push_bytes(b"e-");
            self.push_usize(-exponent as usize);
        }

        if exponent > 0 {
            self.push_byte(b'e');
            self.push_usize(exponent as usize);
        }
    }
}

fn split_float(
    mut value: f64,
    integral_part: &mut u32,
    decimal_part: &mut u32,
    exponent: &mut i16,
) {
    *exponent = normalize_float(&mut value);

    *integral_part = value as u32;
    let mut remainder = value - *integral_part as f64;

    remainder *= 1e9;
    *decimal_part = remainder as u32;

    // rounding
    remainder -= *decimal_part as f64;
    if remainder >= 0.5 {
        *decimal_part += 1;
        if *decimal_part >= 1000000000 {
            *decimal_part = 0;
            *integral_part += 1;
            if *exponent != 0 && *integral_part >= 10 {
                *exponent += 1;
                *integral_part = 1;
            }
        }
    }
}

fn normalize_float(value: &mut f64) -> i16 {
    const POSITIVE_EXP_THRESHOLD: f64 = 1e7;
    const NEGATIVE_EXP_THRESHOLD: f64 = 1e-5;
    let mut exponent = 0;

    if *value >= POSITIVE_EXP_THRESHOLD {
        if *value >= 1e256 {
            *value /= 1e256;
            exponent += 256;
        }
        if *value >= 1e128 {
            *value /= 1e128;
            exponent += 128;
        }
        if *value >= 1e64 {
            *value /= 1e64;
            exponent += 64;
        }
        if *value >= 1e32 {
            *value /= 1e32;
            exponent += 32;
        }
        if *value >= 1e16 {
            *value /= 1e16;
            exponent += 16;
        }
        if *value >= 1e8 {
            *value /= 1e8;
            exponent += 8;
        }
        if *value >= 1e4 {
            *value /= 1e4;
            exponent += 4;
        }
        if *value >= 1e2 {
            *value /= 1e2;
            exponent += 2;
        }
        if *value >= 1e1 {
            *value /= 1e1;
            exponent += 1;
        }
    }

    if *value > 0.0 && *value <= NEGATIVE_EXP_THRESHOLD {
        if *value < 1e-255 {
            *value *= 1e256;
            exponent -= 256;
        }
        if *value < 1e-127 {
            *value *= 1e128;
            exponent -= 128;
        }
        if *value < 1e-63 {
            *value *= 1e64;
            exponent -= 64;
        }
        if *value < 1e-31 {
            *value *= 1e32;
            exponent -= 32;
        }
        if *value < 1e-15 {
            *value *= 1e16;
            exponent -= 16;
        }
        if *value < 1e-7 {
            *value *= 1e8;
            exponent -= 8;
        }
        if *value < 1e-3 {
            *value *= 1e4;
            exponent -= 4;
        }
        if *value < 1e-1 {
            *value *= 1e2;
            exponent -= 2;
        }
        if *value < 1e0 {
            *value *= 1e1;
            exponent -= 1;
        }
    }

    exponent
}
