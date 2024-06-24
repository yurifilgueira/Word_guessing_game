use core::panic;
use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;
use std::io::stdin;
/* use std::process::Command; */

use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::Difficulty;
use crate::Language;
use crate::Player;
use crate::Word;

pub struct Game {
    first_player: Player,
    second_player: Player,
    difficulty: Difficulty,
    language: Language,
    round: usize,
    turn: bool,
    wordlist: HashSet<Word>,
    selected_word: Word,
}

impl Game {
    pub fn new(first_player_name: &str, second_player_name: &str, difficulty: Difficulty, language: Language) -> Self {
        let first_player = Player::new(first_player_name);
        let second_player = Player::new(second_player_name);

        let difficulty_number = get_difficulty_number(&difficulty);
        let language_name = get_language_name(&language);

        let words = get_wordlist(&language_name, &difficulty_number);
        let random_word = get_random_word(&words);

        Self {
            first_player,
            second_player,
            difficulty: difficulty.clone(),
            language: language.clone(),
            round: 1,
            turn: true,
            wordlist: words,
            selected_word: random_word
        }
    }

    pub fn start(&mut self) {
        let mut guess = Word::new("");

        while guess != self.selected_word {
            let mut user_input: String = Default::default();
            
            match self.language {
                Language::English => {
                    if self.turn {
                        println!("\n{}º round.\nIt's {}'s turn!", self.round, self.first_player);
                    } else {
                        println!("\n{}º round.\nIt's {}'s turn!", self.round, self.second_player);
                    }
                },
                Language::Portuguese => {
                    if self.turn {
                        println!("\n{}ª rodada.\nÉ a vez de {}!", self.round, self.first_player);
                    } else {
                        println!("\n{}ª rodada.\nÉ a vez de {}!", self.round, self.second_player);
                    }
                },
            }

            stdin().read_line(&mut user_input).expect("Failed to read the word.");

            /* let mut clear_command = Command::new("clear");
            clear_command.output().expect("Failed to execute process."); */

            guess = Word::new(&user_input.trim_end());

            if guess.length() != self.selected_word.length() {
                match self.language {
                    Language::English => {
                        println!("{} is an invalid word. Your guess must have {} characters.", guess, get_difficulty_number(&self.difficulty));
                    },
                    Language::Portuguese => {
                        println!("{} é uma palavra inválida. O seu guess deve ter {} caracteres.", guess, get_difficulty_number(&self.difficulty));
                    },
                }
            } else {
                if self.wordlist.contains(&guess) {
                    if guess == self.selected_word {
                        println!("{} guess: \x1b[32m{}\x1b[0m", match self.language {
                            Language::English => "Your",
                            Language::Portuguese => "Seu",
                        }, self.selected_word);
                        
                        if self.turn {
                            println!("\n{} {}!", self.first_player, match self.language {
                                Language::English => "won the game",
                                Language::Portuguese => "venceu o jogo",
                            });
                        } else {
                            println!("\n{} {}!", self.second_player, match self.language {
                                Language::English => "won the game",
                                Language::Portuguese => "venceu o jogo",
                            });
                        }
                    } else {
                        print!("{} guess: ", match self.language {
                            Language::English => "Your",
                            Language::Portuguese => "Seu",
                        });

                        for word in &self.wordlist {
                            if *word == guess {
                                println!("{}", word);
                            }
                        }

                        self.turn = !self.turn;

                        if self.turn {
                            self.round += 1;
                        }
                    }
                    
                } else {
                    match self.language {
                        Language::English => {
                            println!("{} is an invalid word.", guess);
                        },
                        Language::Portuguese => {
                            println!("{} é uma palavra inválida.", guess);
                        },
                    }
                }
            }
        }
    }

    // Método temporário
    pub fn show_wordlist(&self) {
        for w in &self.wordlist {
            println!("{}", w);
        }
    }
    
    pub fn get_selected_word(&self) -> &str {
        &self.selected_word.get_word()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[First player: {}] [Second player: {}] [Current round: {}]", self.first_player, self.second_player, self.round)
    }
}

fn get_difficulty_number(difficulty: &Difficulty) -> &str {
    match difficulty {
        Difficulty::Easy => "6",
        Difficulty::Normal => "7",
        Difficulty::Hard => "8",
    }
}

fn get_language_name(language: &Language) -> &str {
    match language {
        Language::English => "english",
        Language::Portuguese => "portuguese",
    }
}

fn get_random_word(wordlist: &HashSet<Word>) -> Word {
    let word = &wordlist.iter().choose(&mut thread_rng());

    match word {
        Some(word) => Word::new(&word.get_word()),
        None => panic!("Erro ao sortear palavra"),
    }
}

fn get_wordlist(language_name: &str, difficulty_number: &str) -> HashSet<Word>{
    read_to_string(format!("resources/wordlist_{}_{}.txt", language_name, difficulty_number))
                .expect("Failed to read the file.")
                .lines()
                .map(|line| Word::new(line.trim()))
                .collect()
}
