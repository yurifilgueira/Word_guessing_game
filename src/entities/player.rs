use std::collections::BinaryHeap;
use std::fmt;

use crate::Word;

pub struct Player {
    name: String,
    guesses: BinaryHeap<Word>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            guesses: BinaryHeap::new(),
        }
    }

    pub fn guess_word(&mut self, word: Word) {
        self.guesses.push(word);
    }

    pub fn show_guessed_words(&self) {
        print!("Guessed words of {}: ", self.name);

        for w in &self.guesses {
            print!("{}, ", w);
        }

        println!("");
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
