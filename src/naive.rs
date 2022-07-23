use std::{error, fmt};

const BLANK_SQUARE: i8 = -1;
const BOARD_LEN: usize = 81;
const SECTION_LEN: usize = BOARD_LEN / 9;
const SECTION_OPTIONS: [i8; SECTION_LEN] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

pub struct SudokuSolver([i8; BOARD_LEN], pub usize);

impl SudokuSolver {
    pub fn try_new(raw: &String) -> Result<Self, SolverError> {
        if raw.len() != BOARD_LEN {
            return Err(SolverError::InvalidLength);
        }

        let mut board = [Default::default(); BOARD_LEN];
        let mut chars = raw.chars();
        for i in 0..board.len() {
            board[i] = match chars.next() {
                Some(c) => Self::magic_matcher(&c)?,
                None => unreachable!(),
            }
        }

        Ok(Self(board, 0))
    }

    pub fn solve(&self) -> Result<Self, SolverError> {
        let mut backtracks = 0;
        let mut board = self.0.clone();
        let mut attempts = [usize::default(); BOARD_LEN];
        let mut i = 0;
        let mut attempt_stack: Vec<usize> = Vec::with_capacity(BOARD_LEN);
        'next_index: while i != BOARD_LEN {
            if self.0[i] == BLANK_SQUARE {
                let row = Self::get_row(&board, i);
                let col = Self::get_col(&board, i);
                let quad = Self::get_quad(&board, i);
                let row_missing = Self::get_missing(&row);
                let col_missing = Self::get_missing(&col);
                let quad_missing = Self::get_missing(&quad);
                let missing =
                    Self::intersect(quad_missing, Self::intersect(row_missing, col_missing));

                for attempt in missing.skip(attempts[i]) {
                    attempt_stack.push(i);
                    attempts[i] += 1;
                    board[i] = *attempt;
                    i += 1;
                    continue 'next_index;
                }

                let prev_i = match attempt_stack.pop() {
                    Some(x) => {
                        backtracks += 1;
                        x
                    }
                    None => {
                        return Err(SolverError::InternalError(
                            "Solution Backtracking Stack is Empty",
                        ))
                    }
                };
                attempts[i] = 0;
                board[i] = BLANK_SQUARE;
                board[prev_i] = BLANK_SQUARE;
                i = prev_i;
            } else {
                i += 1;
            }
        }
        Ok(Self(board, backtracks))
    }

    fn get_row(board: &[i8; BOARD_LEN], i: usize) -> [i8; SECTION_LEN] {
        let accessor = |x: usize| (i % SECTION_LEN) + x * SECTION_LEN;
        [
            board[accessor(0)],
            board[accessor(1)],
            board[accessor(2)],
            board[accessor(3)],
            board[accessor(4)],
            board[accessor(5)],
            board[accessor(6)],
            board[accessor(7)],
            board[accessor(8)],
        ]
    }

    fn get_col(board: &[i8; BOARD_LEN], i: usize) -> [i8; SECTION_LEN] {
        let accessor = |x: usize| (i / SECTION_LEN) * SECTION_LEN + x;
        [
            board[accessor(0)],
            board[accessor(1)],
            board[accessor(2)],
            board[accessor(3)],
            board[accessor(4)],
            board[accessor(5)],
            board[accessor(6)],
            board[accessor(7)],
            board[accessor(8)],
        ]
    }

    fn get_quad(board: &[i8; BOARD_LEN], i: usize) -> [i8; SECTION_LEN] {
        let accessor =
            |x: usize| (i / (3 * SECTION_LEN)) * (3 * SECTION_LEN) + (i % SECTION_LEN / 3) * 3 + x;
        [
            board[accessor(00)],
            board[accessor(01)],
            board[accessor(02)],
            board[accessor(09)],
            board[accessor(10)],
            board[accessor(11)],
            board[accessor(18)],
            board[accessor(19)],
            board[accessor(20)],
        ]
    }

    fn get_missing(arr: &[i8; SECTION_LEN]) -> impl Iterator<Item = &i8> {
        SECTION_OPTIONS.iter().filter(move |&n| !arr.contains(n))
    }

    fn intersect<'a>(
        a: impl Iterator<Item = &'a i8>,
        b: impl Iterator<Item = &'a i8>,
    ) -> impl Iterator<Item = &'a i8> {
        let a: Vec<&i8> = a.collect();
        b.filter(move |n| a.contains(n))
    }

    fn magic_matcher(c: &char) -> Result<i8, SolverError> {
        match c {
            '.' => Ok(BLANK_SQUARE),
            '1' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            c => Err(SolverError::ParserError(*c)),
        }
    }

    fn magic_displayer(i: &i8) -> char {
        match i {
            &BLANK_SQUARE => '.',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => '_',
        }
    }
}

impl fmt::Display for SudokuSolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        for (i, n) in self.0.iter().enumerate() {
            write!(f, "{} ", Self::magic_displayer(n))?;

            if (i + 1) % 3 == 0 {
                write!(f, "  ")?;
            }

            if (i + 1) % 9 == 0 {
                writeln!(f)?;
            }

            if (i + 1) % 27 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SolverError {
    InternalError(&'static str),
    InvalidLength,
    ParserError(char),
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Self::InternalError(s) => write!(f, "Internal Solver Error: {}", s)?,
            Self::InvalidLength => write!(f, "Sudoku puzzle is incorrect length")?,
            Self::ParserError(c) => write!(f, "Could not parse character {}", c)?,
        }
        Ok(())
    }
}

impl error::Error for SolverError {}
