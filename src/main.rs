mod tile;
mod board;

use board::Board;

fn main() {
    let mut board = Board::new(String::from(
        "500086001002000600071000250910020070300145006060090024053000060000903000200510000"));

    // Display Sudoku before solving
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board.tiles[i][j].value);
        }
    }

    println!();

    // Try to solve sudoku in 500 iterations
    // This has to be changed
    loop {
        let mut i = false;

        for row in 0..9 {
            for col in 0..9 {
                let pos = board.check_possibilities(row, col);
                if board.tiles[row][col].value == 0 {
                    // i will be true if any value of the sudoku is changed
                    i = i || board.tiles[row][col].update_tile(pos);
                }
            }
        }

        if !i {
            break;
        }
    }

    // Display sudoku after solving
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board.tiles[i][j].value);
        }
    }

    println!();
}