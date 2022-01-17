mod board;
mod tile;

use board::Board;

//    let mut board = Board::new(String::from(
//         "070000043040009610800634900094052000358460020000800530080070091902100005007040802"));

fn main() {
    // Can only solve a simple sudoku
    let mut board = Board::new(String::from(
        "00000000042100005000700008100000302908320000002070809070530400090000675000600093",
    ));

    // let mut board = Board::new(String::from(
    //     "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    // ));

    board.display();

    solve(board);
}

fn solve(mut board: Board) {
    board.simplify();

    if board.finished() {
        board.display();
        println!("\n\n\n");
    } else {
        // Find the first empty cell and fill it with all values
        let coord = board.first_empty();

        for val in &board.tiles[coord[0]][coord[1]].possible_values {
            solve(board.change_tile(coord[0], coord[1], val));
        }
    }
}
