
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod graphics;
mod strings;

use graphics::{*, Hangman::*};
use strings::*;

fn main() {
    let mut state = Splash;

    let search_string = get_search_string();

    let mut hint = prepare(&search_string);

    let mut character_matches = "".to_string();

    let mut false_guesses = 0;

    loop {
        Graphic::display(&state, &hint);

        println!("Search string [{}] and hint [{}]\n", search_string, hint);

        let mut line = String::new();
        println!("Type something: ");
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => (),
            _ => {
                eprintln!("Can't process your entry");
                continue;
            },
        }

        let line = line.trim();

        if "quit".eq(line) {
            state = Hangman::Bye(false_guesses);
            break;
        }

        let(head, _) = get_first_unicode_char(&line);

        let head = head.to_uppercase();
        // notice head is now of type String, not &str

        if find_char_in_string(&head, &search_string) {
            if find_char_in_string(&head, &character_matches) {
                println!("** G R E A T **\n");
                false_guesses += 1;
            }
            else {
                character_matches.push_str(&head);
                hint = create_hint(&search_string, &character_matches);
            }
        }
        else {
            println!("** M I S S **\n");
            false_guesses += 1;
        }

        state = Hangman::Error(false_guesses);

        if search_string.eq(&hint) {
            state = Hangman::Win(false_guesses);
            break;
        }

        if false_guesses == 7 {
            state = Hangman::Loose;
            break;
        }
    }

    Graphic::display(&state, &hint);

    println!("\nSearched word is {}", search_string);
    println!("\n____________________________________________________________\n");
}
