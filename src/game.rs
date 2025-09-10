use core::fmt;
use std::{collections::{HashSet}};
use strum_macros::{EnumIter};
use strum::IntoEnumIterator;

use crate::{enums::BOARD_SIZE, game, helpers::{check_position_valid, input_coordinates, input_ship_positon}, player::{AIPlayer, Player, RandomSelect, TargetingAlgorithm}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Impact {
    pub coords: Point,
    pub hit: bool
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
    // Returns true if the ship has no health left
    pub fn sunk(&self) -> bool {
        return self.health.iter().all(|&h| h == false);
    }

    // Registers a hit on the ship at the given impact point, if it hits. Returns true if hit, false otherwise.
    pub fn hit(&mut self, impact_point: &Point) -> bool {
        let coords = self.pos.coordinates();
        for (i, coord) in coords.iter().enumerate() {
            if coord.x == impact_point.x && coord.y == impact_point.y {
                self.health[i] = false;
                return true;
            }
        }
        return false;
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

impl fmt::Display for ShipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipType::Submarine { .. } => write!(f, "Submarine"),
            ShipType::Destroyer { .. } => write!(f, "Destroyer"),
            ShipType::Cruiser { .. } => write!(f, "Cruiser"),
            ShipType::Battleship { .. } => write!(f, "Battleship"),
            ShipType::Carrier { .. } => write!(f, "Carrier"),
        }
    }
}


// In Battleship, ships are placed on a board made up of a grid. Instead of implementing this as just a 2D array with ships
// "filling" grid squares, maintain a mapping of ship to position ranges, given ships are 1xn rectangles, where n is the
// length and thus "HP" of a ship. This makes it easier to track which square "makes up" which ship.
// Each player has a board struct for their side.
#[derive(Debug)]
pub struct Board {
    pub occupied: [[bool; BOARD_SIZE]; BOARD_SIZE], // 2D array to track occupied squares
    pub ships: Vec<Ship>,
    pub impacts: HashSet<Impact>
}
impl Board {
    pub fn new(occupied: [[bool; BOARD_SIZE]; BOARD_SIZE], ships: Vec<Ship>) -> Board {
        Board { occupied: occupied, ships, impacts: HashSet::new() }
    }

    pub fn register_strike(&mut self, strike_coords: Point) -> bool {
        let mut hit = false;
        for ship in self.ships.iter_mut() {
            if ship.hit(&strike_coords) {
                hit = true;
                break;
            }
        }
        return hit;
    }

    pub fn all_ships_sunk(&self) -> bool {
        return self.ships.iter().all(|ship| ship.sunk());
    }
    
    pub fn already_struck(&self, strike_coords: Point) -> bool {
        return self.impacts.iter().any(|impact| impact.coords.x == strike_coords.x && impact.coords.y == strike_coords.y);
    }

    pub fn draw_board(&self) {
        let mut impact_board: [[char; 10]; 10] = [['.'; BOARD_SIZE]; BOARD_SIZE];
        for impact in &self.impacts {
            if impact.hit {
                impact_board[impact.coords.y as usize][impact.coords.x as usize] = 'X';
            } else {
                impact_board[impact.coords.y as usize][impact.coords.x as usize] = '.';
            }
        }

        let mut ship_board: [[char; 10]; 10] = [['.'; BOARD_SIZE]; BOARD_SIZE];
        for ship in &self.ships {
            for (coord, index) in ship.pos.coordinates().iter().zip(0..) {
                let display_unit;
                if !ship.health[index] {
                    display_unit = '†';
                } else {
                    display_unit = match ship.ship_type {
                        ShipType::Submarine { .. } => 's',
                        ShipType::Destroyer { .. } => 'd',
                        ShipType::Cruiser { .. } => 'c',
                        ShipType::Battleship { .. } => 'B',
                        ShipType::Carrier { .. } => 'C',
                    };
                }
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
}

pub struct GameState {
    player1: Box<dyn Player>,
    player2: Box<dyn Player>
}

impl GameState {
    pub fn new(player1: Box<dyn Player>, player2: Box<dyn Player>) -> GameState {
        GameState { player1, player2 }
    }

    pub fn game_over(&self) -> bool {
        return self.player1.get_board().all_ships_sunk() || self.player2.get_board().all_ships_sunk();
    }
}

pub fn auto_place_ships(player_placements: &mut Vec<Ship>) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
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
        player_placements.push(ship);
    }

    return occupied;
}

pub fn place_ships(player: i32, player_placements: &mut Vec<Ship>) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    println!("Player {:?}, place your ships.\n=============================", player);

