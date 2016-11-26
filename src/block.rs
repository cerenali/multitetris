use std::result;
use super::BLOCK_SIZE;

pub type Result<T> = result::Result<T, String>;

const down_speed: f64 = 0.05;
const side_movement_speed: f64 = 1.0;


pub static TETROMINOES: [Tetromino; 7] = [
    Tetromino {
        name: Shape::I,
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
    },
    Tetromino {
        name: Shape::J,
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
    },
    Tetromino {
        name: Shape::L,
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
    },
    Tetromino {
        name: Shape::O,
        blocks: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
    },
    Tetromino {
        name: Shape::S,
        blocks: [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
    },
    Tetromino {
        name: Shape::T,
        blocks: [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
    },
    Tetromino {
        name: Shape::Z,
        blocks: [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
        x_offset: 0.0,
        y_offset: 0.0
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
    pub x_offset: f64,
    pub y_offset: f64
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
        // get rightmost block and subtract x from it
        let mut max_width = 0.0;
        for (r, row) in self.blocks.iter().enumerate() {
            let mut width = 0.0;
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    width += 1.0;
                }
            }
            if width > max_width {
                max_width = width;
            }
        }
        // max_width -= self.x;
        println!(">> width for block {:?}: {:?}", self.name, max_width);
        max_width
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
        let mut rightmost = 0.0;
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    if rightmost < c as f64 {
                        rightmost = c as f64;
                    }
                }
            }
        }
        rightmost + self.x_offset + self.block_width()
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
        let mut bottommost = 0.0;
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    if bottommost < r as f64 {
                        bottommost = r as f64;
                    }
                }
            }
        }
        bottommost + self.y_offset + self.block_height()
    }

    pub fn move_down(&mut self) {
        self.y_offset += down_speed;
    }

    pub fn move_left(&mut self) {
        self.x_offset -= side_movement_speed;
    }

    pub fn move_right(&mut self) {
        self.x_offset += side_movement_speed;
    }
}