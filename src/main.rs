pub mod word;

use word::build_word;

fn main() {
    let w1 = build_word(String::from("sabia"));
    let w2 = build_word(String::from("sabiá"));
    let w3 = build_word(String::from("sábia"));
    let w4 = build_word(String::from("trem"));
    let w5 = build_word(String::from("cinco"));
    let w6 = build_word(String::from("língua"));
    let w7 = build_word(String::from("TREM"));
    
    println!("{}", w1 == w2);
    println!("{}", w2 == w3);
    println!("{}", w1 == w3);
    
    println!("{}", w1 == w4);
    println!("{}", w2 == w5);
    println!("{}", w3 == w6);
    
    println!("{}", w4 == w7);
}
