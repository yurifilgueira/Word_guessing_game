use std::collections::BinaryHeap;
use std::fmt;

use crate::Word;

pub struct Player {
    name: String,
    guesses: BinaryHeap<Word>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            guesses: BinaryHeap::new(),
        }
    }

    pub fn guess_word(&mut self, word: Word) {
        self.guesses.push(word);
    }

    pub fn has_guessed_word(&self, word: &Word) -> bool {
        for w in &self.guesses {
            if w == word {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
