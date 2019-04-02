
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Hangman {
    Splash,
}

use Hangman::*;

pub struct Graphic(String);

impl Graphic {
    pub fn new(graphic: &str) -> Graphic {
        Graphic(graphic.to_string())
    }

    pub fn draw(&self) {
        println!("{}", self.0);
    }
}

lazy_static! {
    pub static ref GRAPHICS: HashMap<Hangman, Graphic> = {
        let mut map = HashMap::new();

        map.insert(Splash, Graphic::new("\n\
        .                                             +-------+\n\
        .                                             |      \\|\n\
        .                   _______________                   |\n\
        .                -=[ H A N G M A N ]=-                |\n\
        .                   ^^^^^^^^^^^^^^^                   |\n\
        .                                                     |\n\
        .                                                    /|\\\n\
        .                                                   / | \\\n\
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
        "));

        map
    };
}
