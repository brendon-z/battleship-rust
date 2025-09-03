use std::{collections::HashMap, io};
use enums::OpponentChoice;

use crate::{enums::{Choice, BOARD_SIZE}, game::{auto_place_ships, draw_board, place_ships, set_boards, GameState, Point, Position, Ship}};

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

fn strike_coordinates() -> Point {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let coords: Vec<&str> = input.trim().split(',').collect();
                if coords.len() != 2 {
                    println!("Invalid input, please enter coordinates in the format x,y");
                    continue;
                }
                let x = match coords[0].parse::<i32>() {
                    Ok(num) if num >= 0 && num < BOARD_SIZE as i32 => num,
                    _ => {
                        println!("Invalid x coordinate, please enter a number between 0 and {}", BOARD_SIZE - 1);
                        continue;
                    }
                };
                let y = match coords[1].parse::<i32>() {
                    Ok(num) if num >= 0 && num < BOARD_SIZE as i32 => num,
                    _ => {
                        println!("Invalid y coordinate, please enter a number between 0 and {}", BOARD_SIZE - 1);
                        continue;
                    }
                };
                return Point { x, y };
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

    for i in 1..=2 {
        let occupied: [[bool; BOARD_SIZE]; BOARD_SIZE];
        let auto_place = decide_autoplace();
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
                draw_board(&game_state.player1_board);
            } else {
                draw_board(&game_state.player2_board);
            }
            println!("Enter coordinates to strike (x,y): ");
            let strike_coords = strike_coordinates();
        }
        println!();
    }

}
