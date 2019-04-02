
#[macro_use]
extern crate lazy_static;

mod graphics;

use graphics::{*, Hangman::*};

fn main() {
    let state = Splash;
    &GRAPHICS.get(&state).unwrap().draw();
}
