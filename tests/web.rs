#![cfg(target_arch = "wasm32")]

use video_poker::card::Card;
use video_poker::deck::Deck;
use video_poker::paytable::PayTable;
use video_poker::pixel::Pixel;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pixel_rgba() {
    let pixel = Pixel::rgba(1, 2, 3, 4);
    assert!(pixel.red() == 1);
    assert!(pixel.green() == 2);
    assert!(pixel.blue() == 3);
    assert!(pixel.alpha() == 4);
}

#[wasm_bindgen_test]
fn pixel_blend() {
    let red = Pixel::rgba(255, 0, 0, 255);
    let blue = Pixel::rgba(0, 0, 255, 255);
    let tred = Pixel::rgba(255, 0, 0, 100);
    let tblue = Pixel::rgba(0, 0, 255, 100);

    assert!(red.blend(&blue) == blue);
    println!("blend: {:?}", red.blend(&tblue));
    assert!(red.blend(&tblue) == Pixel::rgba(155, 0, 100, 255));
}

#[wasm_bindgen_test]
fn shuffle() {
    let mut deck = Deck::default();
    deck.shuffle();
}

#[wasm_bindgen_test]
fn jacks_or_better() {
    let paytable = PayTable::jacks_or_better();
    // lose
    assert!(paytable
        .get_payout(
            &[
                Card::new(0, 0),
                Card::new(2, 1),
                Card::new(4, 2),
                Card::new(6, 3),
                Card::new(8, 0)
            ],
            2
        )
        .is_none());

    // jacks or better
    assert!(
        paytable.get_payout(
            &[
                Card::new(0, 0),
                Card::new(2, 1),
                Card::new(4, 2),
                Card::new(10, 3),
                Card::new(10, 0)
            ],
            2
        ) == Some(3)
    );

    // worse than jacks
    assert!(paytable
        .get_payout(
            &[
                Card::new(0, 0),
                Card::new(2, 1),
                Card::new(4, 2),
                Card::new(9, 3),
                Card::new(9, 0)
            ],
            2
        )
        .is_none());

    // double ace
    assert!(
        paytable.get_payout(
            &[
                Card::new(0, 0),
                Card::new(2, 1),
                Card::new(4, 2),
                Card::new(0, 3),
                Card::new(10, 0)
            ],
            2
        ) == Some(3)
    );

    // two pair
    assert!(
        paytable.get_payout(
            &[
                Card::new(0, 0),
                Card::new(2, 1),
                Card::new(2, 2),
                Card::new(6, 3),
                Card::new(6, 0)
            ],
            3
        ) == Some(8)
    );

    // three of a kind
    assert!(
        paytable.get_payout(
            &[
                Card::new(0, 0),
                Card::new(2, 1),
                Card::new(8, 2),
                Card::new(8, 3),
                Card::new(8, 0)
            ],
            4
        ) == Some(15)
    );

    // straight
    assert!(
        paytable.get_payout(
            &[
                Card::new(3, 0),
                Card::new(4, 1),
                Card::new(5, 2),
                Card::new(6, 3),
                Card::new(7, 0)
            ],
            0
        ) == Some(4)
    );

    // straight A-5
    assert!(
        paytable.get_payout(
            &[
                Card::new(0, 0),
                Card::new(1, 1),
                Card::new(2, 2),
                Card::new(3, 3),
                Card::new(4, 0)
            ],
            0
        ) == Some(4)
    );

    // straight T-A
    assert!(
        paytable.get_payout(
            &[
                Card::new(9, 0),
                Card::new(10, 1),
                Card::new(11, 2),
                Card::new(12, 3),
                Card::new(13, 0)
            ],
            0
        ) == Some(4)
    );

    // almost straight
    assert!(paytable
        .get_payout(
            &[
                Card::new(3, 0),
                Card::new(4, 1),
                Card::new(5, 2),
                Card::new(6, 3),
                Card::new(8, 0)
            ],
            0
        )
        .is_none());

    // flush
    assert!(
        paytable.get_payout(
            &[
                Card::new(1, 1),
                Card::new(3, 1),
                Card::new(5, 1),
                Card::new(7, 1),
                Card::new(9, 1)
            ],
            2
        ) == Some(18)
    );

    // full house
    assert!(
        paytable.get_payout(
            &[
                Card::new(3, 0),
                Card::new(3, 1),
                Card::new(5, 2),
                Card::new(5, 3),
                Card::new(5, 0)
            ],
            1
        ) == Some(18)
    );

    // full house inverted
    assert!(
        paytable.get_payout(
            &[
                Card::new(5, 2),
                Card::new(5, 3),
                Card::new(5, 0),
                Card::new(3, 0),
                Card::new(3, 1)
            ],
            3
        ) == Some(36)
    );
}
