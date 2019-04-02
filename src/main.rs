
#[macro_use]
extern crate lazy_static;

mod graphics;
mod strings;

use graphics::{*, Hangman::*};
use strings::*;

fn main() {
    let state = Splash;
    GRAPHICS.get(&state).unwrap().draw();

    let mut line = String::new();
    println!("Type something: ");
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => println!("You typed: {}", &line),
        _ => eprintln!("Can't process your entry"),
    }

    let(head, remainder) = get_first_unicode_char(&line);

    println!("first char: {}\nremainder: {}", head, remainder);
}
