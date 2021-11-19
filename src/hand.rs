use crate::card::Card;
use crate::display::Display;
use crate::font::{Font, FONT_CHAR_HEIGHT, FONT_CHAR_WIDTH};
use crate::pixel::Pixel;

#[derive(Clone, Copy)]
pub struct Hand {
    card: Card,
    hold: bool,
}

impl Hand {
    pub const fn empty() -> Self {
        Hand {
            card: Card::empty(),
            hold: false,
        }
    }

    pub fn toggle_hold(&mut self) {
        self.hold = !self.hold;
    }

    pub fn is_hold(&self) -> bool {
        self.hold
    }

    pub fn set_card(&mut self, card: &Card) {
        self.card = *card;
        self.hold = false;
    }

    pub fn get_card(&self) -> &Card {
        &self.card
    }

    pub fn render(&self, display: &mut Display, font: &Font, x: i32, y: i32, scale: i32) {
        self.card.render(display, font, x, y, scale);
        let hold_y = y + FONT_CHAR_HEIGHT as i32 * scale;
        let background = Pixel::rgba(0, 200, 200, if self.hold { 255 } else { 0 });
        let foreground = Pixel::rgba(255, 255, 255, if self.hold { 255 } else { 0 });
        display.fill_rect(
            x,
            hold_y,
            FONT_CHAR_WIDTH as i32 * scale * 2,
            FONT_CHAR_HEIGHT as i32 * scale / 2,
            background,
        );
        font.render_bytes(display, b"HOLD", x, hold_y, scale / 2, foreground);
    }
}
