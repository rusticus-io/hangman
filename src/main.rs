
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod graphics;
mod strings;

use graphics::{*, Hangman::*};
use strings::*;

fn main() {
    let state = Splash;
    GRAPHICS.get(&state).unwrap().draw();

    let search_string = get_search_string();

    let mut hint = prepare(&search_string);

    loop {
        println!("Search string [{}] and hint [{}]\n", search_string, hint);

        let mut line = String::new();
        println!("Type something: ");
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => println!("You typed: {}", &line),
            _ => eprintln!("Can't process your entry"),
        }

        let line = line.trim();

        if search_string.eq(line) {
            println!("you win.");
            break;
        }

        if "quit".eq(line) {
            println!("bye.");
            break;
        }

        let(head, remainder) = get_first_unicode_char(&line);

        let head = head.to_uppercase();
        // notice head is now of type String, not &str

        if find_char_in_string(&head, &search_string) {
            println!("** Character hit! **\n")
        }
        else {
            println!("** Character miss! **\n")
        }

        println!("first char: {}\nremainder: {}", &head, remainder);
    }
}