    let mut occupied = [[false; BOARD_SIZE]; BOARD_SIZE];
    for ship_type in ShipType::iter() {
        let ship_length = match ship_type {
            ShipType::Submarine { .. } => 1,
            ShipType::Destroyer { .. } => 2,
            ShipType::Cruiser { .. } => 3,
            ShipType::Battleship { .. } => 4,
            ShipType::Carrier { .. } => 5,
        };

        let mut position;
        loop {
            println!("Place your {} (length {})", ship_type, ship_length);
            position = input_ship_positon(ship_length);
            if check_position_valid(&position, &occupied) {
                break;
            }
            println!("Ship overlaps another, please choose another position.");
        }

        for coord in position.coordinates() {
            occupied[coord.y as usize][coord.x as usize] = true;
        }

        let ship = Ship { health: vec![true; ship_length as usize], pos: position, ship_type: ship_type };
        player_placements.push(ship);
        println!();
    }

    return occupied;
}

pub fn human_v_human(game_state: &mut GameState) {
    while !game_state.game_over() {
        for i in 1..=2 {
            println!("Player {}, it's your turn!", i);
            println!("==========================");
            let player;
            let opponent;
            if i == 1 {
                player = &mut game_state.player1;
                opponent = &mut game_state.player2;
            } else {
                player = &mut game_state.player2;
                opponent = &mut game_state.player1;
            }
            player.get_board().draw_board();
            player.attack(opponent);
        }
    }

    let winning_player;
    let hit_stats;
    if game_state.player1.get_board().all_ships_sunk() {
        winning_player = 2;
        println!("Player 2 wins!");
        hit_stats = game_state.player2.hit_stats();
    } else {
        winning_player = 1;
        println!("Player 1 wins!");
        hit_stats = game_state.player1.hit_stats();
    }

    println!("Player {}'s hit statistics:", winning_player);
    println!("{} successful hits out of {} total strikes made, - a {}% hit rate", hit_stats.0, hit_stats.1, hit_stats.0 as f32 / hit_stats.1 as f32 * 100.0);
}

pub fn human_v_ai(game_state: &mut GameState) {
    while !game_state.game_over() {
        for i in 1..=2 {
            let player;
            let opponent;
            if i == 1 {
                player = &mut game_state.player1;
                opponent = &mut game_state.player2;
            } else {
                player = &mut game_state.player2;
                opponent = &mut game_state.player1;
            }
            
            if player.as_any().is::<AIPlayer>() {
                println!("AI is making its move...");
            } else {
                println!("Player {}, it's your turn!", i);
                println!("==========================");
                player.get_board().draw_board();
            }
            player.attack(opponent);
        }
    }

    let winning_player;
    let hit_stats;
    if game_state.player1.get_board().all_ships_sunk() {
        winning_player = 2;
        println!("The AI wins!");
        hit_stats = game_state.player2.hit_stats();
    } else {
        winning_player = 1;
        println!("You win!");
        hit_stats = game_state.player1.hit_stats();
    }

    println!("Player {}'s hit statistics:", winning_player);
    println!("{} successful hits out of {} total strikes made, - a {}% hit rate", hit_stats.0, hit_stats.1, hit_stats.0 as f32 / hit_stats.1 as f32 * 100.0);
}
