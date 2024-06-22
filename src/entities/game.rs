use core::panic;
use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;

use rand::seq::IteratorRandom;
use rand::thread_rng;

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
    selected_word: Word,
}

impl Game {
    pub fn new(first_player_name: &str, second_player_name: &str, difficulty: Difficulty, language: Language) -> Self {
        let first_player = Player::new(first_player_name);
        let second_player = Player::new(second_player_name);

        let difficulty_number = get_difficulty_number(&difficulty);
        let language_name = get_language_name(&language);

        let words = get_wordlist(&language_name, &difficulty_number);
        let random_word = get_random_word(&words);

        Self {
            first_player,
            second_player,
            difficulty: difficulty.clone(),
            language: language.clone(),
            round: 0,
            wordlist: words,
            selected_word: random_word
        }
    }

    // Método temporário
    pub fn show_wordlist(&self) {
        for w in &self.wordlist {
            println!("{}", w);
        }
    }
    
    pub fn get_selected_word(&self) -> &str {
        &self.selected_word.get_word()
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

fn get_random_word(wordlist: &HashSet<Word>) -> Word {
    let word = &wordlist.iter().choose(&mut thread_rng());

    match word {
        Some(word) => Word::new(&word.get_word()),
        None => panic!("Erro ao sortear palavra"),
    }
}

fn get_wordlist(language_name: &str, difficulty_number: &str) -> HashSet<Word>{
    read_to_string(format!("resources/wordlist_{}_{}.txt", language_name, difficulty_number))
                .expect("Erro ao ler o arquivo")
                .lines()
                .map(|line| Word::new(line.trim()))
                .collect()
}
