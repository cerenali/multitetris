use std::result;
use super::BOARD_WIDTH;
use super::BOARD_HEIGHT;

pub type Result<T> = result::Result<T, String>;

const MOVEMENT_SPEED: f64 = 1.0;

pub static TETROMINOS: [Tetromino; 7] = [
    Tetromino {
        name: Shape::I,
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 2.0,
        y_offset: 0.0,
        color: [0.0, 1.0, 1.0, 1.0]
    },
    Tetromino {
        name: Shape::J,
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 2.0,
        y_offset: 0.0,
        color: [0.0, 1.0, 1.0, 1.0]
    },
    Tetromino {
        name: Shape::L,
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 2.0,
        y_offset: 0.0,
        color: [1.0, 0.6, 0.0, 1.0]
    },
    Tetromino {
        name: Shape::O,
        blocks: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 1.0,
        y_offset: 0.0,
        color: [1.0, 1.0, 0.0, 1.0]
    },
    Tetromino {
        name: Shape::S,
        blocks: [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 2.0,
        y_offset: 0.0,
        color: [0.48, 1.0, 0.0, 1.0]
    },
    Tetromino {
        name: Shape::T,
        blocks: [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 2.0,
        y_offset: 0.0,
        color: [0.4, 0.0, 0.8, 1.0]
    },
    Tetromino {
        name: Shape::Z,
        blocks: [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        x_offset: (BOARD_WIDTH as f64 / 2.0) - 2.0,
        y_offset: 0.0,
        color: [1.0, 0.0, 0.0, 1.0]
    },  
];

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L
}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    pub name: Shape,
    pub blocks: [[u8; 4]; 4],
    pub x_offset: f64, // offset (in block cell units) of the top left cell
    pub y_offset: f64,
    pub color: [f32; 4] // color of the block
}

impl Tetromino {
    pub fn rotate_right(&mut self) {
        if self.name == Shape::O {
            return
        }
        let mut new_blocks = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                new_blocks[r][c] = self.blocks[4 - c - 1][r];
            }
        }
        self.blocks = new_blocks;
    }

    pub fn rotate_left(&mut self) {
        if self.name == Shape::O {
            return
        }
        let mut new_blocks = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                new_blocks[r][c] = self.blocks[c][4 - r - 1];
            }
        }

        self.blocks = new_blocks;
    }

    pub fn block_height(&self) -> f64 {
        let mut height = 0.0;
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    height += 1.0;
                    break;
                }
            }
        }
        height
    }

    pub fn block_width(&self) -> f64 {
        let mut cols_vec: [i64; 4] = [0, 0, 0, 0];
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    cols_vec[c] = 1;
                }
            }
        }
        let mut width = 0.0;
        for x in cols_vec.iter() {
            if *x == 1 {
                width += 1.0;
            }
        }
        width
    }

    // coordinates of leftmost point in block
    pub fn leftmost(&self) -> f64 {
        let mut leftmost = 3.0;
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    if leftmost > c as f64 {
                        leftmost = c as f64;
                    }
                }
            }
        }
        leftmost + self.x_offset
    }

    pub fn rightmost(&self) -> f64 {
        self.leftmost() + self.block_width() - 1.0
    }

    pub fn topmost(&self) -> f64 {
        let mut topmost = 3.0;
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    if topmost > r as f64 {
                        topmost = r as f64;
                    }
                }
            }
        }
        topmost + self.y_offset
    }

    pub fn bottommost(&self) -> f64 {
        self.topmost() + self.block_height() - 1.0
    }

    pub fn move_down(&mut self) {
        self.y_offset += MOVEMENT_SPEED;
    }

    pub fn move_left(&mut self) {
        self.x_offset -= MOVEMENT_SPEED;
    }

    pub fn move_right(&mut self) {
        self.x_offset += MOVEMENT_SPEED;
    }
}