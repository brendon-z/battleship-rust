use rand::Rng;

use crate::{enums::BOARD_SIZE, game::Position};

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

    return pos;
}