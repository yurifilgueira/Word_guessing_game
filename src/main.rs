mod entities;

use crate::entities::Difficulty;
use entities::Game;
use crate::entities::Language;
use entities::Player;
use entities::Word;

fn main() {
    let w1 = Word::new(String::from("sabia"));
    let w2 = Word::new(String::from("sabiá"));
    let w3 = Word::new(String::from("sábia"));
    let w4 = Word::new(String::from("trem"));
    let w5 = Word::new(String::from("cinco"));
    let w6 = Word::new(String::from("língua"));
    let w7 = Word::new(String::from("TREM"));
    
    println!("{}", w1 == w2);
    println!("{}", w2 == w3);
    println!("{}", w1 == w3);
    
    println!("{}", w1 == w4);
    println!("{}", w2 == w5);
    println!("{}", w3 == w6);
    
    println!("{}", w4 == w7);

    println!("{}", w1);

    let mut p1 = Player::new(String::from("Yato"));
    let p2 = Player::new(String::from("Mariana"));

    p1.guess_word(w1);
    p1.guess_word(w5);
    p1.guess_word(w7);

    p1.show_guessed_words();

    let g1 = Game::new(p1, p2, Difficulty::Easy, Language::Portuguese);
    println!("{}", g1);
    g1.show_wordlist();
}
