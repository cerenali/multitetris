use std::result;
pub type Result<T> = result::Result<T, String>;

pub static blocks: [[[u8; 4]; 4]; 7] = [
        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
];

pub struct Tetromino {
    pub blocks: [[u8; 4]; 4],
    pub x: f64, // col
    pub y: f64 // row
}

impl Tetromino {
    pub fn rotate(&mut self) -> Result<()> {
        let mut new_blocks = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                new_blocks[r][c] = self.blocks[4 - c - 1][r];
            }
        }

        self.blocks = new_blocks;
        Ok(())
    }

    pub fn block_height(&self) -> f64 {
        let mut height = 0.0;
        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if self.blocks[r][c] == 1 {
                    height += 1.0;
                    continue;
                }
            }
        }

        height
    }

    pub fn move_down(&mut self) {
        self.y += 0.1;
    }
}