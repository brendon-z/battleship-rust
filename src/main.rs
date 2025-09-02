use std::io;
use enums::OpponentChoice;

pub mod enums;

fn choose_opponent() -> OpponentChoice {
    loop {
        println!("Do you want to play against a human or a computer (NOT IMPLEMENTED YET)?");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let answer = input.trim();
                match answer {
                    "human" => {return OpponentChoice::Human},
                    "computer" => {
                        println!("Computer opponents are not implemented yet.");
                    },
                    _ => {println!("Invalid option!")}
                }
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
    println!("You have chosen to battle a {:?}", opponent_choice);
}
