use std::fmt::format;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

mod entities; 
use bincode::deserialize;
use clearscreen::clear;
use entities::GameConfig;

use crate::entities::{Difficulty, Game, Language, Player, Word};

fn main() {
    println!("Username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("Escolha (s para servidor, p para jogador):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim() == "s" {

        let listener = TcpListener::bind("127.0.0.1:6000").expect("Listener failed");

        if let Ok((mut stream, _addr)) = listener.accept() {
            let game_thread = thread::spawn(move || handle_client(&mut stream, &username));
            let _ = game_thread.join();

        } else {
            eprintln!("Erro ao aceitar a conexão.");
        }

    } else if choice.trim() == "p" {
        let mut client = TcpStream::connect("127.0.0.1:6000").expect("Falha ao conectar ao servidor");
        client.set_nonblocking(true).expect("Falha ao definir modo não bloqueante");
        let mut client_clone = client.try_clone().expect("Falha ao clonar o cliente");

        client.write_all(&username.trim().as_bytes()).expect("Falha ao enviar dados do jogador");
        let client_recv_thread = thread::spawn(move || {
            let mut buffer = Vec::new();

            loop {
                let mut temp_buffer = [0; 1024];
                match client.read(&mut temp_buffer) {
                    Ok(0) => break,
                    Ok(_n) => {
                        buffer.extend_from_slice(&temp_buffer);

                        clear().unwrap();
                        print_initial_message(&buffer);

                        buffer.clear();
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10)); 
                    }
                    Err(e) => {
                        eprintln!("Error reading from client: {}", e);
                        break; 
                    }
                }
            }
        });

        let client_send_thread = thread::spawn(move || {
            loop {
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Falha ao ler a entrada");

                if let Err(e) = client_clone.write_all(guess.trim().as_bytes()) {
                    eprintln!("Erro ao enviar dados: {}", e);
                    break;
                }
            }
        });

        let _ = client_recv_thread.join(); 
        let _ = client_send_thread.join(); 

    } 

    println!("Should stop");
}

fn handle_client(stream: &mut TcpStream, first_player_name: &str) {
    let mut buffer = Vec::new();

    loop {
        let mut temp_buffer = [0; 1024];
        match stream.read(&mut temp_buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                buffer.extend_from_slice(&temp_buffer[..n]);

                if let Ok(second_player_username) = String::from_utf8(buffer.clone()) { 
                    println!("Player connected - {:?}", second_player_username);
                    buffer.clear();

                    let mut game = Game::new(first_player_name.trim(), &second_player_username, Difficulty::Normal, Language::English);
                    game.start(stream);
                } 
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10)); 
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break; 
            }
        }
    }
}

fn print_initial_message(buffer: &Vec<u8>) {

    if let Ok(game_config) = deserialize::<GameConfig>(&buffer) {             

        match game_config.language {
            Language::English => println!("Welcome to word guessing game!\nLanguage: English\nDifficulty: {}\n\n{}\n\nRules:\nA {} letters long word was drawn.\nThe first player to guess correctly win the game.\nRepeat words is {}allowed.", match game_config.difficulty {
                Difficulty::Easy => "\x1b[32mEasy\x1b[0m",
                Difficulty::Normal => "\x1b[33mNormal\x1b[0m",
                Difficulty::Hard => "\x1b[31mHard\x1b[0m",
            }, game_config, get_difficulty_number(&game_config.difficulty), match game_config.difficulty {
                Difficulty::Hard => "",
                _ => "not ",
            }),
            Language::Portuguese => println!("Bem-vindo ao word guessing game!\nIdioma: Português\nDificuldade: {}\n\n{}\n\nRegras:\nUma palavra de {} caracteres foi sorteada.\nO primeiro jogador a adivinhar corretamente vence o jogo.\nRepetir palavras {}é permitido.", match game_config.difficulty {
                Difficulty::Easy => "\x1b[32mFácil\x1b[0m",
                Difficulty::Normal => "\x1b[33mNormal\x1b[0m",
                Difficulty::Hard => "\x1b[31mDifícil\x1b[0m",
            }, game_config, get_difficulty_number(&game_config.difficulty), match game_config.difficulty {
                Difficulty::Hard => "",
                _ => "não ",
            }),
        }

    }
}

fn get_difficulty_number(difficulty: &Difficulty) -> &str {
    match difficulty {
        Difficulty::Easy => "6",
        Difficulty::Normal => "7",
        Difficulty::Hard => "8",
    }
}

