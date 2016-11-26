use std::result;

use piston::input::*;

// use rand::{thread_rng, Rng};
use rand::Rng;

use super::block::Tetromino;
use super::block::TETROMINOES;

pub type Result<T> = result::Result<T, String>;

use super::BLOCK_SIZE;
use super::BOARD_WIDTH;
use super::BOARD_HEIGHT;

#[derive(PartialEq, Debug, Clone, Copy)]
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
            current_piece: *(::rand::thread_rng().choose(&TETROMINOES).unwrap()),
            state: GameState::Continue
        }
    }

    pub fn handle_key_press(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                println!("button pressed: {:?}", but);
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.current_piece.rotate_right();

                        // undo rotation if it was invalid...lol
                        if self.current_piece_out_of_bounds() {
                            self.current_piece.rotate_left();
                        }
                    }
                    Button::Keyboard(Key::Down) => {
                        self.current_piece.move_down();
                    }
                    Button::Keyboard(Key::Left) => {
                        // TODO: test out of bounds for other pieces
                        if self.current_piece.leftmost() - self.current_piece.block_width() >= 0.0 {
                            self.current_piece.move_left();
                        }
                    }
                    Button::Keyboard(Key::Right) => {
                        if self.current_piece.rightmost() + self.current_piece.block_width() < BOARD_WIDTH as f64  {
                            self.current_piece.move_right();
                        }
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
        if self.state != GameState::Continue {
            return Ok(self.state.clone())
        }
        // check if game is over or not
        // check top row of board and see if any of them are filled?
        for (col, colblock) in self.cells[0].iter().enumerate() {
            if *colblock == 1 {
                // TODO: clear current piece?
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
        let above_bottom: bool = self.current_piece.bottommost() < BOARD_HEIGHT as f64;//(self.current_piece.y as i64) < BOARD_HEIGHT as i64 - self.current_piece.block_height() as i64;
        println!("     > curr y: {:?}, curr height: {:?}", self.current_piece.bottommost(), self.current_piece.block_height());
        if !above_bottom {
            return false
        }

        // check if piece will intersect with a piece already on the board
        // let current_x = self.current_piece.x;
        // let current_y = self.current_piece.y;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    // let board_x: i64 = (col as i64) + (current_x as i64);
                    // let board_y: i64 = (r + current_y + 0.1) as i64; // 1 down
                    // if board_x >= BOARD_WIDTH as i64 || board_y >= BOARD_HEIGHT as i64 {
                    //     break;
                    // }
                    // println!("     > gonna check self.cells at {:?}, {:?}", board_y, board_x);
                    // if self.cells[board_y as usize][board_x as usize] == 1 {
                    //     return false;
                    // }

                    if self.cells[(self.current_piece.bottommost() + 0.1) as usize][col] == 1 {
                        return false // ???
                    }
                }
            }
        }

        above_bottom
    }

    fn current_piece_out_of_bounds(&self) -> bool {
        // let current_x = self.current_piece.x;
        // let current_y = self.current_piece.y;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    // let board_x: i64 = (col as i64) + (current_x as i64);
                    // let board_y: i64 = (r as i64) + (current_y as i64);
                    // if board_x < 0 || board_y < 0 || board_x >= BOARD_WIDTH as i64 || board_y >= BOARD_HEIGHT as i64 {
                    //     return true;
                    // }
                    if self.current_piece.leftmost() < 0.0 || self.current_piece.rightmost() >= BOARD_WIDTH as f64 || self.current_piece.bottommost() >= BOARD_HEIGHT as f64 {
                        return true
                    }
                }
            }
        }
        false
    }

    pub fn set_piece_on_board(&mut self) {
        // let current_x = self.current_piece.x;
        // let current_y = self.current_piece.y;
        let current_x = self.current_piece.leftmost();
        let current_y = self.current_piece.topmost();
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let x: i64 = (col as i64) + (current_x as i64);
                    let y: i64 = (r as i64) + (current_y as i64);
                    // TODO this is probably a dumb way to deal with it
                    // if x >= BOARD_WIDTH as i64 || y >= BOARD_HEIGHT as i64 {
                    //     break;
                    // }
                    self.cells[y as usize][x as usize] = 1;

                }
            }
        }
    }

    pub fn get_next_piece(&self) -> Tetromino {
        // add new piece to the board (randomly chosen)
        *(::rand::thread_rng().choose(&TETROMINOES).unwrap())
        // let new_block = *(::rand::thread_rng().choose(&blocks).unwrap());
        // Tetromino { 
        //     blocks: new_block,
        //     x: (BOARD_WIDTH as f64) / 2.0, // note: x is col, y is row
        //     y: 0.0 // x, y is top left corner of the block
        // }
    }

    pub fn draw_board(self) -> Result<()> {
        // draw the board ?
        Ok(())
    }
}