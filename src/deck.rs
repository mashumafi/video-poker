use crate::card::Card;
use crate::math::rand_usize;

pub struct Deck {
    cards: [Card; 52],
    index: usize,
}

fn fisher_yates_shuffle(cards: &mut [Card]) {
    for i in 0..cards.len() - 1 {
        let j = rand_usize(i, cards.len());
        unsafe {
            let pa: *mut Card = cards.get_unchecked_mut(i);
            let pb: *mut Card = cards.get_unchecked_mut(j);
            core::ptr::swap(pa, pb);
        }
    }
}

impl Deck {
    pub const fn default() -> Self {
        let mut cards = [Card::empty(); 52];
        let mut i: u8 = 0;
        while i < 52 {
            cards[i as usize] = Card::new(i % 13, i / 13);
            i += 1
        }
        Deck { cards, index: 0 }
    }

    pub fn shuffle(&mut self) {
        fisher_yates_shuffle(&mut self.cards);
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn draw(&mut self) -> Option<&Card> {
        let card = self.cards.get(self.index);
        self.index += 1;
        card
    }

    pub fn remainder(&self) -> Option<&[Card]> {
        self.cards.get(self.index..)
    }
}
