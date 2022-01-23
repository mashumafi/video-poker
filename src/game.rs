use crate::deck::Deck;
use crate::display::{Display, DISPLAY_HEIGHT};
use crate::font::{Font, FONT_CHAR_HEIGHT, FONT_CHAR_WIDTH};
use crate::hand::Hand;
use crate::label::Label;
use crate::paytable::PayTable;
use crate::pixel::Pixel;
use crate::strategy::Strategy;


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_hold_value(hold: &str, value: f64);
}

const COMPRESSED_FONT: [u8; 652] = [
    0x00, 0x11, 0x20, 0xa1, 0x41, 0x0c, 0x0e, 0x08, 0x08, 0x40, 0x00, 0x05, 0x38, 0x20, 0x00, 0x01,
    0x20, 0xa1, 0x43, 0xcc, 0x92, 0x08, 0x10, 0x21, 0x50, 0x80, 0x00, 0x02, 0x02, 0x44, 0x60, 0x00,
    0x01, 0x20, 0x03, 0xe5, 0x01, 0x14, 0x00, 0x01, 0x20, 0x10, 0xe0, 0x80, 0x00, 0x02, 0x04, 0x4c,
    0xa0, 0x00, 0x01, 0x20, 0x01, 0x43, 0x82, 0x08, 0x00, 0x01, 0x20, 0x11, 0xf3, 0xe0, 0x0f, 0x80,
    0x08, 0x54, 0x20, 0x00, 0x01, 0x20, 0x03, 0xe1, 0x44, 0x15, 0x00, 0x01, 0x20, 0x10, 0xe0, 0x81,
    0x00, 0x02, 0x10, 0x64, 0x20, 0x00, 0x02, 0x01, 0x47, 0x89, 0x92, 0x00, 0x01, 0x10, 0x21, 0x50,
    0x81, 0x00, 0x02, 0x20, 0x44, 0x20, 0x00, 0x01, 0x20, 0x01, 0x41, 0x01, 0x8d, 0x00, 0x01, 0x08,
    0x40, 0x00, 0x01, 0x02, 0x00, 0x01, 0x04, 0x00, 0x01, 0x38, 0xf8, 0x00, 0x20, 0x38, 0x70, 0x63,
    0xe3, 0x8f, 0x8e, 0x1c, 0x00, 0x04, 0x07, 0x0e, 0x1c, 0x78, 0x70, 0x44, 0x88, 0xa2, 0x04, 0x00,
    0x01, 0x91, 0x22, 0x10, 0x20, 0x20, 0x02, 0x08, 0x91, 0x22, 0x44, 0x88, 0x04, 0x09, 0x22, 0x04,
    0x01, 0x11, 0x22, 0x00, 0x02, 0x43, 0xe1, 0x08, 0x97, 0x22, 0x44, 0x80, 0x08, 0x31, 0xf3, 0xc7,
    0x82, 0x0e, 0x1e, 0x00, 0x02, 0x80, 0x00, 0x01, 0x81, 0x15, 0x3e, 0x78, 0x80, 0x10, 0x08, 0x20,
    0x24, 0x44, 0x11, 0x02, 0x00, 0x01, 0x20, 0x43, 0xe1, 0x02, 0x17, 0x22, 0x44, 0x80, 0x20, 0x88,
    0x20, 0x24, 0x44, 0x11, 0x02, 0x10, 0x20, 0x20, 0x02, 0x00, 0x01, 0x10, 0x22, 0x44, 0x88, 0x7c,
    0x70, 0x23, 0xc3, 0x84, 0x0e, 0x1c, 0x00, 0x01, 0x40, 0x00, 0x02, 0x02, 0x0e, 0x22, 0x78, 0x70,
    0x00, 0x20, 0x78, 0xf9, 0xf1, 0xc4, 0x4f, 0x9f, 0x22, 0x40, 0x89, 0x11, 0xc7, 0x87, 0x1e, 0x1e,
    0x7c, 0x88, 0x44, 0x81, 0x02, 0x24, 0x42, 0x01, 0x22, 0x40, 0xd9, 0x12, 0x24, 0x48, 0x91, 0x20,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x04, 0x42, 0x01, 0x24, 0x40, 0xa9, 0x92, 0x24, 0x48, 0x91, 0x20,
    0x10, 0x88, 0x44, 0xf1, 0xe2, 0x07, 0xc2, 0x01, 0x38, 0x40, 0x89, 0x52, 0x27, 0x88, 0x9e, 0x1c,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x64, 0x42, 0x01, 0x24, 0x40, 0x89, 0x32, 0x24, 0x0a, 0x91, 0x02,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x24, 0x42, 0x11, 0x22, 0x40, 0x89, 0x12, 0x24, 0x09, 0x11, 0x02,
    0x10, 0x88, 0x78, 0xf9, 0x01, 0xc4, 0x4f, 0x8e, 0x22, 0x7c, 0x89, 0x11, 0xc4, 0x06, 0x91, 0x3c,
    0x10, 0x70, 0x00, 0x20, 0x44, 0x89, 0x12, 0x27, 0xc3, 0x00, 0x01, 0x18, 0x10, 0x00, 0x01, 0x80,
    0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x18, 0x00, 0x01, 0x44, 0x89, 0x12, 0x20, 0x42, 0x10, 0x08,
    0x28, 0x00, 0x01, 0x40, 0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x20, 0x00, 0x01, 0x44, 0x88, 0xa1,
    0x40, 0x82, 0x08, 0x08, 0x00, 0x02, 0x01, 0xc7, 0x87, 0x0f, 0x1c, 0x7c, 0x78, 0x44, 0x88, 0x40,
    0x81, 0x02, 0x04, 0x08, 0x00, 0x03, 0x24, 0x48, 0x91, 0x22, 0x20, 0x88, 0x44, 0xa8, 0xa0, 0x82,
    0x02, 0x02, 0x08, 0x00, 0x02, 0x01, 0xe4, 0x48, 0x11, 0x3e, 0x20, 0x78, 0x28, 0xd9, 0x10, 0x84,
    0x02, 0x01, 0x08, 0x00, 0x02, 0x02, 0x24, 0x48, 0x91, 0x20, 0x20, 0x08, 0x10, 0x89, 0x10, 0x87,
    0xc3, 0x00, 0x01, 0x18, 0x00, 0x01, 0xf8, 0x01, 0xe7, 0x87, 0x0f, 0x1e, 0x20, 0x70, 0x00, 0x20,
    0x40, 0x20, 0x12, 0x04, 0x00, 0x06, 0x02, 0x00, 0x05, 0x40, 0x00, 0x01, 0x02, 0x04, 0x00, 0x06,
    0x02, 0x00, 0x05, 0x78, 0xe0, 0x72, 0x44, 0x0d, 0x1e, 0x1c, 0x78, 0x79, 0x61, 0xe7, 0x88, 0x91,
    0x22, 0x44, 0x88, 0x44, 0x20, 0x13, 0x84, 0x0a, 0x91, 0x22, 0x44, 0x89, 0x92, 0x02, 0x08, 0x91,
    0x22, 0x28, 0x88, 0x44, 0x20, 0x12, 0x44, 0x0a, 0x91, 0x22, 0x78, 0x79, 0x01, 0xc2, 0x08, 0x91,
    0x22, 0x10, 0x78, 0x44, 0x21, 0x12, 0x24, 0x08, 0x91, 0x22, 0x40, 0x09, 0x00, 0x01, 0x22, 0x48,
    0x8a, 0x2a, 0x28, 0x08, 0x44, 0xf8, 0xe2, 0x23, 0x88, 0x91, 0x1c, 0x40, 0x09, 0x03, 0xc1, 0x87,
    0x84, 0x14, 0x44, 0x70, 0x00, 0x21, 0x10, 0x41, 0x00, 0x01, 0x02, 0x04, 0x1c, 0x28, 0x00, 0x09,
    0x20, 0x40, 0x80, 0x07, 0x0e, 0x1c, 0x7c, 0x00, 0x08, 0x7c, 0x20, 0x40, 0x82, 0x4f, 0x8e, 0x08,
    0x7c, 0x00, 0x08, 0x08, 0x40, 0x40, 0x45, 0x8f, 0x9f, 0x3e, 0x7c, 0x00, 0x08, 0x10, 0x20, 0x40,
    0x80, 0x07, 0x0e, 0x3e, 0x38, 0x00, 0x08, 0x20, 0x20, 0x40, 0x80, 0x02, 0x0e, 0x08, 0x38, 0x00,
    0x08, 0x7c, 0x10, 0x41, 0x00, 0x01, 0x07, 0x04, 0x1c, 0x10, 0x00, 0xb8,
];

