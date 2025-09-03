use std::{collections::{HashMap, HashSet}};
use strum_macros::{EnumIter};
use strum::IntoEnumIterator;

use crate::enums::BOARD_SIZE;

type Map<K, V> = HashMap<K, V>;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Impact {
    coords: Point,
    hit: bool
}

// Position enum enforces constraint that ships must be placed horizontally or vertically, not diagonally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Horizontal { start_x: i32, end_x: i32, y: i32 },
    Vertical { start_y: i32, end_y: i32, x: i32 },
}

impl Position {
    pub fn coordinates(&self) -> Vec<Point> {
        let mut coords = Vec::new();
        match self {
            Position::Horizontal { start_x, end_x, y } => {
                for x in *start_x..=*end_x {
                    coords.push(Point { x, y: *y });
                }
            },
            Position::Vertical { start_y, end_y, x } => {
                for y in *start_y..=*end_y {
                    coords.push(Point { x: *x, y });
                }
            },
        }
        coords
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Horizontal { start_x: 0, end_x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Ship {
    Submarine { health: [bool; 1], pos: Position },
    Destroyer { health: [bool; 2], pos: Position },
    Cruiser { health: [bool; 3], pos: Position },
    Battleship { health: [bool; 4], pos: Position },
    Carrier { health: [bool; 5], pos: Position },
}

// In Battleship, ships are placed on a board made up of a grid. Instead of implementing this as a 2D array with ships
// "filling" grid squares, maintain a mapping of ship to position ranges, given ships are 1xn rectangles, where n is the
// length and thus "HP" of a ship. This makes it easier to track which square "makes up" which ship.
// Each player has a board struct for their side.
#[derive(Debug)]
pub struct Board {
    pub occupied: [[bool; BOARD_SIZE]; BOARD_SIZE], // 2D array to track occupied squares
    pub ships: Map<Ship, Position>,
    pub impacts: HashSet<Impact>
}

#[derive(Debug)]
pub struct GameState {
    pub player1_board: Board,
    pub player2_board: Board
}

pub fn auto_place_ships(player_placements: &mut Map<Ship, Position>) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    println!("Automatically placing ships...\n=============================");

    let mut occupied = [[false; BOARD_SIZE]; BOARD_SIZE];
    for ship in Ship::iter() {
        let ship_length = match ship {
            Ship::Submarine { .. } => 1,
            Ship::Destroyer { .. } => 2,
            Ship::Cruiser { .. } => 3,
            Ship::Battleship { .. } => 4,
            Ship::Carrier { .. } => 5,
        };
        let position = crate::helpers::generate_random_position(ship_length, &mut occupied);
        println!("{:?} placed at {:?}", ship, position);
        player_placements.insert(ship, position);
    }

    return occupied;
}

pub fn place_ships(player: i32, player_placements: &mut Map<Ship, Position>) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    println!("Player {:?}, place your ships.\n=============================", player);
    let placements: HashMap<Ship, Position> = HashMap::new();

    print!("Patrol boat position: ");
    todo!();
}

pub fn set_boards(player1_placements:Map<Ship, Position>, player1_occupied: [[bool; BOARD_SIZE]; BOARD_SIZE], player2_placements:Map<Ship, Position>, player2_occupied: [[bool; BOARD_SIZE]; BOARD_SIZE]) -> GameState {
    let player1_board = Board{ occupied: player1_occupied, ships: player1_placements, impacts: HashSet::new() };
    let player2_board = Board{ occupied: player2_occupied, ships: player2_placements, impacts: HashSet::new() };

    return GameState { player1_board: player1_board, player2_board: player2_board };
}

pub fn draw_board(board: &Board) {
    let mut display_board = [['.'; BOARD_SIZE]; BOARD_SIZE];

    for (ship, pos) in &board.ships {
        for coord in pos.coordinates() {
            let display_unit = match ship {
                Ship::Submarine { .. } => 's',
                Ship::Destroyer { .. } => 'd',
                Ship::Cruiser { .. } => 'c',
                Ship::Battleship { .. } => 'B',
                Ship::Carrier { .. } => 'C',
            };
            display_board[coord.y as usize][coord.x as usize] = display_unit;
        }
    }

    println!("  0 1 2 3 4 5 6 7 8 9");
    for y in 0..BOARD_SIZE {
        print!("{} ", y);
        for x in 0..BOARD_SIZE {
            print!("{} ", display_board[y][x]);
        }
        println!();
    }
}
