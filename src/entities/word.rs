use std::{fmt, hash::Hash};

#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialOrd)]
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

}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for c in self.word.chars() {
            // Calcula o hash de cada caractere normalizado. Depois o Hasher soma tudo para formar
            // um único código.
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
