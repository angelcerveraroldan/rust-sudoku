use super::tile::Tile;

#[derive(Clone)]
pub struct Board {
    pub(crate) tiles: Vec<Vec<Tile>>,
}

impl Board {
    /// Turn a 81 character long String into a sudoku board
    /// A number will represent the tiles value
    /// A 0 will represent an empty tile
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
                let c = board_chars.nth(0).unwrap().to_digit(10)
                    .expect("Please enter only numbers from 0 to 9, where 0 is an empty tile") as u8;

                row_vec.push(Tile::new(row_coordinate, col_coordinate, c));
            }

            // Add row to tiles
            tiles.push(row_vec);
        }

        Board { tiles }
    }

    /// Given a tiles coordinate, return a vector with all the values that could go in that tile
    /// To do this, we will first add every value in the same row and column to a vector
    /// We will then add every value in the same block to that same vector
    /// This vector will contain all numbers that tile CANNOT be
    ///
    /// Then we make a new vector containing every number not in the last vector
    /// This new vector contains all the values that tile CAN be
    pub fn check_possibilities(&self, row: usize, col: usize) -> Vec<u8> {
        // Vector containing all values it cannot be
        let mut cannot_be: Vec<u8> = Vec::new();
        // Vector containing all possible values
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

    /// Check if the sudoku is finished, if there are no empty tiles, it is finished
    pub fn finished(&self) -> bool {
        let mut finished = true;

        for row in 0..9 {
            for col in 0..9 {
                if self.tiles[row][col].value == 0 {
                    finished = false;
                    break;
                }
            }
        }

        finished
    }

    /// When we need to guess the value a tile contains, we will need to have multiple boards each with a different value for a given tile
    /// This function will clone the board and change one of its values
    pub fn change_tile(&self, row: usize, col: usize, val: &u8) -> Board {
        let mut clone = self.clone();

        clone.tiles[row][col].value = *val;

        clone
    }

    /// Update all tiles
    /// This will change a tiles value if there is only one possible value for it
    /// It will also update every tiles possible_values vector
    /// We need to run it in a loop until it is no longer able to simplify it further or until the board is finished
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

            // We want to keep simplifying the board until there are no more possible changes we can make without guessing
            // or until the board is finished
            if self.finished() || !i {
                break;
            }
        }
    }

    /// This will update the board as much as it can using the simplify function
    /// If it cant finish the sudoku, it will fill the board with every possible value for its first empty tile
    pub fn solve(&mut self, answers: &mut Vec<Board>) {
        self.simplify();

        if self.finished() {
            // If we solve a board, add the solved board to the answers vector
            answers.push(self.clone());
        } else {
            // Find the first empty cell and fill it with all values,
            // we can assume there is an empty board since if there weren't any, board.finished() would return true
            let coord = self.first_empty();

            // Run the solve function after filling the first empty tile with all its possible values
            for val in &self.tiles[coord[0]][coord[1]].possible_values {
                self.change_tile(coord[0], coord[1], val).solve(answers);
            }
        }
    }


    /// Return the index of the first empty cell
    pub fn first_empty(&self) -> Vec<usize> {
        for row in 0..9 {
            for col in 0..9 {
                if self.tiles[row][col].value == 0 {
                    return vec![row, col];
                }
            }
        }

        panic!("There are no empty tiles!");
    }

    /// Display board in the terminal
    /// TODO: Implement display trait
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