enum State {
    Begin,
    Hold(usize),
    HoldHint(usize),
    End,
}

static mut DISPLAY: Display = Display::new();
static mut FONT: Font = Font::default();

pub struct Game {
    state: State,
    deck: Deck,
    paytable: PayTable,
    credits: usize,
    hand: [Hand; 5],
    wager: usize,
    strategy: Strategy,
    score: f64,
}

impl Game {
    pub const fn default() -> Self {
        Game {
            state: State::Begin,
            deck: Deck::default(),
            paytable: PayTable::jacks_or_better(),
            credits: 0,
            hand: [Hand::empty(); 5],
            wager: 5,
            strategy: Strategy::default(),
            score: 0.0,
        }
    }

    pub fn setup(&mut self) {
        unsafe {
            FONT.decompress_from_bytes(&COMPRESSED_FONT);

            self.redraw();
        }
    }

    fn deal(&mut self) {
        match self.state {
            State::Hold(wager) | State::HoldHint(wager) => {
                for hand in self.hand.iter_mut() {
                    if !hand.is_hold() {
                        if let Some(card) = self.deck.draw() {
                            hand.set_card(card);
                        }
                    }
                }

                let cards = self.hand.map(|hand| *hand.get_card());
                self.credits += self.paytable.get_payout(&cards, wager - 1).unwrap_or(0) as usize;

                if let Some(cards) = self.deck.remainder() {
                    if let Some(suggestions) = self.strategy.get_suggestions(
                        &self.hand.map(|hand| *hand.get_card()),
                        cards,
                        &self.paytable,
                        self.wager - 1,
                    ) {
                        let mut suggestions = suggestions.iter().rev();
                        if let Some(last) = suggestions.next() {
                            if let Some(prev) = suggestions.clone().next() {
                                let hand = self
                                    .hand
                                    .map(|hand| if hand.is_hold() { b'H' } else { b'-' });
                                if last.0.as_str().as_bytes() == hand {
                                    self.score += last.1 - prev.1
                                } else {
                                    for suggestion in suggestions {
                                        if suggestion.0.as_str().as_bytes() == hand {
                                            self.score -= last.1 - suggestion.1;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                self.state = State::End;
            }
            State::Begin | State::End => {
                self.deck.reset();
                self.deck.shuffle();

                for hand in self.hand.iter_mut() {
                    if let Some(card) = self.deck.draw() {
                        hand.set_card(card);
                    }
                }

                if self.credits > self.wager {
                    self.credits -= self.wager;

                    self.strategy.reset();
                    self.state = State::Hold(self.wager);
                }
            }
        }
    }

    pub fn key_down(&mut self, code: i32) {
        match self.state {
            State::Hold(_) | State::HoldHint(_) => {
                if (49..=53).contains(&code) {
                    if let Some(hand) = self.hand.get_mut(code as usize - 49) {
                        hand.toggle_hold();
                    }
                }

                if 32 == code {
                    self.deal();
                }
            }
            State::Begin | State::End => {
                if 32 == code {
                    self.deal();
                }
            }
        }

        self.redraw();
    }

    fn redraw(&mut self) {
        unsafe {
            DISPLAY.fill(Pixel::rgba(64, 255, 64, 255));
            let card_y = (FONT_CHAR_HEIGHT + 2) as i32 * 2 * (9);
            for (i, hand) in self.hand.iter().enumerate() {
                hand.render(
                    &mut DISPLAY,
                    &FONT,
                    FONT_CHAR_WIDTH as i32 * 3 * 5 * i as i32,
                    card_y,
                    5,
                );
            }
            let mut label = Label::empty();
            label.push_bytes(b"Credits: ");
            label.push_usize(self.credits);
            label.render(
                &mut DISPLAY,
                &FONT,
                0,
                DISPLAY_HEIGHT as i32 - FONT_CHAR_HEIGHT as i32 * 4,
                4,
                Pixel::rgba(255, 255, 255, 255),
            );

            label.clear();
            label.push_bytes(b"Score: ");
            label.push_f64(self.score);
            label.render(
                &mut DISPLAY,
                &FONT,
                0,
                DISPLAY_HEIGHT as i32 - FONT_CHAR_HEIGHT as i32 * 4 * 2,
                4,
                Pixel::rgba(255, 255, 255, 255),
            );

            if let State::HoldHint(_wager) = self.state {
                if let Some(cards) = self.deck.remainder() {
                    if let Some(suggestions) = self.strategy.get_suggestions(
                        &self.hand.map(|hand| *hand.get_card()),
                        cards,
                        &self.paytable,
                        self.wager - 1,
                    ) {
                        for (y, name) in suggestions
                            .iter()
                            .rev()
                            .take(5)
                            .map(|(name, _value)| name)
                            .enumerate()
                        {
                            for (x, hold) in name
                                .as_str()
                                .as_bytes()
                                .iter()
                                .map(|hold| hold == &b'H')
                                .enumerate()
                            {
                                if hold {
                                    DISPLAY.fill_rect(
                                        x as i32 * FONT_CHAR_WIDTH as i32 * 3 * 5,
                                        card_y
                                            + FONT_CHAR_HEIGHT as i32 * 2 * 5
                                            + FONT_CHAR_HEIGHT as i32 * 5 * y as i32,
                                        FONT_CHAR_WIDTH as i32 * 2 * 5,
                                        (FONT_CHAR_HEIGHT - 1) as i32 * 5,
                                        Pixel::rgba(0, 0, 255, 255),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            let winning_index = match self.state {
                State::End => self
                    .paytable
                    .get_winning_index(&self.hand.map(|hand| *hand.get_card())),
                _ => None,
            };
            self.paytable
                .render(&mut DISPLAY, &FONT, 0, 0, 2, self.wager - 1, winning_index);
        }
    }

    pub fn update_suggestions(&mut self) {
        if let State::Hold(wager) = self.state {
            if let Some(cards) = self.deck.remainder() {
                if let Some(suggestions) = self.strategy.get_suggestions(
                    &self.hand.map(|hand| *hand.get_card()),
                    cards,
                    &self.paytable,
                    self.wager - 1,
                ) {
                    for (name, value) in suggestions.iter() {
                        log_hold_value(name, *value);
                    }

                    self.state = State::HoldHint(wager);

                    self.redraw();
                }
            }
        }
    }

    pub fn key_up(&mut self, _code: i32) {}

    pub fn mouse_move(&mut self, _x: i32, _y: i32) {}

    pub fn mouse_down(&mut self, _x: i32, _y: i32) {}

    pub fn mouse_up(&mut self, _x: i32, _y: i32) {}

    pub fn get_display(&self) -> &Display {
        unsafe { &DISPLAY }
    }

    pub fn add_bankroll(&mut self, bankroll: usize) {
        self.credits += bankroll;

        self.redraw();
    }

    pub fn cashout(&mut self) -> usize {
        let bankroll = self.credits;
        self.credits = 0;

        self.redraw();

        bankroll
    }
}
