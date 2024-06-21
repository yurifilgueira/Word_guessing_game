mod game;
mod player;
mod word;

pub use game::Game;
pub use player::Player;
pub use word::Word;

#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Clone)]
pub enum Language {
    English,
    Portuguese,
}