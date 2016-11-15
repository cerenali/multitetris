use std::result;

use piston::input::*;

use super::block::Tetromino;
use super::block::blocks;

pub type Result<T> = result::Result<T, String>;

pub struct Board {
    pub cells: [[u8; 10]; 15], // arbitrary width n height
    current_piece: Tetromino    // current active Tetromino
}

impl Board {
    pub fn init_board() -> Board {
        Board {
            cells: [[0; 10]; 15],
            current_piece: Tetromino { blocks: blocks[0] }
        }
    }

    pub fn handle_key_press(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                println!("button pressed: {:?}", but);
                match but {
                    Button::Keyboard(Key::Up) => {
                        // does this do anything
                    }
                    Button::Keyboard(Key::Down) => {
                        // move current piece down faster
                    }
                    Button::Keyboard(Key::Left) => {
                        // move current piece left
                    }
                    Button::Keyboard(Key::Right) => {
                        // move current piece right
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

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

    pub fn advance_board(&mut self) -> Result<()> {
        // make the existing piece fall
        // if it's done (reached bottom), spawn a new one (get_next_piece)
        Ok(())
    }

    pub fn get_next_piece(self, new_piece: Tetromino) -> Board {
        // add new piece to the board (randomly chosen)
        self
    }

    pub fn draw_board(self) -> Result<()> {
        // draw the board ?
        Ok(())
    }
}