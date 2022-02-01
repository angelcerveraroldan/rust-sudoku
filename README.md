# Rust sudoku algorithm
A simple algorithm to get an answer for any valid sudoku.

## How the algorithm works
First, the algorithm will go through each one of the tiles in the board and check what numbers could be in each one of the tiles. To do this, it checks all numbers in the same row, column and box  and makes a vec containing all numbers not found. This vec will contain all numbers that could be contained in the tile, if the vec length is 1, it will fill in the tile. 

Using just that part of the algorithm, we can already solve some simple sudoku, but not all.  

To find an answer to any valid sudoku, we will let the computer guess what goes in some tiles.
If we run the first part of the algorithm and none of the tiles values are updated, then it will find the tile with the smallest number of possible allowed values and fill in the tile with one of the values. We then run the first part of the algorithm again and repeat until the sudoku is solved or until we see an error.

We can tell that there has been an error if an empty tile has no possible values.

## Todos:
- [ ] Generate a sudoku to then play
- [ ] Check it sudoku is invalid
