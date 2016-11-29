use std::result;

use piston::input::*;

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
    pub current_piece: Tetromino, // current active Tetromino
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
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.current_piece.rotate_right();

                        // undo rotation if it was invalid...lol
                        if self.current_piece_out_of_bounds() {
                            self.current_piece.rotate_left();
                        }
                    }
                    Button::Keyboard(Key::Down) => {
                        // TODO: drop piece to bottom?
                    }
                    Button::Keyboard(Key::Left) => {
                        if self.current_piece.leftmost() - 1.0 >= 0.0 {
                            self.current_piece.move_left();
                        }
                    }
                    Button::Keyboard(Key::Right) => {
                        if self.current_piece.rightmost() + 1.0 < BOARD_WIDTH as f64  {
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
        // TODO check if the player cleared a line
        // TODO returns the number of the line cleared?
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
        let above_bottom: bool = self.current_piece.bottommost() + 1.0 < BOARD_HEIGHT as f64;
        if !above_bottom {
            return false
        }

        // check if piece will intersect with a piece already on the board
        let current_x = self.current_piece.x_offset;
        let current_y = self.current_piece.y_offset;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let board_x: i64 = (col as i64) + (current_x as i64);
                    let board_y: i64 = ((r as f64) + current_y + 1.0) as i64; // 1 down
                    if self.cells[board_y as usize][board_x as usize] == 1 {
                        return false;
                    }
                }
            }
        }

        above_bottom
    }

    fn current_piece_out_of_bounds(&self) -> bool {
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    if self.current_piece.leftmost() < 0.0 || self.current_piece.rightmost() >= BOARD_WIDTH as f64 || self.current_piece.bottommost() >= BOARD_HEIGHT as f64 {
                        return true
                    }
                }
            }
        }
        false
    }

    pub fn set_piece_on_board(&mut self) {
        let current_x = self.current_piece.x_offset;
        let current_y = self.current_piece.y_offset;
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
        *(::rand::thread_rng().choose(&TETROMINOES).unwrap())
    }

    pub fn draw_board(self) -> Result<()> {
        // draw the board ?
        Ok(())
    }
}