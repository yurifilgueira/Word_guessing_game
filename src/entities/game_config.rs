use std::fmt;

use serde_derive::{Deserialize, Serialize};

use super::{Difficulty, Language};

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub first_player_name: String,
    pub second_player_name: String,
    pub language: Language,
    pub difficulty: Difficulty,
}

impl GameConfig {
    pub fn new (first_player_name: &String, second_player_name: &String, language: Language, difficulty: Difficulty) -> Self {
        
        Self {
            first_player_name: first_player_name.to_string(),
            second_player_name: second_player_name.to_string(),
            language: language,
            difficulty: difficulty,
        }
    }
}


impl fmt::Display for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.language {
            Language::English => write!(f, "[First player: {}] [Second player: {}]", self.first_player_name, self.second_player_name),
            Language::Portuguese => write!(f, "[Primeiro jogador: {}] [Segundo jogador: {}]", self.first_player_name, self.second_player_name),
        }
    }
}
