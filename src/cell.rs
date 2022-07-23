use bitvec::{array::BitArray, BitArr};
use rand::prelude::{SliceRandom, ThreadRng};
use std::fmt;

/// Number of possible states
pub const STATES: usize = 9;

/// The state of a cell at a given time
pub type CellState = BitArr!(for STATES, in u16);

#[derive(Clone)]
/// Represents the possible states of a Sudoku cell
pub struct Cell {
    /// The state of the cell
    state: CellState,

    /// The result of the cell
    result: Option<usize>,
}

impl Cell {
    /// Collapses a cell by removing states
    pub fn collapse(&mut self, state: CellState) {
        self.state = self.state & !state;
        self.result = match state.count_zeros() {
            1 => Some(self.state.first_one().unwrap()),
            _ => None,
        };
    }

    /// Sets the cell to its largest possible state
    pub fn solve_rng(&self, rng: &mut ThreadRng) -> Option<Self> {
        self.state
            .iter_ones()
            .collect::<Vec<usize>>()
            .choose(rng)
            .map(ToOwned::to_owned)
            .map(Self::from)
    }

    /// Updates the result for an fully collapsed cell
    pub fn solve(&self) -> Self {
        assert!(self.entropy() == 1);

        Self::from(self.state.first_one().unwrap())
    }

    /// The number of possible states in the cell's superposition
    pub fn entropy(&self) -> usize {
        self.state.count_ones()
    }

    /// The result of the cell
    pub fn result(&self) -> Option<usize> {
        self.result
    }

    /// The state of the cell
    pub fn state(&self) -> CellState {
        self.state
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: BitArray::new([0b1_1111_1111]),
            result: None,
        }
    }
}

impl From<usize> for Cell {
    fn from(n: usize) -> Self {
        Self {
            state: {
                let mut bits = BitArray::ZERO;
                bits.set(n, true);
                bits
            },
            result: Some(n),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.result {
            Some(i) => write!(f, "{}", i + 1)?,
            None => write!(f, "({})", self.entropy())?,
        }

        Ok(())
    }
}
