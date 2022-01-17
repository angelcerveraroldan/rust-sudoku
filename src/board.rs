use super::tile::Tile;

#[derive(Clone)]
pub struct Board {
    pub(crate) tiles: Vec<Vec<Tile>>,
}

impl Board {
    /// Turn a 81 character long String into a sudoku board
    pub fn new(board_str: String) -> Board {
        if board_str.len() != 81 {
            panic!("The String passed to make the board isn't of length 81")
        }
        let mut board_chars = board_str.chars();

        let mut tiles: Vec<Vec<Tile>> = Vec::new();

        for row_coordinate in 0..9 {
            // Create row
            let mut row_vec: Vec<Tile> = Vec::new();
            for col_coordinate in 0..9 {
                // Fill row
                let c = board_chars.nth(0).unwrap().to_digit(10).unwrap() as u8;
                row_vec.push(Tile::new(row_coordinate, col_coordinate, c));
            }

            // Add row to tiles
            tiles.push(row_vec);
        }

        Board { tiles }
    }

    pub fn check_possibilities(&self, row: usize, col: usize) -> Vec<u8> {
        let mut cannot_be: Vec<u8> = Vec::new();
        let mut pos: Vec<u8> = Vec::new();

        for n in 0..9 {
            // Check column
            if self.tiles[row][n].value != 0 && n != col {
                cannot_be.push(self.tiles[row][n].value)
            }
            // Check row
            if self.tiles[n][col].value != 0 && n != row {
                cannot_be.push(self.tiles[n][col].value)
            }
        }

        // Check block
        let block_index = ((row / 3) as usize, (col / 3) as usize);
        // Top left tile in that block is
        let block_top_left = (3 * block_index.0, 3 * block_index.1);

        for i in 0..3 {
            for j in 0..3 {
                if self.tiles[block_top_left.0 + i][block_top_left.1 + j].value != 0 {
                    cannot_be.push(self.tiles[block_top_left.0 + i][block_top_left.1 + j].value)
                }
            }
        }

        for i in 1..10 {
            if !cannot_be.contains(&i) {
                pos.push(i)
            }
        }

        pos
    }

    pub fn finished(&self) -> bool {
        let mut finished = true;

        for row in 0..9 {
            for col in 0..9 {
                if self.tiles[row][col].value == 0 {
                    finished = false
                }
            }
        }

        finished
    }

    // Todo improve valid function
    pub fn valid(&self) -> bool {
        let mut valid = true;

        for row in 0..9 {
            for col in 0..9 {
                if self.tiles[row][col].value == 0 && self.tiles[row][col].possible_values.len() < 1
                {
                    valid = false;
                }
            }
        }

        valid
    }

    pub fn change_tile(&self, row: usize, col: usize, val: &u8) -> Board {
        let mut clone = self.clone();

        clone.tiles[row][col].value = *val;

        clone
    }

    /// Update all tiles that only have one valid answer
    pub fn simplify(&mut self) {
        loop {
            let mut i = false;

            for row in 0..9 {
                for col in 0..9 {
                    let pos = self.check_possibilities(row, col);
                    if self.tiles[row][col].value == 0 {
                        // i will be true if any value of the sudoku is changed
                        i = i || self.tiles[row][col].update_tile(pos);
                    }
                }
            }

            if self.finished() || !i {
                break;
            }
        }
    }

    pub fn first_empty(&self) -> Vec<usize> {
        for row in 0..9 {
            for col in 0..9 {
                if self.tiles[row][col].value == 0 {
                    return vec![row, col];
                }
            }
        }

        panic!("aa");
    }

    pub fn display(&self) {
        println!();

        for i in 0..9 {
            for j in 0..9 {
                print!("{} ", self.tiles[i][j].value);

                if (j + 1) % 3 == 0 {
                    print!(" ");
                }
            }

            println!();
            if (i + 1) % 3 == 0 {
                println!();
            }
        }
    }
}
