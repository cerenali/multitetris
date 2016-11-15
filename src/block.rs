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
    pub blocks: [[u8; 4]; 4]
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

    pub fn move_down(&mut self) -> Result<()> {
        Ok(())
    }
}