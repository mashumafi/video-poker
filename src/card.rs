use crate::display::Display;
use crate::font::Font;
use crate::pixel::Pixel;

const RED: Pixel = Pixel::rgba(255, 0, 0, 255);
const BLACK: Pixel = Pixel::rgba(0, 0, 0, 255);

#[derive(Clone, Copy)]
pub struct Card {
    rank: u8,
    suit: u8,
}

impl Card {
    pub const fn new(rank: u8, suit: u8) -> Self {
        Card { rank, suit }
    }
    pub const fn empty() -> Card {
        Card {
            rank: 255,
            suit: 255,
        }
    }

    pub const fn name(&self) -> [u8; 2] {
        [self.rank_name(), self.suit_name()]
    }

    pub const fn suit(&self) -> u8 {
        self.suit
    }

    pub const fn suit_name(&self) -> u8 {
        match self.suit {
            0 => b'~' + 1,
            1 => b'~' + 4,
            2 => b'~' + 3,
            3 => b'~' + 2,
            4_u8..=u8::MAX => b'X',
        }
    }

    pub const fn rank(&self) -> u8 {
        self.rank
    }

    pub const fn rank_name(&self) -> u8 {
        match self.rank {
            0 => b'A',
            1..=8 => b'1' + self.rank,
            9 => b'T',
            10 => b'J',
            11 => b'Q',
            12 => b'K',
            13_u8..=u8::MAX => b'X',
        }
    }

    pub const fn color(&self) -> Pixel {
        match self.suit {
            0 => BLACK,
            1 => RED,
            2 => BLACK,
            3 => RED,
            4..=u8::MAX => Pixel::rgba(0, 0, 255, 255),
        }
    }

    pub fn render(&self, display: &mut Display, font: &Font, x: i32, y: i32, scale: i32) {
        font.render_bytes(display, &self.name(), x, y, scale, self.color());
    }
}
