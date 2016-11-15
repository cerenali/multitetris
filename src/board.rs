use std::result;

use super::block::Tetromino;

pub type Result<T> = result::Result<T, String>;

pub struct Board {
    pub cells: [[u8; 10]; 15] // arbitrary width n height
}

impl Board {
    pub fn check_line_cleared(self) -> Result<u8> {
        // check if the player cleared a line
        // returns the number of the line cleared
        Ok((0))
    }

    pub fn check_game_over(self) -> Result<bool> {
        // check if the player has lost or not
        // (determined by checking if a piece has touched the top)
        Ok(true)
    }

    pub fn advance_board(self) -> Result<()> {
        // make the existing piece fall
        // handle user input ?? / keyboard controls
        Ok(())
    }

    pub fn add_new_piece(self, new_piece: Tetromino) -> Board {
        // add new piece to the board
        self
    }

    pub fn draw_board(self) -> Result<()> {
        // draw the board ?
        Ok(())
    }
}