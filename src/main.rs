mod entities;

use crate::entities::Difficulty;
use entities::Game;
use crate::entities::Language;
use entities::Player;
use entities::Word;

fn main() {
    let w1 = Word::new("sabia");
    let w2 = Word::new("sabiá");
    let w3 = Word::new("sábia");
    let w4 = Word::new("trem");
    let w5 = Word::new("cinco");
    let w6 = Word::new("língua");
    let w7 = Word::new("TREM");
    
    println!("{}", w1 == w2);
    println!("{}", w2 == w3);
    println!("{}", w1 == w3);
    
    println!("{}", w1 == w4);
    println!("{}", w2 == w5);
    println!("{}", w3 == w6);
    
    println!("{}", w4 == w7);

    println!("{}", w1);

    /* let mut game = Game::new("Yoshio", "Katsumi", Difficulty::Easy, Language::Portuguese); */
    let mut game = Game::new("Kiyoshi", "Yasuko", Difficulty::Normal, Language::Portuguese);
    /* let mut game = Game::new("Tadashi", "Emiko", Difficulty::Hard, Language::Portuguese); */
    /* let mut game = Game::new("Akio", "Etsuko", Difficulty::Easy, Language::English); */
    /* let mut game = Game::new("Tetsuo", "Yaeko", Difficulty::Normal, Language::English); */
    /* let mut game = Game::new("Satoshi", "Misaki", Difficulty::Hard, Language::English); */

    println!("Selected word: {}", game.get_selected_word());

    game.start();
}
