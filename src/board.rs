use std::result;

use piston::input::*;

// use rand::{thread_rng, Rng};
use rand::Rng;

use super::block::Tetromino;
use super::block::blocks;

pub type Result<T> = result::Result<T, String>;

use super::BLOCK_SIZE;
use super::BOARD_WIDTH;
use super::BOARD_HEIGHT;

#[derive(PartialEq, Debug)]
pub enum GameState {
    Continue,
    PlayerLost,
    PlayerQuit
}

pub struct Board {
    pub cells: [[u8; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
    pub current_piece: Tetromino,    // current active Tetromino
    pub state: GameState
}

impl Board {
    pub fn init_board() -> Board {
        Board {
            cells: [[0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
            current_piece: Tetromino { 
                blocks: blocks[0], // TODO randomly select one
                x: (BOARD_WIDTH as f64) / 2.0, // note: x is col, y is row
                y: 0.0 // x, y is top left corner of the block
            },
            state: GameState::Continue
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

    pub fn advance_board(&mut self) -> Result<GameState> {
        // check if game is over or not
        // check top row of board and see if any of them are filled?
        for (col, colblock) in self.cells[0].iter().enumerate() {
            if *colblock == 1 {
                return Ok(GameState::PlayerLost)
            }
        }

        // make the existing piece fall
        if self.can_move_current_piece_down() {
            self.current_piece.move_down();
        } else {
            // add piece to board cells
            self.set_piece_on_board();

            // get new piece
            self.current_piece = self.get_next_piece();
        }

        Ok(GameState::Continue)
    }

    fn can_move_current_piece_down(&self) -> bool {
        let has_reached_bottom: bool = self.current_piece.y < BOARD_HEIGHT as f64 - self.current_piece.block_height();
        if !has_reached_bottom {
            return false
        }

        // check if piece will intersect with a piece already on the board
        let current_x = self.current_piece.x;
        let current_y = self.current_piece.y;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let board_x: i64 = (col as i64) + (current_x as i64);
                    let board_y: i64 = (r as i64) + (current_y as i64) + 1; // 1 down
                    if self.cells[board_y as usize][board_x as usize] == 1 {
                        return false;
                    }
                }
            }
        }

        has_reached_bottom
    }

    pub fn set_piece_on_board(&mut self) {
        let current_x = self.current_piece.x;
        let current_y = self.current_piece.y;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let x: i64 = (col as i64) + (current_x as i64);
                    let y: i64 = (r as i64) + (current_y as i64);
                    self.cells[y as usize][x as usize] = 1;
                }
            }
        }
    }

    pub fn get_next_piece(&self) -> Tetromino {
        // add new piece to the board (randomly chosen)
        let new_block = *(::rand::thread_rng().choose(&blocks).unwrap());
        Tetromino { 
            blocks: new_block,
            x: (BOARD_WIDTH as f64) / 2.0, // note: x is col, y is row
            y: 0.0 // x, y is top left corner of the block
        }
    }

    pub fn draw_board(self) -> Result<()> {
        // draw the board ?
        Ok(())
    }
}