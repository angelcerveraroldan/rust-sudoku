pub struct Tile {
    row_coordinate: u8,
    col_coordinate: u8,

    value: u8,
    possible_values: Vec<u8>,
}

impl Tile {
    pub fn new(row_coordinate: u8, col_coordinate: u8, value: u8) -> Tile {
        // We will update this later for each empty value
        let possible_values: Vec<u8>= Vec::new();

        Tile { row_coordinate, col_coordinate, value, possible_values }
    }

    /// Given a vec of possible values, update a tile
    pub fn update_tile(&mut self, new_possible: Vec<u8>) {
        if self.value != 0 { panic!("Don't update a tile that's not empty!"); }
        if new_possible.len() == 1 {
            self.value = new_possible[0];
            self.possible_values = Vec::new();
        } else { self.possible_values = new_possible; }
    }

    pub fn display(&self) {
        println!("(col, row): ({}, {}), value: {}, possible_values: {:?}",
                 self.col_coordinate, self.row_coordinate, self.value, self.possible_values);
    }
}