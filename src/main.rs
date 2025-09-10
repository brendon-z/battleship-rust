use std::{io::{self, Write}};

use enums::OpponentChoice;

use crate::{enums::{Choice, BOARD_SIZE}, game::{auto_place_ships, human_v_ai, human_v_human, place_ships, Board, GameState, Ship}, player::{AIPlayer, HumanPlayer, RandomSelect}};

pub mod enums;
pub mod game;
pub mod helpers;
pub mod player;

fn choose_opponent() -> OpponentChoice {
    loop {
        print!("Do you want to play against a human or a computer? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let answer = input.trim();
                match answer {
                    "human" => { return OpponentChoice::Human },
                    "computer" => {
                        return OpponentChoice::AI;
                    },
                    _ => { println!("Invalid option!") }
                }
            },
            Err(_) => {
                println!("Failed to read input, try again.");
            }
        }
        println!();
    }
}

fn decide_autoplace(player: i32) -> Choice {
    loop {
        print!("Player {}, do you want to automatically place your ships? ", player);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let answer = input.trim();
                match answer.to_lowercase().as_str() {
                    "yes" => { return Choice::Yes },
                    "no" => { return Choice::No },
                    _ => { println!("Invalid option, please answer with [yes/no]!") }
                }
            },
            Err(_) => {
                println!("Failed to read input, try again.");
            }
        }
        println!();
    }
}

fn main() {
    println!("Welcome to Battleship, implemented in Rust.");
    let opponent_choice = choose_opponent();
    println!("You have chosen to battle a {:?}.\n", opponent_choice);

    let mut player_placements: Vec<Vec<Ship>> = Vec::new();
    let mut player_occupied: Vec<[[bool; BOARD_SIZE]; BOARD_SIZE]> = Vec::new();

    let player1_placements: Vec<Ship> = Vec::new();
    player_placements.push(player1_placements);
    let player2_placements: Vec<Ship> = Vec::new();
    player_placements.push(player2_placements);

    for i in 1..=2 {
        let occupied: [[bool; BOARD_SIZE]; BOARD_SIZE];
        let auto_place = decide_autoplace(i);
        println!();

        match auto_place {
            Choice::Yes => { occupied = auto_place_ships(&mut player_placements[(i - 1) as usize]); },
            Choice::No => { occupied = place_ships(i, &mut player_placements[(i - 1) as usize]); }
        }
        player_occupied.push(occupied);
    }

    let human_player1 = HumanPlayer::new(1, Board::new(player_occupied[0], player_placements[0].clone()));

    match opponent_choice {
        OpponentChoice::Human => {
            let human_player2 = HumanPlayer::new(2, Board::new(player_occupied[1], player_placements[1].clone()));
            let mut game_state = GameState::new(Box::new(human_player1), Box::new(human_player2));
            human_v_human(&mut game_state);
        },
        OpponentChoice::AI => {
            let ai_player2 = AIPlayer::new(2, Board::new(player_occupied[1], player_placements[1].clone()), Box::new(RandomSelect));
            let mut game_state = GameState::new(Box::new(human_player1), Box::new(ai_player2));
            human_v_ai(&mut game_state);
        }
    }
}
