use std::{collections::{HashMap, HashSet}};

type Map<K, V> = HashMap<K, V>;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Impact {
    coords: Point,
    hit: bool
}

// Position enum enforces constraint that ships must be placed horizontally or vertically, not diagonally.
#[derive(Debug)]
pub enum Position {
    Horizontal { start_x: i32, end_x: i32 },
    Vertical { start_y: i32, end_y: i32 },
}

#[derive(Debug)]
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
struct Board {
    ships: Map<Ship, Position>,
    impacts: HashSet<Impact>
}

pub struct GameState {
    player1_board: Board,
    player2_board: Board
}

pub fn auto_place_ships(player: i32) -> HashMap<Ship, Position> {
    todo!();
}

pub fn place_ships(player: i32) -> HashMap<Ship, Position> {
    println!("Player {:?}, place your ships.\n=============================", player);
    let placements: HashMap<Ship, Position> = HashMap::new();

    print!("Patrol boat position: ");

    return placements;
}

pub fn set_boards(player1_placements:Map<Ship, Position>, player2_placements:Map<Ship, Position>) -> GameState {
    let player1_board = Board{ ships: player1_placements, impacts: HashSet::new() };
    let player2_board = Board{ ships: player2_placements, impacts: HashSet::new() };

    return GameState { player1_board: player1_board, player2_board: player2_board };
}
