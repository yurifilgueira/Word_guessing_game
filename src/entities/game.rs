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
        let difficulty_number = get_difficulty_number(&difficulty);
        let language_name = get_language_name(&language);

        Self {
            first_player,
            second_player,
            difficulty: difficulty.clone(),
            language: language.clone(),
            round: 0,
            wordlist: read_to_string(format!("resources/wordlist_{}_{}.txt", language_name, difficulty_number))
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

fn get_difficulty_number(difficulty: &Difficulty) -> &str {
    match difficulty {
        Difficulty::Easy => "6",
        Difficulty::Normal => "7",
        Difficulty::Hard => "8",
    }
}

fn get_language_name(language: &Language) -> &str {
    match language {
        Language::English => "english",
        Language::Portuguese => "portuguese",
    }
}
