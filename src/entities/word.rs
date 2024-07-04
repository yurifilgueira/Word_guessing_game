use std::{fmt, hash::Hash};
use std::collections::HashMap;
extern crate unicode_normalization;
use serde_derive::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

#[derive(Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Word {
    word: String,
}

impl Word {
    pub fn new(word: &str) -> Self {
        Self {word: String::from(word.to_uppercase())}
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn length(&self) -> usize {
        self.word.chars().count()
    }

    pub fn show_status(&self, selected_word: &Word) {
        let mut letter_counts: HashMap<char, usize> = HashMap::new();

        for i in 0..selected_word.word.chars().count() {
            if self.word.contains(normalize_char(self.word.chars().nth(i).unwrap())) && normalize_char(self.word.chars().nth(i).unwrap()) != normalize_char(selected_word.word.chars().nth(i).unwrap()) {
                *letter_counts.entry(normalize_char(selected_word.word.chars().nth(i).unwrap())).or_insert(0) += 1;
            }
        }

        let mut counted_letters: HashMap<char, usize> = HashMap::new();

        for i in 0..self.word.chars().count() {
            if normalize_char(self.word.chars().nth(i).unwrap()) == normalize_char(selected_word.word.chars().nth(i).unwrap()) {
                print!("\x1b[32m{}\x1b[0m", self.word.chars().nth(i).unwrap());
            } else if selected_word.word.nfc().collect::<String>().contains(normalize_char(self.word.chars().nth(i).unwrap())) && *counted_letters.entry(normalize_char(self.word.chars().nth(i).unwrap())).or_insert(0) < *letter_counts.entry(normalize_char(self.word.chars().nth(i).unwrap())).or_insert(0) {
                print!("\x1b[33m{}\x1b[0m", self.word.chars().nth(i).unwrap());
                *counted_letters.entry(normalize_char(self.word.chars().nth(i).unwrap())).or_insert(0) += 1;
            } else {
                print!("{}", self.word.chars().nth(i).unwrap());
            }
        }
        println!();
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for c in self.word.chars() {
            normalize_char(c).hash(state);
        }
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Word) -> bool {
        if self.word.chars().count() != other.word.chars().count() {
            return false;
        }

        for (l1, l2) in self.word.chars().zip(other.word.chars()) {

            if !letters_are_equal(l1, l2) {
                return false;
            }
        }

        true
    }
}

fn letters_are_equal(first_letter: char, second_letter: char) -> bool {
    normalize_char(first_letter) == normalize_char(second_letter)
}

fn normalize_char(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' => 'a',
        'Á' | 'À' | 'Â' | 'Ã' => 'A',
        'é' | 'è' | 'ê' => 'e',
        'É' | 'È' | 'Ê' => 'E',
        'í' | 'ì' | 'î' => 'i',
        'Í' | 'Ì' | 'Î' => 'I',
        'ó' | 'ò' | 'ô' | 'õ' => 'o',
        'Ó' | 'Ò' | 'Ô' | 'Õ' => 'O',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'Ú' | 'Ù' | 'Û' | 'Ü' => 'U',
        'ç' => 'c',
        'Ç' => 'C',
        _ => c,
    }
}
