pub static blocks: [Tetromino; 7] = [
    Tetromino { // I
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]]
    },
    Tetromino { // J
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]]
    },
    Tetromino { // L
        blocks: [[0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]]
    },
    Tetromino { // O
        blocks: [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    },
    Tetromino { // S
        blocks: [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]]
    },
    Tetromino { // T
        blocks: [[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]]
    },
    Tetromino { // Z
        blocks: [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
    },
];


pub struct Tetromino {
    // properties go here?
    blocks: [[u8; 4]; 4]
}

pub enum Direction {
    Left,
    Right
}

impl Tetromino {
    pub fn rotate(self, dir: Direction) -> Tetromino {
        let mut new_blocks = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

        for (r, row) in self.blocks.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                new_blocks[r][c] = self.blocks[4 - c - 1][r];
            }
        }

        Tetromino {
            blocks: new_blocks
        }
    }
}