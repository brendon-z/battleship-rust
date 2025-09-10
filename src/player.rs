use std::any::Any;

use crate::{enums::BOARD_SIZE, game::{Board, Impact, Point}, helpers::input_coordinates};

pub trait Player {
    fn attack(&mut self, opponent: &mut Box<dyn Player>) -> Point;
    fn player_number(&self) -> i32;
    fn get_board(&self) -> &Board;
    fn get_board_mut(&mut self) -> &mut Board;
    fn hit_stats(&self) -> (i32, i32);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct HumanPlayer {
    player_number: i32,
    board: Board
}

impl HumanPlayer {
    pub fn new(player_number: i32, board: Board) -> HumanPlayer {
        HumanPlayer { player_number, board }
    }
}

impl Player for HumanPlayer {
    fn player_number(&self) -> i32 {
        self.player_number
    }

    fn attack(&mut self, opponent: &mut Box<dyn Player>) -> Point {
        loop {
            let strike_coords = input_coordinates();
            if !self.board.already_struck(strike_coords) {
                if opponent.get_board_mut().register_strike(strike_coords) {
                    println!("Hit!\n");
                    self.board.impacts.insert(Impact { coords: strike_coords, hit: true });
                } else {
                    self.board.impacts.insert(Impact { coords: strike_coords, hit: false });
                    println!("Miss!\n");
                }
                return strike_coords;
            }
            println!("You have already struck this coordinate. Try again.");
        }
    }

    fn get_board(&self) -> &Board {
        &self.board
    }

    fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    // Returns stats about hits made by a particular player (total hits, total attacks launcehd)
    fn hit_stats(&self) -> (i32, i32) {
        let mut hit_count = 0;
        for impact in &self.board.impacts {
            if impact.hit {
                hit_count += 1;
            }
        }
        return (hit_count, self.board.impacts.len() as i32);
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }

}

pub struct AIPlayer {
    pub player_number: i32,
    pub board: Board,
    pub algorithm: Box<dyn TargetingAlgorithm>,
}

impl AIPlayer {
    pub fn new(player_number: i32, board: Board, algorithm: Box<dyn TargetingAlgorithm>) -> AIPlayer {
        AIPlayer { player_number, board, algorithm }
    }
}

impl Player for AIPlayer {
    fn player_number(&self) -> i32 {
        self.player_number
    }

    fn attack(&mut self, opponent:  &mut Box<dyn Player>) -> Point {
        let strike = self.algorithm.target(&mut self.board);
        if opponent.get_board_mut().register_strike(strike) {
            self.board.impacts.insert(Impact { coords: strike, hit: true });
            println!("AI Player {} strikes at ({},{}) - Hit!\n", self.player_number, strike.x, strike.y);
        } else {
            self.board.impacts.insert(Impact { coords: strike, hit: false });
            println!("AI Player {} strikes at ({},{}) - Miss!\n", self.player_number, strike.x, strike.y);
        }
        return strike;
    }

    fn get_board(&self) -> &Board {
        &self.board
    }

    fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    // Returns stats about hits made by a particular player (total hits, total attacks launcehd)
    fn hit_stats(&self) -> (i32, i32) {
        let mut hit_count = 0;
        for impact in &self.board.impacts {
            if impact.hit {
                hit_count += 1;
            }
        }
        return (hit_count, self.board.impacts.len() as i32);
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub trait TargetingAlgorithm {
    fn target(&mut self, board: &mut Board) -> Point;
}

pub struct RandomSelect;
impl TargetingAlgorithm for RandomSelect {
    fn target(&mut self, board: &mut Board) -> Point {
        use rand::Rng;
        let mut rng = rand::rng();
        loop {
            let x = rng.random_range(0..BOARD_SIZE as i32);
            let y = rng.random_range(0..BOARD_SIZE as i32);
            let strike_coords = Point { x, y };
            if !board.already_struck(strike_coords) {
                return strike_coords
            }
        }
    }
}
