use crate::card::Card;
use crate::paytable::PayTable;

use arrayvec::{ArrayString, ArrayVec};

pub type Hold = [bool; 5];

pub fn hold_from_usize(num: usize) -> Hold {
    [
        num & 0x01 != 0,
        num & 0x02 != 0,
        num & 0x04 != 0,
        num & 0x08 != 0,
        num & 0x10 != 0,
    ]
}

fn s_hold_str(hold: bool) -> &'static str {
    if hold {
        "H"
    } else {
        "-"
    }
}

fn hold_str(hold: &Hold) -> ArrayString<5> {
    let mut string = ArrayString::<5>::new();
    string.try_push_str(s_hold_str(*hold.get(0).unwrap_or(&false)));
    string.try_push_str(s_hold_str(*hold.get(1).unwrap_or(&false)));
    string.try_push_str(s_hold_str(*hold.get(2).unwrap_or(&false)));
    string.try_push_str(s_hold_str(*hold.get(3).unwrap_or(&false)));
    string.try_push_str(s_hold_str(*hold.get(4).unwrap_or(&false)));
    string
}

fn hold_index(hold: &Hold) -> Option<(Hold, usize)> {
    let mut new_hold = *hold;
    for (i, is_hold) in new_hold.iter_mut().enumerate() {
        if !*is_hold {
            *is_hold = true;
            return Some((new_hold, i));
        }
    }
    None
}

fn get_value(
    hand: &[Card; 5],
    deck: &[Card],
    paytable: &PayTable,
    wager: usize,
    hold: &Hold,
) -> (u64, u64) {
    let mut value = (0, 0);
    if let Some((new_hold, index)) = hold_index(hold) {
        let mut new_hand = *hand;
        for i in 0..deck.len() {
            if let Some(card) = new_hand.get_mut(index) {
                if let Some(first_card) = deck.get(i) {
                    if let Some(remainder) = deck.get(i + 1..) {
                        *card = *first_card;
                        let new_value = get_value(&new_hand, remainder, paytable, wager, &new_hold);
                        value = (value.0 + new_value.0, value.1 + new_value.1)
                    }
                }
            }
        }
    }
    (
        paytable.get_payout(hand, wager).unwrap_or(0) + value.0,
        1 + value.1,
    )
}

type Suggestions = ArrayVec<(ArrayString<5>, f64), 32>;

pub struct Strategy {
    suggestions: Suggestions,
}

impl Strategy {
    pub const fn default() -> Self {
        Strategy {
            suggestions: ArrayVec::new_const(),
        }
    }

    pub fn reset(&mut self) {
        self.suggestions.clear();
    }

    pub fn is_finished(&self) -> bool {
        self.suggestions.is_full()
            || (self.suggestions.remaining_capacity() == 1
                && self
                    .suggestions
                    .last()
                    .unwrap_or(&(ArrayString::new(), 0.0))
                    .1
                    > 2.0)
    }

    pub fn get_suggestions(
        &mut self,
        hand: &[Card; 5],
        deck: &[Card],
        paytable: &PayTable,
        wager: usize,
    ) -> Option<&Suggestions> {
        if self.is_finished() {
            return Some(&self.suggestions);
        }

        let hold = hold_from_usize(self.suggestions.remaining_capacity() - 1);
        let (value, count) = get_value(hand, deck, paytable, wager, &hold);
        let new_suggestion = (hold_str(&hold), value as f64 / count as f64);
        for i in 0..self.suggestions.len() {
            if let Some(old_suggestion) = self.suggestions.get(i) {
                if old_suggestion.1 > new_suggestion.1 {
                    let mut back: Suggestions = ArrayVec::new();
                    if let Some(items) = self.suggestions.get(i..) {
                        back.try_extend_from_slice(items);
                    }
                    self.suggestions.truncate(i);
                    self.suggestions.try_push(new_suggestion);
                    self.suggestions.try_extend_from_slice(&back);

                    return None;
                }
            }
        }
        self.suggestions.try_push(new_suggestion);
        None
    }
}
