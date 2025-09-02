use std::{collections::HashMap, io};
use enums::OpponentChoice;

use crate::{enums::{Choice, BOARD_SIZE}, game::{auto_place_ships, place_ships, set_boards, GameState, Position, Ship}};

pub mod enums;
pub mod game;
pub mod helpers;

fn choose_opponent() -> OpponentChoice {
    loop {
        println!("Do you want to play against a human or a computer (NOT IMPLEMENTED YET)?");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let answer = input.trim();
                match answer {
                    "human" => { return OpponentChoice::Human },
                    "computer" => {
                        println!("Computer opponents are not implemented yet.");
                    },
                    _ => { println!("Invalid option!") }
                }
            },
            Err(_) => {
                println!("Failed to read input, try again.");
            }
        }
    }
}

fn decide_autoplace() -> Choice {
    loop {
        println!("Automatically place your ships?");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let answer = input.trim();
                match answer.to_lowercase().as_str() {
                    "yes" => { return Choice::Yes },
                    "no" => {
                        println!("Manual ship placements not implemented yet!");
                    },
                    _ => { println!("Invalid option, please answer with [yes/no]!") }
                }
            },
            Err(_) => {
                println!("Failed to read input, try again.");
            }
        }
    }  
}

fn main() {
    println!("Welcome to Battleship, implemented in Rust.");
    let opponent_choice = choose_opponent();
    println!("You have chosen to battle a {:?}.", opponent_choice);

    let mut player_placements: Vec<HashMap<Ship, Position>> = Vec::new();
    let mut player_occupied: Vec<[[bool; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();

    let player1_placements: HashMap<Ship, Position> = HashMap::new();
    player_placements.push(player1_placements);
    let player2_placements: HashMap<Ship, Position> = HashMap::new();
    player_placements.push(player2_placements);

    for n in 1..=2 {
        let occupied: [[bool; BOARD_SIZE]; BOARD_SIZE];
        let auto_place = decide_autoplace();
        match auto_place {
            Choice::Yes => { occupied = auto_place_ships(&mut player_placements[(n - 1) as usize]); },
            Choice::No => { occupied = place_ships(n, &mut player_placements[(n - 1) as usize]); }
        }
        player_occupied.push(occupied);
    }

    let mut game_state = set_boards(player_placements[0].clone(), player_occupied[0], player_placements[1].clone(), player_occupied[1]);
}
