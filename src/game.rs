use std::{collections::{HashMap, HashSet}};
use strum_macros::{EnumIter};
use strum::IntoEnumIterator;

use crate::enums::BOARD_SIZE;

type Map<K, V> = HashMap<K, V>;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ship {
    pub health: Vec<bool>,
    pub pos: Position,
    pub ship_type: ShipType,
}

impl Ship {
    pub fn sunk(&self) -> bool {
        return self.health.iter().all(|&h| h == false);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum ShipType {
    Submarine { health: [bool; 1], pos: Position },
    Destroyer { health: [bool; 2], pos: Position },
    Cruiser { health: [bool; 3], pos: Position },
    Battleship { health: [bool; 4], pos: Position },
    Carrier { health: [bool; 5], pos: Position },
}



// impl Ship {
//     pub fn hit(&self) -> Point {
//         match self {
//             Ship::Submarine { health, pos } => 1,
//             Ship::Destroyer { .. } => 2,
//             Ship::Cruiser { .. } => 3,
//             Ship::Battleship { .. } => 4,
//             Ship::Carrier { .. } => 5,
//         }
//     }
// }

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

impl GameState {
    pub fn all_ships_sunk(&self, player: i32) -> bool {
        let board = if player == 1 { &self.player1_board } else { &self.player2_board };
        let mut res = true;
        for (ship, _) in &board.ships {
            res = res && ship.sunk();
        }
        return res;
    }
}

pub fn auto_place_ships(player_placements: &mut Map<Ship, Position>) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    println!("Automatically placing ships...\n=============================");

    let mut occupied = [[false; BOARD_SIZE]; BOARD_SIZE];
    for ship_type in ShipType::iter() {
        let ship_length = match ship_type {
            ShipType::Submarine { .. } => 1,
            ShipType::Destroyer { .. } => 2,
            ShipType::Cruiser { .. } => 3,
            ShipType::Battleship { .. } => 4,
            ShipType::Carrier { .. } => 5,
        };
        let position = crate::helpers::generate_random_position(ship_length, &mut occupied);
        let ship = Ship { health: vec![true; ship_length as usize], pos: position, ship_type: ship_type };
        println!("{:?} placed at {:?}", ship_type, position);
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
    let mut impact_board: [[char; 10]; 10] = [['.'; BOARD_SIZE]; BOARD_SIZE];
    for impact in &board.impacts {
        if impact.hit {
            impact_board[impact.coords.y as usize][impact.coords.x as usize] = 'X';
        } else {
            impact_board[impact.coords.y as usize][impact.coords.x as usize] = '.';
        }
    }

    let mut ship_board: [[char; 10]; 10] = [['.'; BOARD_SIZE]; BOARD_SIZE];
    for (ship, pos) in &board.ships {
        for coord in pos.coordinates() {
            let display_unit = match ship.ship_type {
                ShipType::Submarine { .. } => 's',
                ShipType::Destroyer { .. } => 'd',
                ShipType::Cruiser { .. } => 'c',
                ShipType::Battleship { .. } => 'B',
                ShipType::Carrier { .. } => 'C',
            };
            ship_board[coord.y as usize][coord.x as usize] = display_unit;
        }
    }

    println!("Impacts");
    println!("  0 1 2 3 4 5 6 7 8 9");
    for y in 0..BOARD_SIZE {
        print!("{} ", y);
        for x in 0..BOARD_SIZE {
            print!("{} ", impact_board[y][x]);
        }
        println!();
    }
    println!("---------------------");
    println!("Your ships");
    println!("  0 1 2 3 4 5 6 7 8 9");
    for y in 0..BOARD_SIZE {
        print!("{} ", y);
        for x in 0..BOARD_SIZE {
            print!("{} ", ship_board[y][x]);
        }
        println!();
    }
}
