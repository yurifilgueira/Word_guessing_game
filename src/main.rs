mod entities;

use crate::entities::Difficulty;
use entities::Game;
use crate::entities::Language;
use entities::Player;
use entities::Word;

fn main() {
    /* let mut game = Game::new("Akio", "Etsuko", Difficulty::Easy, Language::English); */
    /* let mut game = Game::new("Tetsuo", "Yaeko", Difficulty::Normal, Language::English); */
    /* let mut game = Game::new("Satoshi", "Misaki", Difficulty::Hard, Language::English); */
    /* let mut game = Game::new("Yoshio", "Katsumi", Difficulty::Easy, Language::Portuguese); */
    let mut game = Game::new("Kiyoshi", "Yasuko", Difficulty::Normal, Language::Portuguese);
    /* let mut game = Game::new("Tadashi", "Emiko", Difficulty::Hard, Language::Portuguese); */

    game.start();
}
