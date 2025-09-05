use std::io::{self, Write};

use enums::OpponentChoice;

use crate::{enums::{Choice, BOARD_SIZE}, game::{auto_place_ships, place_ships, set_boards, Ship}, helpers::input_coordinates};

pub mod enums;
pub mod game;
pub mod helpers;

fn choose_opponent() -> OpponentChoice {
    loop {
        print!("Do you want to play against a human or a computer (NOT IMPLEMENTED YET)? ");
        io::stdout().flush().unwrap();

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

    let mut game_state = set_boards(player_placements[0].clone(), player_occupied[0], player_placements[1].clone(), player_occupied[1]);

    while !game_state.all_ships_sunk(1) && !game_state.all_ships_sunk(2) {
        for i in 1..=2 {
            println!("Player {}, it's your turn!", i);
            println!("==========================");
            if i == 1 {
                game_state.draw_board(1);
            } else {
                game_state.draw_board(2);
            }

            loop {
                let strike_coords = input_coordinates();

                if !game_state.already_struck(i, strike_coords) {
                    game_state.register_strike(i, strike_coords);
                    break;
                }
                println!("You have already struck this coordinate. Try again.");
            }
            println!();
        }
        println!();
    }

    let winning_player;
    let hit_stats;
    if game_state.all_ships_sunk(1) {
        winning_player = 2;
        println!("Player 2 wins!");
        hit_stats = game_state.player2_board.hit_stats();
    } else {
        winning_player = 1;
        println!("Player 1 wins!");
        hit_stats = game_state.player1_board.hit_stats();
    }

    println!("Player {}'s hit statistics:", winning_player);
    println!("{} successful hits out of {} total strikes made, with - a {}% hit rate", hit_stats.0, hit_stats.1, hit_stats.0 as f32 / hit_stats.1 as f32 * 100.0);
}
