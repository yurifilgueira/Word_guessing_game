pub mod word;

use word::Word;

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
}
