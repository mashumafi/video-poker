use crate::math::clamp;
use crate::pixel::Pixel;

pub const DISPLAY_WIDTH: usize = 800;
pub const DISPLAY_HEIGHT: usize = 600;

#[repr(C)]
pub struct Display {
    pixels: [Pixel; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
    pub const fn new() -> Display {
        Display {
            pixels: [Pixel::new(0); DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }
    pub fn fill(&mut self, pixel: Pixel) {
        for pixel_ref in self.pixels.iter_mut() {
            *pixel_ref = pixel;
        }
    }

    pub fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        let x1 = clamp(x0, 0, (DISPLAY_WIDTH - 1) as i32);
        let x2 = clamp(x0 + w - 1, 0, (DISPLAY_WIDTH - 1) as i32);
        let y1 = clamp(y0, 0, (DISPLAY_HEIGHT - 1) as i32);
        let y2 = clamp(y0 + h - 1, 0, (DISPLAY_HEIGHT - 1) as i32);

        for y in y1..=y2 {
            for x in x1..=x2 {
                self.blend(x, y, pixel);
            }
        }
    }

    pub fn put(&mut self, x: i32, y: i32, pixel: Pixel) {
        let index = y as usize * DISPLAY_WIDTH + x as usize;
        if let Some(pixel_ref) = self.pixels.get_mut(index) {
            *pixel_ref = pixel;
        }
    }

    pub fn blend(&mut self, x: i32, y: i32, pixel: Pixel) {
        let index = y as usize * DISPLAY_WIDTH + x as usize;
        if let Some(pixel_ref) = self.pixels.get_mut(index) {
            *pixel_ref = pixel_ref.blend(&pixel);
        }
    }
}
