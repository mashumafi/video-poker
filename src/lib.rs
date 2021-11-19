#![no_std]
#![feature(const_fn_trait_bound)]

pub mod card;
pub mod deck;
mod display;
mod font;
mod game;
mod hand;
mod label;
mod math;
pub mod paytable;
pub mod pixel;
mod strategy;

use crate::display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use game::Game;

static mut GAME: Game = Game::default();

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_display_width() -> usize {
    DISPLAY_WIDTH
}

#[wasm_bindgen]
pub fn get_display_height() -> usize {
    DISPLAY_HEIGHT
}

#[wasm_bindgen]
pub fn get_display() -> *const Display {
    unsafe { GAME.get_display() }
}

#[wasm_bindgen]
pub fn update_suggestions() {
    unsafe { GAME.update_suggestions() }
}

#[wasm_bindgen]
pub fn mouse_move(x: i32, y: i32) {
    unsafe {
        GAME.mouse_move(x, y);
    }
}

#[wasm_bindgen]
pub fn mouse_down(x: i32, y: i32) {
    unsafe {
        GAME.mouse_down(x, y);
    }
}

#[wasm_bindgen]
pub fn mouse_up(x: i32, y: i32) {
    unsafe {
        GAME.mouse_up(x, y);
    }
}

#[wasm_bindgen]
pub fn key_down(code: i32) {
    unsafe {
        GAME.key_down(code);
    }
}

#[wasm_bindgen]
pub fn key_up(code: i32) {
    unsafe {
        GAME.key_up(code);
    }
}

#[wasm_bindgen]
pub fn setup() {
    unsafe {
        GAME.add_bankroll(200 * 4);
        GAME.setup();
    }
}

#[wasm_bindgen]
pub fn add_bankroll(bankroll: usize) {
    unsafe {
        GAME.add_bankroll(bankroll);
    }
}

#[wasm_bindgen]
pub fn cashout() -> usize {
    unsafe { GAME.cashout() }
}
