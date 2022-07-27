#[derive(Clone)]
pub struct Tile {
    pub row_coordinate: u8,
    pub col_coordinate: u8,

    pub value: u8,
    pub possible_values: Vec<u8>,
}

impl Tile {
    pub fn new(row_coordinate: u8, col_coordinate: u8, value: u8) -> Tile {
        // We will update this later for each empty value
        let possible_values: Vec<u8> = Vec::new();

        Tile {
            row_coordinate,
            col_coordinate,
            value,
            possible_values,
        }
    }

    /// Given a vec of possible values, update a tile
    /// If there is only one possible value for a tile, fill it in and empty the possible_values vector
    /// If there are multiple possible values for a tile, update the possible_values vector
    ///
    /// Because there can only be one answer to any tiles this function changes, we will mutate the value in the board
    pub fn update_tile(&mut self, new_possible: Vec<u8>) -> bool {
        if self.value != 0 {
            panic!("Don't update a tile that's not empty!");
        }

        if new_possible.len() == 1 {
            self.value = new_possible[0];
            self.possible_values = Vec::new();
            true // Return true if a value in the board has been changed
        } else {
            self.possible_values = new_possible;
            false // Return false if no value was changed
        }
    }

}
