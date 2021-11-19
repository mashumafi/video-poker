use core::fmt;

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Pixel(u32);

fn blend_a(alpha_a: f32, alpha_b: f32) -> f32 {
    alpha_a + alpha_b * (1.0 - alpha_a)
}

fn blend_c(color_a: u32, alpha_a: f32, color_b: u32, alpha_b: f32, alpha_o: f32) -> u8 {
    ((color_a as f32 * alpha_a + color_b as f32 * alpha_b * (1.0 - alpha_a)) / alpha_o) as u8
}

impl Pixel {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(((a as u32) << (3 * 8)) | ((b as u32) << (2 * 8)) | ((g as u32) << (8)) | (r as u32))
    }

    pub const fn new(color: u32) -> Pixel {
        Pixel(color)
    }

    pub const fn red(&self) -> u32 {
        self.0 & 0xff
    }

    pub const fn green(&self) -> u32 {
        self.0 >> 8 & 0xff
    }

    pub const fn blue(&self) -> u32 {
        self.0 >> 16 & 0xff
    }

    pub const fn alpha(&self) -> u32 {
        self.0 >> 24 & 0xff
    }

    pub fn blend(&self, pixel: &Pixel) -> Pixel {
        if self.alpha() == 0 || pixel.alpha() == 255 {
            *pixel
        } else if pixel.alpha() == 0 {
            *self
        } else {
            let alpha_a = pixel.alpha() as f32 / 255.0;
            let alpha_b = self.alpha() as f32 / 255.0;
            let alpha_o = blend_a(alpha_a, alpha_b);
            if alpha_o > 0.0 {
                let red_o = blend_c(pixel.red(), alpha_a, self.red(), alpha_b, alpha_o);
                let green_o = blend_c(pixel.green(), alpha_a, self.green(), alpha_b, alpha_o);
                let blue_o = blend_c(pixel.blue(), alpha_a, self.blue(), alpha_b, alpha_o);
                Pixel::rgba(red_o, green_o, blue_o, (alpha_o * 255.0) as u8)
            } else {
                Pixel::new(0)
            }
        }
    }
}

impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
            .field("r", &self.red())
            .field("g", &self.green())
            .field("b", &self.blue())
            .field("a", &self.alpha())
            .finish()
    }
}
