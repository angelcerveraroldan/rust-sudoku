mod board;
mod tile;

use board::Board;

fn main() {
    let mut board = Board::new(String::from(
        "287591364453286197900437528624719835839645271175823946742358619398164752500972483"
    ));

    // Vector where all the possible answers to the sudoku will be stored
    let mut answers: Vec<Board>= Vec::new();

    println!("Unsolved sudoku: ");
    board.display();
    println!("Answers: ");

    // Fill the answers vector with all possible answers
    board.solve(&mut answers);

    // Print every possible solution to the entered sudoku
    for board in answers {
        board.display();
        println!("\n");
    }
}

