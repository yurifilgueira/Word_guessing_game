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

    let g1 = Game::new("Yoshio", "Katsumi", Difficulty::Easy, Language::Portuguese);
    let mut g2 = Game::new("Kiyoshi", "Yasuko", Difficulty::Normal, Language::Portuguese);
    let g3 = Game::new("Tadashi", "Emiko", Difficulty::Hard, Language::Portuguese);
    let g4 = Game::new("Akio", "Etsuko", Difficulty::Easy, Language::English);
    let g5 = Game::new("Tetsuo", "Yaeko", Difficulty::Normal, Language::English);
    let g6 = Game::new("Satoshi", "Misaki", Difficulty::Hard, Language::English);

    println!("{}", g1);
    println!("{}", g2);
    println!("{}", g3);
    println!("{}", g4);
    println!("{}", g5);
    println!("{}", g6);

    println!("Selected word: {}", g1.get_selected_word());
    println!("Selected word: {}", g2.get_selected_word());
    println!("Selected word: {}", g3.get_selected_word());
    println!("Selected word: {}", g4.get_selected_word());
    println!("Selected word: {}", g5.get_selected_word());
    println!("Selected word: {}", g6.get_selected_word());

    println!("\x1b[32mTexto verde\x1b[0m");
    println!("\x1b[33mTexto amarelo\x1b[0m");

    g2.start();
}
