use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

mod entities; 
use bincode::deserialize;
use clearscreen::clear;
use entities::{GameClient, GameConfig};

use crate::entities::{Difficulty, Game, Language, Player, Word};

fn main() {
    println!("Username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    println!("Escolha (s para servidor, p para jogador):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim() == "s" {
        let listener = TcpListener::bind("0.0.0.0:6000").expect("Listener failed");

        if let Ok((mut stream, _addr)) = listener.accept() {
            let game_thread = thread::spawn(move || handle_client(&mut stream, &username));
            let _ = game_thread.join();

        } else {
            eprintln!("Erro ao aceitar a conexão.");
        }

    } else if choice.trim() == "p" {
        println!("Digite o IP do servidor:");
        let mut server_ip = String::new();
        io::stdin().read_line(&mut server_ip).unwrap();
        println!("{}", server_ip.trim());
        let server_address = format!("{}:6000", server_ip.trim());

        let mut client = TcpStream::connect(&server_address).expect("Falha ao conectar ao servidor");
        client.set_nonblocking(true).expect("Falha ao definir modo não bloqueante");
        client.write_all(&username.trim().as_bytes()).expect("Falha ao enviar dados do jogador");

        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 1024];

        loop {
            match client.read(&mut temp_buffer) {
                Ok(0) => break,
                Ok(_n) => {
                    buffer.extend_from_slice(&temp_buffer[.._n]);

                    clear().unwrap();

                    let game_config = deserialize::<GameConfig>(&buffer).unwrap();

                    let mut game = GameClient::new(&mut client, game_config);
                    game.start();
                    buffer.clear();
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                },
                Err(e) => {
                    eprintln!("Error reading from client: {}", e);
                    break;
                }
            }
        }
    }
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
