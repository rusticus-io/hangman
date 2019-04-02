
#[macro_use]
extern crate lazy_static;

mod graphics;

use graphics::{*, Hangman::*};

fn main() {
    let state = Splash;
    GRAPHICS.get(&state).unwrap().draw();

    let mut line = String::new();
    println!("Type something: ");
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => println!("You typed: {}", &line),
        _ => eprintln!("Can't process your entry"),
    }
}
