use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;

use crate::Difficulty;
use crate::Language;
use crate::Player;
use crate::Word;

pub struct Game {
    first_player: Player,
    second_player: Player,
    difficulty: Difficulty,
    language: Language,
    round: usize,
    wordlist: HashSet<Word>,
}

impl Game {
    pub fn new(first_player: Player, second_player: Player, difficulty: Difficulty, language: Language) -> Self {
        Self {
            first_player,
            second_player,
            difficulty,
            language,
            round: 0,
            wordlist: read_to_string(format!("resources/wordlist_{}_{}.txt", get_language(), get_difficulty_number()))
                .expect("Erro ao ler o arquivo")
                .lines()
                .map(|line| Word::new(line.trim().to_string()))
                .collect(),
        }
    }

    // Método temporário
    pub fn show_wordlist(&self) {
        for w in &self.wordlist {
            println!("{}", w);
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[First player: {}] [Second player: {}] [Current round: {}]", self.first_player, self.second_player, self.round)
    }
}

fn get_difficulty_number() -> String {
    String::from("6")
}

fn get_language() -> String {
    String::from("english")
}
