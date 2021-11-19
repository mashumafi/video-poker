use crate::card::Card;
use crate::display::Display;
use crate::font::{Font, FONT_CHAR_HEIGHT, FONT_CHAR_WIDTH};
use crate::math::{max, min};
use crate::pixel::Pixel;

use arrayvec::ArrayVec;

const ROW_NAME_LENGTH: i32 = 15;
const HEIGHT_PADDING: i32 = 1;
const MAX_NUM_LENGTH: i32 = 4;

#[derive(Clone, Copy)]
struct PayRow {
    name: [u8; ROW_NAME_LENGTH as usize],
    validate: &'static dyn Fn(&[Card]) -> bool,
    payout: [u64; 5],
}

fn royal_flush(cards: &[Card]) -> bool {
    if !straight_flush(cards) {
        return false;
    }
    for card in cards {
        if card.rank() == 0 {
            return true;
        }
    }
    false
}

fn straight_flush(cards: &[Card]) -> bool {
    straight(cards) && flush(cards)
}

fn four_of_a_kind(cards: &[Card]) -> bool {
    let mut ranks = [0; 13];
    for card in cards {
        if let Some(rank) = ranks.get_mut(card.rank() as usize) {
            *rank += 1;
            if *rank == 4 {
                return true;
            }
        }
    }
    false
}

fn full_house(cards: &[Card]) -> bool {
    let mut ranks = [0; 13];
    let mut unique_ranks = 0;
    for card in cards {
        if let Some(rank) = ranks.get_mut(card.rank() as usize) {
            *rank += 1;
            if *rank == 1 {
                unique_ranks += 1;
            }
            if unique_ranks > 2 {
                return false;
            }
        }
    }
    true
}

fn flush(cards: &[Card]) -> bool {
    let mut suits = [0; 4];
    for card in cards {
        if let Some(suit) = suits.get_mut(card.suit() as usize) {
            *suit += 1;
            if *suit == 5 {
                return true;
            }
        }
    }
    false
}

fn straight(cards: &[Card]) -> bool {
    let mut ranks = [0; 14];
    let mut lo_rank = 13;
    let mut hi_rank = 0;
    for card in cards {
        let mut mranks = ArrayVec::<_, 2>::new();
        if card.rank() == 0 {
            mranks.try_push(0);
            mranks.try_push(13);
        } else {
            mranks.try_push(card.rank());
        }
        for rank in mranks {
            if let Some(rank) = ranks.get_mut(rank as usize) {
                *rank += 1;
                if *rank > 1 {
                    return false;
                }
                lo_rank = min(lo_rank, card.rank() as i32);
                hi_rank = max(hi_rank, card.rank() as i32);
                if hi_rank - lo_rank > 4 {
                    return false;
                }
            }
        }
    }
    hi_rank - lo_rank == 4
}

fn three_of_a_kind(cards: &[Card]) -> bool {
    let mut ranks = [0; 13];
    for card in cards {
        if let Some(rank) = ranks.get_mut(card.rank() as usize) {
            *rank += 1;
            if *rank == 3 {
                return true;
            }
        }
    }
    false
}

fn two_pair(cards: &[Card]) -> bool {
    let mut ranks = [0; 13];
    let mut pair_count = 0;
    for card in cards {
        if let Some(rank) = ranks.get_mut(card.rank() as usize) {
            *rank += 1;
            if *rank == 2 {
                pair_count += 1;
            }
            if pair_count == 2 {
                return true;
            }
        }
    }
    false
}

fn jacks_or_better(cards: &[Card]) -> bool {
    let mut ranks = [0; 13];
    for card in cards {
        if let Some(rank) = ranks.get_mut(card.rank() as usize) {
            if (1..10).contains(&card.rank()) {
                continue;
            }
            *rank += 1;
            if *rank > 1 {
                return true;
            }
        }
    }
    false
}

impl PayRow {
    fn is_winner(&self, cards: &[Card]) -> bool {
        (self.validate)(cards)
    }

    fn get_payout(&self, cards: &[Card], credits: usize) -> Option<u64> {
        if (self.validate)(cards) {
            self.payout.get(credits).copied()
        } else {
            None
        }
    }
}

pub struct PayTable {
    pay_row: [PayRow; 9],
}

