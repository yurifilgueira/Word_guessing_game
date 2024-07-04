mod game;
mod player;
mod word;
mod game_config;

pub use game::Game;
pub use player::Player;
use serde_derive::{Deserialize, Serialize};
pub use word::Word;
pub use game_config::GameConfig;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Language {
    English,
    Portuguese,
}
