use std::{collections::HashSet, io::{self, Write}};

use rand::Rng;
use strum::IntoEnumIterator;

use crate::{enums::{Direction, BOARD_SIZE}, game::{Point, Position}};

pub fn input_coordinates() -> Point {
    print!("Enter coordinates (x,y): ");
    io::stdout().flush().unwrap();
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

pub fn check_position_valid(pos: &Position, occupied: &[[bool; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    match pos {
        Position::Horizontal { start_x, end_x, y } => {
            if *y < 0 || *y >= BOARD_SIZE as i32 || *start_x < 0 || *end_x >= BOARD_SIZE as i32 || *start_x > *end_x {
                return false;
            }
            for x in *start_x..=*end_x {
                if occupied[*y as usize][x as usize] {
                    return false;
                }
            }
        },
        Position::Vertical { start_y, end_y, x } => {
            if *x < 0 || *x >= BOARD_SIZE as i32 || *start_y < 0 || *end_y >= BOARD_SIZE as i32 || *start_y > *end_y {
                return false;
            }
            for y in *start_y..=*end_y {
                if occupied[y as usize][*x as usize] {
                    return false;
                }
            }
        },
    }
    true
}

pub fn generate_random_position(ship_length: i32, occupied: &mut [[bool; BOARD_SIZE]; BOARD_SIZE]) -> Position {
    let mut rng = rand::rng();

    let mut pos;
    let vertical = rng.random_bool(0.5);
    if vertical {
        let start_y = rng.random_range(0..(BOARD_SIZE as i32 - ship_length + 1));
        let x = rng.random_range(0..BOARD_SIZE) as i32;
        pos = Position::Vertical { start_y, end_y: start_y + ship_length - 1, x };
    } else {
        let start_x = rng.random_range(0..(BOARD_SIZE as i32 - ship_length + 1));
        let y = rng.random_range(0..BOARD_SIZE) as i32;
        pos =  Position::Horizontal { start_x, end_x: start_x + ship_length - 1, y };
    }

    while check_position_valid(&pos, occupied) == false {
        let vertical = rng.random_bool(0.5);
        if vertical {
            let start_y = rng.random_range(0..(BOARD_SIZE as i32 - ship_length + 1));
            let x = rng.random_range(0..BOARD_SIZE) as i32;
            pos = Position::Vertical { start_y, end_y: start_y + ship_length - 1, x };
        } else {
            let start_x = rng.random_range(0..(BOARD_SIZE as i32 - ship_length + 1));
            let y = rng.random_range(0..BOARD_SIZE) as i32;
            pos =  Position::Horizontal { start_x, end_x: start_x + ship_length - 1, y };
        }
    }

    match pos {
        Position::Horizontal { start_x, end_x, y } => {
            for x in start_x..=end_x {
                occupied[y as usize][x as usize] = true;
            }
        },
        Position::Vertical { start_y, end_y, x } => {
            for y in start_y..=end_y {
                occupied[y as usize][x as usize] = true;
            }
        },
    }

    return pos;
}

pub fn input_ship_positon(ship_length: i32) -> Position {
        let start_pos = input_coordinates();
        let input_direction;
        let mut allowed_directions = HashSet::new();

        for direction in Direction::iter() {
            if (direction == Direction::Down && start_pos.y + (ship_length - 1) < BOARD_SIZE as i32) 
            || (direction == Direction::Up && start_pos.y - (ship_length - 1) >= 0)
            || (direction == Direction::Left && start_pos.x - (ship_length - 1) >= 0)
            || (direction == Direction::Right && start_pos.x + (ship_length - 1) < BOARD_SIZE as i32) {
                allowed_directions.insert(direction);
            }
        }

        let position;
        loop {
            print!("Enter orientation (");
            for direction in &allowed_directions {
                print!("{} ", direction.direction_to_string());
            }
            print!("):");

            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let answer = input.trim().to_lowercase();
                match Direction::string_to_direction(&answer) {
                    Some(dir) => {
                        if allowed_directions.contains(&dir) {
                            input_direction = dir;
                            position = match input_direction {
                                Direction::Down => Position::Vertical { start_y: start_pos.y, end_y: start_pos.y + (ship_length - 1), x: start_pos.x },
                                Direction::Up => Position::Vertical { start_y: start_pos.y - (ship_length - 1), end_y: start_pos.y, x: start_pos.x },
                                Direction::Left => Position::Horizontal { start_x: start_pos.x - (ship_length - 1), end_x: start_pos.x, y: start_pos.y },
                                Direction::Right => Position::Horizontal { start_x: start_pos.x, end_x: start_pos.x + (ship_length - 1), y: start_pos.y },
                            };

                            break;
                        } else {
                            println!("Direction not allowed from this position, please choose another.");
                        }
                    },
                    None => { println!("Invalid option, please answer with [l, r, u or d]!") }
                }
            }
        }
        return position;
}