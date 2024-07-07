use std::fmt;

use serde_derive::{Deserialize, Serialize};

use super::{Difficulty, Language, Player, Word};

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub first_player: Player,
    pub second_player: Player,
    pub selected_word: Word,
    pub language: Language,
    pub difficulty: Difficulty, 
}

impl GameConfig {
    pub fn new (first_player: &String, second_player: &String, language: Language, difficulty: Difficulty, selected_word: &str) -> Self {
        
        Self {
            first_player: Player::new(first_player),
            second_player: Player::new(second_player),
            selected_word: Word::new(selected_word),
            language,
            difficulty,
        }
    }
}


impl fmt::Display for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.language {
            Language::English => write!(f, "[First player: {}] [Second player: {}]", self.first_player, self.second_player),
            Language::Portuguese => write!(f, "[Primeiro jogador: {}] [Segundo jogador: {}]", self.first_player, self.second_player),
        }
    }
}
