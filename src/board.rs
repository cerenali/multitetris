use piston::input::*;

use rand::Rng;

use super::BOARD_WIDTH;
use super::BOARD_HEIGHT;
use super::block::Shape;
use super::block::Tetromino;
use super::block::TETROMINOS;

// 1st entry is # of points for clearing 1 line, etc.
const SCORES_PER_LINE: [u64; 4] = [80, 200, 600, 2400];

const GHOST_PIECE_OPACITY: f32 = 0.35;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameState {
  Playing,
  Paused,
  Over // donezo. player lost or quit
}

pub struct Board {
  pub cells: [[u8; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
  pub current_piece: Tetromino, // current active Tetromino
  pub ghost_piece: Tetromino, // ghost piece to display at bottom
  pub state: GameState,
  pub score: u64,

  pub token: i32, // token identifier for use by the game server
  pub new_block: bool, // true for 1 iteration if new block, false otherwise

  // line_counts[i] = # of filled blocks in row i
  line_counts: [i64; BOARD_HEIGHT as usize],
  // for "random bag" generation of the next tetromino
  tetrominos_bag: Vec<Tetromino>,
}

impl Board {
  pub fn init_board() -> Board {
    let mut bag = TETROMINOS.to_vec();
    ::rand::thread_rng().shuffle(&mut bag);

    let first_piece: Tetromino = bag.remove(0);
    let mut ghost_piece: Tetromino = first_piece.clone();
    ghost_piece.y_offset = BOARD_HEIGHT as f64 - ghost_piece.block_height();
    // some shapes are drawn starting from 2nd, not 1st row in block
    if ghost_piece.name == Shape::T || ghost_piece.name == Shape::S || ghost_piece.name == Shape::Z {
      ghost_piece.y_offset -= 1.0;
    }
    ghost_piece.color[3] = GHOST_PIECE_OPACITY; // transparent

    Board {
      cells: [[0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
      current_piece: first_piece,
      ghost_piece: ghost_piece,
      state: GameState::Playing,
      score: 0,

      token: 0,
      new_block: true,

      line_counts: [0; BOARD_HEIGHT as usize],
      tetrominos_bag: bag
    }
  }

  pub fn handle_key_press(&mut self, inp: &Input) {
    match *inp {
      Input::Press(but) => {
        match but {
          Button::Keyboard(Key::Up) => {
            self.current_piece.rotate_right();
            self.ghost_piece.rotate_right();
            // undo rotation if it was invalid...lol
            if self.current_piece_out_of_bounds() {
              self.current_piece.rotate_left();
              self.ghost_piece.rotate_left();
            }
            self.update_ghost_piece();
          }
          Button::Keyboard(Key::Left) => {
            if self.can_move_current_piece_left() {
              self.current_piece.move_left();
              self.update_ghost_piece();
            }
          }
          Button::Keyboard(Key::Right) => {
            if self.can_move_current_piece_right() {
              self.current_piece.move_right();
              self.update_ghost_piece();
            }
          }
          Button::Keyboard(Key::Down) => {
            if self.can_move_piece_down(&self.current_piece) {
              self.current_piece.move_down();
            }
          }
          Button::Keyboard(Key::Space) => {
            // pressing spacebar drops piece to bottom
            while self.can_move_piece_down(&self.current_piece) {
              self.current_piece.move_down();
            }
          }
          // TODO: these options disabled for multiplayer

          // P = pause button
          // Button::Keyboard(Key::P) => {
          //     if self.state == GameState::Playing {
          //         self.state = GameState::Paused;
          //     } else if self.state == GameState::Paused {
          //         self.state = GameState::Playing;
          //     }
          // }

          // R = restart button
          // Button::Keyboard(Key::R) => {
          //     // reset all game variables
          //     let mut bag = TETROMINOS.to_vec();
          //     ::rand::thread_rng().shuffle(&mut bag);

          //     self.cells = [[0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];
          //     self.current_piece = bag.remove(0);
          //     self.state = GameState::Playing;
          //     self.line_counts = [0; BOARD_HEIGHT as usize];
          //     self.tetrominos_bag = bag;
          //     self.score = 0;
          //     println!("===== GAME RESTARTED =====");
          // }
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
    let current_y = self.current_piece.y_offset;

    for r in 0..self.current_piece.blocks.len() {
      for c in 0..self.current_piece.blocks[0].len() {
        if self.current_piece.blocks[r][c] == 1 {
          let board_y: usize = ((r as f64) + current_y) as usize;
          rows_affected.push(board_y);
        }
      }
    }

    let mut rows_cleared = 0;
    for row in rows_affected.iter() {
      if self.line_counts[*row] == BOARD_WIDTH {
        self.clear_row(*row);
        rows_cleared += 1;
      }
    }
    if rows_cleared > 0 {
      self.score += SCORES_PER_LINE[rows_cleared - 1];
    }
  }

  pub fn advance_board(&mut self) {
    // println!("=== SCORE: {:?} ===", self.score);

    if self.state != GameState::Playing {
      return
    }
    // check if game is over or not
    // check top row of board and see if any of them are filled?
    for col in 0..self.cells[0].len() {
      if self.cells[0][col] == 1{
        self.state = GameState::Over;
        return
      }
    }

    // make the existing piece fall
    if self.can_move_piece_down(&self.current_piece) {
      self.new_block = false;
      self.current_piece.move_down();
    } else {
      // add piece to board cells
      self.set_piece_on_board();

      // clear line if necessary
      self.clear_line_if_needed();

      // get new piece
      self.current_piece = self.get_next_piece();
      self.new_block = true;
      self.update_ghost_piece();
    }
  }

  fn can_move_piece_down(&self, piece: &Tetromino) -> bool {
    if piece.bottommost() + 1.0 >= BOARD_HEIGHT as f64 {
      return false
    }
    // check if piece will intersect with a piece already on the board
    let current_x = piece.x_offset;
    let current_y = piece.y_offset;
    for row in 0..piece.blocks.len() {
      for col in 0..piece.blocks[0].len() {
        if piece.blocks[row][col] == 1 {
          let board_x: i64 = (col as i64) + (current_x as i64);
          let board_y: i64 = (row as i64) + (current_y as i64) + 1;
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
    for row in 0..self.current_piece.blocks.len() {
      for col in 0..self.current_piece.blocks[0].len() {
        if self.current_piece.blocks[row][col] == 1 {
          let board_x: i64 = (col as i64) + (current_x as i64) - 1;
          let board_y: i64 = (row as i64) + (current_y as i64);
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
    for row in 0..self.current_piece.blocks.len() {
      for col in 0..self.current_piece.blocks[0].len() {
        if self.current_piece.blocks[row][col] == 1 {
          let board_x: i64 = (col as i64) + (current_x as i64) + 1;
          let board_y: i64 = (row as i64) + (current_y as i64);
          if self.cells[board_y as usize][board_x as usize] == 1 {
            return false;
          }
        }
      }
    }
    true
  }

  fn current_piece_out_of_bounds(&self) -> bool {
    if self.current_piece.leftmost() < 0.0 || self.current_piece.rightmost() >= BOARD_WIDTH as f64 || self.current_piece.bottommost() >= BOARD_HEIGHT as f64 {
      return true
    }
    false
  }

  pub fn set_piece_on_board(&mut self) {
    let current_x = self.current_piece.x_offset;
    let current_y = self.current_piece.y_offset;
    for row in 0..self.current_piece.blocks.len() {
      for col in 0..self.current_piece.blocks[0].len() {
        if self.current_piece.blocks[row][col] == 1 {
          let x: i64 = (col as i64) + (current_x as i64);
          let y: i64 = (row as i64) + (current_y as i64);
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

  // helper function to update position of ghost piece w/ current piece
  pub fn update_ghost_piece(&mut self) {
    if self.current_piece.name != self.ghost_piece.name {
      self.ghost_piece = self.current_piece.clone();
      self.ghost_piece.color[3] = GHOST_PIECE_OPACITY;
    }
    self.ghost_piece.y_offset = 0.0;
    self.ghost_piece.x_offset = self.current_piece.x_offset;
    while self.can_move_piece_down(&self.ghost_piece) {
      self.ghost_piece.move_down();
    }
  }
}