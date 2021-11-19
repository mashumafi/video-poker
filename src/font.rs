use crate::display::Display;
use crate::pixel::Pixel;

const FONT_IMAGE_WIDTH: usize = 128;
const FONT_IMAGE_HEIGHT: usize = 64;
const FONT_IMAGE_COLS: usize = 18;
const FONT_IMAGE_ROWS: usize = 7;
pub const FONT_CHAR_WIDTH: usize = FONT_IMAGE_WIDTH / FONT_IMAGE_COLS;
pub const FONT_CHAR_HEIGHT: usize = FONT_IMAGE_HEIGHT / FONT_IMAGE_ROWS;
const BITS_IN_BYTE: usize = 8;

pub struct Font {
    pixels: [u8; FONT_IMAGE_WIDTH * FONT_IMAGE_HEIGHT],
}

impl Font {
    pub const fn default() -> Self {
        Font {
            pixels: [0; 1024 * 8],
        }
    }

    pub fn decompress_from_bytes(&mut self, bytes: &[u8]) {
        let n = bytes.len();
        let mut i = 0;
        let mut pixels_size: usize = 0;
        while i < n {
            if let Some(byte) = bytes.get(i).cloned() {
                if byte == 0x00 {
                    i += 1;
                    if let Some(next_byte) = bytes.get(i).cloned() {
                        pixels_size += next_byte as usize * 8;
                    } else {
                        break;
                    }
                    i += 1;
                } else {
                    for bit_index in 0..BITS_IN_BYTE {
                        if let Some(pixel_ref) = self.pixels.get_mut(pixels_size) {
                            *pixel_ref = ((byte >> (BITS_IN_BYTE - bit_index - 1)) & 1) * 0xFF;
                        } else {
                            break;
                        }
                        pixels_size += 1;
                    }
                    i += 1;
                }
            } else {
                break;
            }
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&u8> {
        self.pixels.get(y as usize * FONT_IMAGE_WIDTH + x as usize)
    }

    pub fn render_ascii(
        &self,
        display: &mut Display,
        code: u8,
        start_x: i32,
        start_y: i32,
        scale: i32,
        color: Pixel,
    ) {
        if (32..=126 + 4).contains(&code) {
            let char_x = (code - 32) as usize % FONT_IMAGE_COLS;
            let char_y = (code - 32) as usize / FONT_IMAGE_COLS;

            for y in 0..FONT_CHAR_HEIGHT as i32 {
                for x in 0..FONT_CHAR_WIDTH as i32 {
                    for scale_x in 0..scale {
                        for scale_y in 0..scale {
                            let font_x = char_x as i32 * FONT_CHAR_WIDTH as i32 + x;
                            let font_y = char_y as i32 * FONT_CHAR_HEIGHT as i32 + y;
                            let display_x = start_x + x * scale + scale_x;
                            let display_y = start_y + y * scale + scale_y;

                            if let Some(alpha) = self.get(font_x, font_y) {
                                if *alpha == 0xFF {
                                    display.blend(display_x, display_y, color);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self.render_ascii(display, b'?', start_x, start_y, scale, color)
        }
    }

    pub fn render_bytes(
        &self,
        display: &mut Display,
        bytes: &[u8],
        x: i32,
        y: i32,
        scale: i32,
        color: Pixel,
    ) {
        for (i, byte) in bytes.iter().enumerate() {
            self.render_ascii(
                display,
                *byte,
                x + i as i32 * FONT_CHAR_WIDTH as i32 * scale,
                y,
                scale,
                color,
            );
        }
    }
}
