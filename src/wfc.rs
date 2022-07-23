use rand::prelude::{SliceRandom, ThreadRng};
use std::{convert::TryInto, fmt};

use crate::cell::{Cell, CellState};

#[derive(Debug)]
pub enum ParseError {
    InvalidSize(usize),
    InvalidInput(usize, char),
    InternalError,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidSize(i) => {
                write!(f, "A board was provided with an invalid length of {}", i)?
            }
            Self::InvalidInput(i, c) => write!(f, "Character {} at position {} is invalid", c, i)?,
            Self::InternalError => write!(f, "An internal error has occurred")?,
        }

        Ok(())
    }
}

/// Represents the state of a board at a given time
pub type BoardState = [Cell; 81];

pub struct Board {
    /// Current state of the board
    state: BoardState,

    /// A stack of the historic board states
    history: Vec<BoardState>,

    /// The number of backtracks the solution required
    backtracks: usize,

    /// Random noise for selecting and solving cells
    rng: ThreadRng,
}

impl Board {
    /// Parse a sudoku board where '.' represents an unknown cell
    pub fn try_new(raw: &String) -> Result<Self, ParseError> {
        if raw.len() != 81 {
            return Err(ParseError::InvalidSize(raw.len()));
        }

        let char_map = |c: (usize, char)| -> Result<Cell, ParseError> {
            match c.1 {
                '.' => Ok(Cell::default()),
                '1' => Ok(Cell::from(0)),
                '2' => Ok(Cell::from(1)),
                '3' => Ok(Cell::from(2)),
                '4' => Ok(Cell::from(3)),
                '5' => Ok(Cell::from(4)),
                '6' => Ok(Cell::from(5)),
                '7' => Ok(Cell::from(6)),
                '8' => Ok(Cell::from(7)),
                '9' => Ok(Cell::from(8)),
                _ => Err(ParseError::InvalidInput(c.0, c.1)),
            }
        };

        match TryInto::<[Cell; 81]>::try_into(
            raw.chars()
                .enumerate()
                .map(char_map)
                .collect::<Result<Vec<Cell>, ParseError>>()?,
        ) {
            Ok(state) => Ok(Self {
                state,
                history: vec![],
                backtracks: 0,
                rng: rand::thread_rng(),
            }),
            Err(_) => Err(ParseError::InternalError),
        }
    }

    /// An iterator over all adjacent cells to i
    fn adjacencies(i: usize) -> impl Iterator<Item = usize> {
        [0, 1, 2, 9, 10, 11, 18, 19, 20]
            .iter()
            .map(move |&x| (i / (27)) * (27) + (i % 9 / 3) * 3 + x)
            .chain((0..9).map(move |x| (i / 9) * 9 + x))
            .chain((0..9).map(move |x| (i % 9) + x * 9))
            .filter(move |&x| i != x)
    }

    /// Iterates over the board and collapse cells
    fn collapse(&mut self, updates: Vec<usize>) {
        let mut updates = updates;

        while !updates.is_empty() {
            let mut new_updates = vec![];

            for i in 0..self.state.len() {
                if self.state[i].result().is_some() {
                    continue;
                }

                let state = Self::adjacencies(i)
                    .filter(|j| updates.iter().find(|&k| j == k).is_some())
                    .map(|j| &self.state[j])
                    .fold(CellState::ZERO, |acc, c| acc | c.state());

                if state.count_ones() == 0 {
                    continue;
                }

                self.state[i].collapse(state);

                if self.state[i].result().is_some() {
                    new_updates.push(i);
                }
            }

            for &i in &updates {
                self.state[i] = self.state[i].solve();
            }

            updates = new_updates;
        }
    }

    /// Randomly selects once cell with the lowest entropy
    fn lowest_entropy(&mut self) -> Option<usize> {
        let mut cells = self
            .state
            .iter()
            .enumerate()
            .filter(|(_, c)| c.result().is_none())
            .collect::<Vec<(usize, &Cell)>>();

        if cells.is_empty() {
            return None;
        }

        cells.sort_by(|&(_, c1), &(_, c2)| c1.entropy().cmp(&c2.entropy()));

        let least_entropy = cells[0].1.entropy();

        cells
            .iter()
            .take_while(|(_, c)| c.entropy() == least_entropy)
            .map(|&(i, _)| i)
            .collect::<Vec<usize>>()
            .choose(&mut self.rng)
            .map(ToOwned::to_owned)
    }

    /// Tries to solve a cell, if there is no solution it resets the board
    fn observe(&mut self, i: usize) -> Vec<usize> {
        match self.state[i].solve_rng(&mut self.rng) {
            Some(cell) => {
                self.history.push({
                    let mut state = self.state.clone();
                    state[i].collapse(cell.state());
                    state
                });
                self.state[i] = cell;
                vec![i]
            }
            None => {
                self.backtracks += 1;
                self.state = self.history.pop().unwrap();
                vec![]
            }
        }
    }

    /// Fills in every unsolved cell
    pub fn solve(&mut self) {
        let mut updates = self
            .state
            .iter()
            .enumerate()
            .filter(|(_, c)| c.result().is_some())
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        self.collapse(updates);

        while let Some(i) = self.lowest_entropy() {
            updates = self.observe(i);
            self.collapse(updates);
        }
    }

    /// The number of backtracks the solution required
    pub fn backtracks(&self) -> usize {
        self.backtracks
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..self.state.len() {
            write!(f, "{:#5?} ", self.state[i])?;

            if (i + 1) % 9 == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..self.state.len() {
            write!(f, "{:?} ", self.state[i])?;

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