impl PayTable {
    pub const fn jacks_or_better() -> Self {
        PayTable {
            pay_row: [
                PayRow {
                    name: *b"ROYAL FLUSH....",
                    validate: &royal_flush,
                    payout: [250, 500, 750, 1000, 4000],
                },
                PayRow {
                    name: *b"STRAIGHT FLUSH.",
                    validate: &straight_flush,
                    payout: [50, 100, 150, 200, 250],
                },
                PayRow {
                    name: *b"FOUR OF A KIND.",
                    validate: &four_of_a_kind,
                    payout: [25, 50, 75, 100, 125],
                },
                PayRow {
                    name: *b"FULL HOUSE.....",
                    validate: &full_house,
                    payout: [9, 18, 27, 36, 45],
                },
                PayRow {
                    name: *b"FLUSH..........",
                    validate: &flush,
                    payout: [6, 12, 18, 24, 30],
                },
                PayRow {
                    name: *b"STRAIGHT.......",
                    validate: &straight,
                    payout: [4, 8, 12, 16, 20],
                },
                PayRow {
                    name: *b"THREE OF A KIND",
                    validate: &three_of_a_kind,
                    payout: [3, 6, 9, 12, 15],
                },
                PayRow {
                    name: *b"TWO PAIR.......",
                    validate: &two_pair,
                    payout: [2, 4, 6, 8, 10],
                },
                PayRow {
                    name: *b"JACKS OR BETTER",
                    validate: &jacks_or_better,
                    payout: [1, 2, 3, 4, 5],
                },
            ],
        }
    }

    pub fn get_payout(&self, cards: &[Card], credits: usize) -> Option<u64> {
        for row in self.pay_row.iter() {
            if let Some(payout) = row.get_payout(cards, credits) {
                return Some(payout);
            }
        }
        None
    }

    pub fn get_winning_index(&self, cards: &[Card]) -> Option<usize> {
        for (i, row) in self.pay_row.iter().enumerate() {
            if row.is_winner(cards) {
                return Some(i);
            }
        }
        None
    }

    pub fn render(
        &self,
        display: &mut Display,
        font: &Font,
        x: i32,
        y: i32,
        scale: i32,
        wager: usize,
        winner: Option<usize>,
    ) {
        display.fill_rect(
            x,
            y,
            FONT_CHAR_WIDTH as i32 * 40 * scale,
            (FONT_CHAR_HEIGHT as i32 + HEIGHT_PADDING) * self.pay_row.len() as i32 * scale,
            Pixel::rgba(0, 0, 0, 64),
        );

        for (i, row) in self.pay_row.iter().enumerate() {
            let actual_y = y + i as i32 * scale * (FONT_CHAR_HEIGHT as i32 + HEIGHT_PADDING);
            font.render_bytes(
                display,
                &row.name,
                x,
                actual_y,
                scale,
                Pixel::rgba(255, 255, 255, 255),
            );

            let start_x = x + (ROW_NAME_LENGTH + 1) * FONT_CHAR_WIDTH as i32 * scale;
            for (j, payout) in row.payout.iter().enumerate() {
                let actual_x =
                    start_x + j as i32 * (MAX_NUM_LENGTH + 1) * FONT_CHAR_WIDTH as i32 * scale;
                if j == wager || i == winner.unwrap_or(50) {
                    display.fill_rect(
                        actual_x,
                        actual_y,
                        MAX_NUM_LENGTH * FONT_CHAR_WIDTH as i32 * scale,
                        FONT_CHAR_HEIGHT as i32 * scale,
                        Pixel::rgba(0, 0, 255, 255),
                    );
                }
                font.render_bytes(
                    display,
                    &render_num(*payout),
                    actual_x,
                    actual_y,
                    scale,
                    Pixel::rgba(255, 255, 255, 255),
                );
            }
        }
    }
}

fn render_num(mut payout: u64) -> [u8; MAX_NUM_LENGTH as usize] {
    let mut num = [b' ', b' ', b' ', b' '];
    let mut idx = 0;
    while payout > 0 {
        if let Some(char_ref) = num.get_mut(idx) {
            *char_ref = (payout % 10) as u8 + b'0';
            idx += 1;
            payout /= 10;
        }
    }
    num.reverse();
    num
}
