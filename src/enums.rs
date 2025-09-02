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
