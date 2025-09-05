use core::fmt;

use strum_macros::EnumIter;

pub static BOARD_SIZE: usize = 10;

#[derive(Debug)]
pub enum OpponentChoice {
    Human,
    AI
}

#[derive(Debug)]
pub enum Choice {
    Yes,
    No
}

#[derive(Debug, PartialEq, Eq, EnumIter, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn string_to_direction(input: &str) -> Option<Direction> {
        match input {
            "u" => Some(Direction::Up),
            "d" => Some(Direction::Down),
            "l" => Some(Direction::Left),
            "r" => Some(Direction::Right),
            _ => None
        }
    }

    pub fn direction_to_string(&self) -> &str {
        match self {
            Direction::Up => "u",
            Direction::Down => "d",
            Direction::Left => "l",
            Direction::Right => "r",
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}