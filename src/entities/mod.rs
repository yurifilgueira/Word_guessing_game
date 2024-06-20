mod game;
mod player;
mod word;

pub use game::Game;
pub use player::Player;
pub use word::Word;

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub enum Language {
    English,
    Portuguese,
}