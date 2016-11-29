use std::result;

use piston::input::*;

use rand::Rng;

use super::block::Tetromino;
use super::block::TETROMINOS;

pub type Result<T> = result::Result<T, String>;

use super::BLOCK_SIZE;
use super::BOARD_WIDTH;
use super::BOARD_HEIGHT;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameState {
    Playing,
    Paused,
    Over // donezo. player lost or quit
}

pub struct Board {
    pub cells: [[u8; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
    pub current_piece: Tetromino, // current active Tetromino
    pub state: GameState,

    // line_counts[i] = # of filled blocks in row i
    line_counts: [i64; BOARD_HEIGHT as usize] ,
    // for "random bag" generation of the next tetromino
    tetrominos_bag: Vec<Tetromino>
}

impl Board {
    pub fn init_board() -> Board {
        let mut bag = TETROMINOS.to_vec();
        ::rand::thread_rng().shuffle(&mut bag);

        Board {
            cells: [[0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
            current_piece: bag.remove(0),
            state: GameState::Playing,

            line_counts: [0; BOARD_HEIGHT as usize],
            tetrominos_bag: bag
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
                        if self.can_move_current_piece_left() {
                            self.current_piece.move_left();
                        }
                    }
                    Button::Keyboard(Key::Right) => {
                        if self.can_move_current_piece_right() {
                            self.current_piece.move_right();
                        }
                    }
                    // P = pause button
                    Button::Keyboard(Key::P) => {
                        if self.state == GameState::Playing {
                            self.state = GameState::Paused;
                        } else if self.state == GameState::Paused {
                            self.state = GameState::Playing;
                        }
                    }
                    // R = restart button
                    Button::Keyboard(Key::R) => {
                        // reset all game variables
                        let mut bag = TETROMINOS.to_vec();
                        ::rand::thread_rng().shuffle(&mut bag);
                        
                        self.cells = [[0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];
                        self.current_piece = bag.remove(0);
                        self.state = GameState::Playing;
                        self.line_counts = [0; BOARD_HEIGHT as usize];
                        self.tetrominos_bag = bag;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // iterate thru board and clear the specified row
    fn clear_row(&mut self, row: usize) {
        for col in 0..BOARD_WIDTH {
            self.cells[row][col as usize] = 0;
        }
        self.line_counts[row] = 0;

        // move all cells above (and including) the cleared row down by one
        for r in (1..(row+1)).rev() {
            for col in 0..BOARD_WIDTH {
                self.cells[r as usize][col as usize] = self.cells[(r-1) as usize][col as usize];
            }
            self.line_counts[r as usize] = self.line_counts[(r-1) as usize];
        }
    }

    pub fn clear_line_if_needed(&mut self) {
        // iterate through rows affected by current piece
        let mut rows_affected = Vec::new();
        let current_x = self.current_piece.x_offset;
        let current_y = self.current_piece.y_offset;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let board_y: usize = ((r as f64) + current_y) as usize;
                    rows_affected.push(board_y);
                }
            }
        }

        for row in rows_affected.iter() {
            if self.line_counts[*row] == BOARD_WIDTH as i64 {
                self.clear_row(*row);
            }
        }
    }

    pub fn advance_board(&mut self) -> Result<GameState> {
        if self.state != GameState::Playing {
            return Ok(self.state.clone())
        }
        // check if game is over or not
        // check top row of board and see if any of them are filled?
        for (col, colblock) in self.cells[0].iter().enumerate() {
            if *colblock == 1 {
                // TODO: clear current piece?
                self.state = GameState::Over;
                return Ok(GameState::Over)
            }
        }

        // make the existing piece fall
        if self.can_move_current_piece_down() {
            self.current_piece.move_down();
        } else {
            // add piece to board cells
            self.set_piece_on_board();

            // clear line if necessary
            self.clear_line_if_needed();

            // get new piece
            self.current_piece = self.get_next_piece();
        }

        Ok(GameState::Playing)
    }

    fn can_move_current_piece_down(&self) -> bool {
        if self.current_piece.bottommost() + 1.0 >= BOARD_HEIGHT as f64 {
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
        true
    }

    fn can_move_current_piece_left(&self) -> bool {
        if self.current_piece.leftmost() - 1.0 < 0.0 {
            return false;
        }
        // check if will intersect with a piece already on the board
        let current_x = self.current_piece.x_offset;
        let current_y = self.current_piece.y_offset;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let board_x: i64 = (col as i64) + (current_x as i64) - 1;
                    let board_y: i64 = ((r as f64) + current_y) as i64; // 1 down
                    if self.cells[board_y as usize][board_x as usize] == 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn can_move_current_piece_right(&self) -> bool {
        if self.current_piece.rightmost() + 1.0 >= BOARD_WIDTH as f64  {
            return false;
        }
        // check if will intersect with a piece already on the board
        let current_x = self.current_piece.x_offset;
        let current_y = self.current_piece.y_offset;
        for (r, row) in self.current_piece.blocks.iter().enumerate() {
            for (col, colblock) in row.iter().enumerate() {
                if *colblock == 1 {
                    let board_x: i64 = (col as i64) + (current_x as i64) + 1;
                    let board_y: i64 = ((r as f64) + current_y) as i64; // 1 down
                    if self.cells[board_y as usize][board_x as usize] == 1 {
                        return false;
                    }
                }
            }
        }
        true
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
                    self.line_counts[y as usize] += 1;
                }
            }
        }
    }

    pub fn get_next_piece(&mut self) -> Tetromino {
        if self.tetrominos_bag.len() == 0 {
            self.tetrominos_bag = TETROMINOS.to_vec();
            ::rand::thread_rng().shuffle(&mut self.tetrominos_bag);
        }
        self.tetrominos_bag.remove(0)
    }

    pub fn draw_board(self) -> Result<()> {
        // draw the board ?
        Ok(())
    }
}