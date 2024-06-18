use std::fmt;

#[derive(Debug)]
pub struct Word {
    word: String,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Word) -> bool {
        if self.word.chars().count() != other.word.chars().count() {
            return false;
        }
        
        for (l1, l2) in self.word.to_lowercase().chars().zip(other.word.to_lowercase().chars()) {
            if !letters_are_equal(l1, l2) {
                return false;
            }
        }
        
        true
    }
}

impl Word {
    pub fn new(word: String) -> Self {
        Self {word}
    }
}

fn letters_are_equal(first_letter: char, second_letter: char) -> bool {
   normalize_char(first_letter) == normalize_char(second_letter)
}

fn normalize_char(c: char) -> char {
    match c {
        'á' | 'à' | 'â' | 'ã' => 'a',
        'é' | 'è' | 'ê' => 'e',
        'í' | 'ì' | 'î' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' => 'o',
        'ú' | 'ù' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        _ => c,
    }
}
