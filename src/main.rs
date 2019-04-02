
#[macro_use]
extern crate lazy_static;

mod graphics;
mod strings;

use graphics::{*, Hangman::*};
use strings::*;

fn main() {
    let state = Splash;
    GRAPHICS.get(&state).unwrap().draw();

    loop {
        let mut line = String::new();
        println!("Type something: ");
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => println!("You typed: {}", &line),
            _ => eprintln!("Can't process your entry"),
        }

        if "quit".eq(line.trim()) {
            println!("bye.");
            break;
        }

        let(head, remainder) = get_first_unicode_char(&line);

        let head = head.to_uppercase();
        // notice head is now of type String, not &str

        println!("first char: {}\nremainder: {}", &head, remainder);
    }
}
