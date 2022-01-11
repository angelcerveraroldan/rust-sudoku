use super::tile::Tile;

pub struct Board {
    pub(crate) tiles: Vec<Vec<Tile>>,
}

impl Board {
    /// Turn a 81 character long String into a sudoku board
    pub fn new(board_str: String) -> Board {
        if board_str.len() != 81 { panic!("The String passed to make the board isn't of length 81") }
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
            if self.tiles[row][n].value != 0 && n != col { cannot_be.push(self.tiles[row][n].value) }
            // Check row
            if self.tiles[n][col].value != 0 && n != row { cannot_be.push(self.tiles[n][col].value) }
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
            if !cannot_be.contains(&i) { pos.push(i) }
        }

        pos
    }
}
