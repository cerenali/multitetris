use std::io::{self, Write};

use super::board;

pub fn game_loop(board: Board) {
    // keep track of score, status, etc.
    loop {
        // read in user input (if any)
        // use that to rotate or move the piece as needed
        // on each loop (clock tick), move the pieces downward
        // update Score if necessary
        // update game status (won or lost) if necessary

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Err(err) => {
                panic!("error: {}", err);
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                // HANDLE INPUT
            }
        }
    }
    // when loop is broken, display user score / game over message
}
