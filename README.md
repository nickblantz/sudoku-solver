# sudoku-solver

Two sudoku solver implementations with naive backtracking and wave function collapse 


## Example
[A Sudoku designed to work against the brute force algorithm](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#/media/File:Sudoku_puzzle_hard_for_brute_force.svg)

```
Unsolved:
. . .   . . .   . . .
. . .   . . 3   . 8 5
. . 1   . 2 .   . . .

. . .   5 . 7   . . .
. . 4   . . .   1 . .
. 9 .   . . .   . . .

5 . .   . . .   . 7 3
. . 2   . 1 .   . . .
. . .   . 4 .   . . 9


WFC | 6385 backtracks | 0.11 seconds
9 8 7   6 5 4   3 2 1
2 4 6   1 7 3   9 8 5
3 5 1   9 2 8   7 4 6

1 2 8   5 3 7   6 9 4
6 3 4   8 9 2   1 5 7
7 9 5   4 6 1   8 3 2

5 1 9   2 8 6   4 7 3
4 7 2   3 1 9   5 6 8
8 6 3   7 4 5   2 1 9


Naive | 69175252 backtracks | 53.83 seconds
9 8 7   6 5 4   3 2 1
2 4 6   1 7 3   9 8 5
3 5 1   9 2 8   7 4 6

1 2 8   5 3 7   6 9 4
6 3 4   8 9 2   1 5 7
7 9 5   4 6 1   8 3 2

5 1 9   2 8 6   4 7 3
4 7 2   3 1 9   5 6 8
8 6 3   7 4 5   2 1 9
```