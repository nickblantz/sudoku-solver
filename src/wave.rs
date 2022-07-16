use std::{
    fmt::{self, Debug},
    convert::{TryFrom, TryInto},
    // collections::{BinaryHeap, HashSet, HashMap},
};

use rand::{prelude::ThreadRng, Rng};


const ENTROPY_1: u16 = 0b0_0000_0001;
const ENTROPY_2: u16 = 0b0_0000_0010;
const ENTROPY_3: u16 = 0b0_0000_0100;
const ENTROPY_4: u16 = 0b0_0000_1000;
const ENTROPY_5: u16 = 0b0_0001_0000;
const ENTROPY_6: u16 = 0b0_0010_0000;
const ENTROPY_7: u16 = 0b0_0100_0000;
const ENTROPY_8: u16 = 0b0_1000_0000;
const ENTROPY_9: u16 = 0b1_0000_0000;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entropy(u16);

impl Default for Entropy {
    fn default() -> Self {
        Self(0b1_1111_1111)
    }
}

impl fmt::Display for Entropy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self.0 {
            ENTROPY_1 => write!(f, "{:#5}", 1)?,
            ENTROPY_2 => write!(f, "{:#5}", 2)?,
            ENTROPY_3 => write!(f, "{:#5}", 3)?,
            ENTROPY_4 => write!(f, "{:#5}", 4)?,
            ENTROPY_5 => write!(f, "{:#5}", 5)?,
            ENTROPY_6 => write!(f, "{:#5}", 6)?,
            ENTROPY_7 => write!(f, "{:#5}", 7)?,
            ENTROPY_8 => write!(f, "{:#5}", 8)?,
            ENTROPY_9 => write!(f, "{:#5}", 9)?,
            x => write!(f, "{:#05x}", x)?,
        }

        Ok(())
    }
}

impl TryFrom<(usize, char)> for Entropy {
    type Error = ParseError;

    fn try_from(c: (usize, char)) -> Result<Self, Self::Error> {
        match c.1 {
            '.' => Ok(Self::default()),
            '1' => Ok(Entropy::new(1)),
            '2' => Ok(Entropy::new(2)),
            '3' => Ok(Entropy::new(3)),
            '4' => Ok(Entropy::new(4)),
            '5' => Ok(Entropy::new(5)),
            '6' => Ok(Entropy::new(6)),
            '7' => Ok(Entropy::new(7)),
            '8' => Ok(Entropy::new(8)),
            '9' => Ok(Entropy::new(9)),
            _ => Err(ParseError::InvalidInput(c.0, c.1)),
        }
    }
}

impl Entropy {
    fn new(n: u32) -> Self {
        Self(1 << (n - 1))
    }

    fn converge(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    fn possibilities(&self) -> Vec<Self> {
        (1..=9)
            .filter(|n| self.0 & 1 << (n - 1) != 0)
            .map(|n| Self::new(n))
            .collect()
    }

    fn collapse(self, rng: &mut ThreadRng) -> Self {
        let possibilities = self.possibilities();
        println!("len {}", possibilities.len());
        possibilities[rng.gen_range(0..possibilities.len())]
    }

    // fn collapse(self) -> Self {
    //     Self::new(16 - self.0.leading_zeros())
    // }

    fn info(&self) -> u32 {
        self.0.count_ones()
    }

    fn is_collapsed(&self) -> bool {
        self.info() == 1
    }
}

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
            Self::InvalidSize(i) => write!(f, "A board was provided with an invalid length of {}", i)?,
            Self::InvalidInput(i, c) => write!(f, "Character {} at position {} is invalid", c, i)?,
            Self::InternalError => write!(f, "An internal error has occurred")?
        }

        Ok(())
    }
}

pub struct Board {
    board: [Entropy; 81],
    updates: Vec<usize>,
    rng: ThreadRng,
}

impl Board {
    fn col_accessor(i: usize, x: usize) -> usize {
        (i / 9) * 9 + x
    }

    fn row_accessor(i: usize, x: usize) -> usize {
        (i % 9) + x * 9
    }

    fn sect_accessor(i: usize, x: usize) -> usize {
        (i / (27)) * (27) + (i % 9 / 3) * 3 + x
    }

    fn col_iter() -> impl Iterator<Item = usize> {
        0..9
    }

    fn row_iter() -> impl Iterator<Item = usize> {
        0..9
    }

    fn sect_iter() -> impl Iterator<Item = usize> {
        [
            00, 01, 02,
            09, 10, 11,
            18, 19, 20
        ].iter().map(ToOwned::to_owned)
    }
    
    fn maybe_collapse<I: Iterator<Item = usize>>(
        &self,
        i: usize,
        iter: fn() -> I,
        accessor: fn(usize, usize) -> usize,
        entropy: &mut Entropy,
    ) {
        for x in iter() {
            let accessor = accessor(i, x);
            
            if accessor == i {
                continue;
            }

            if self.board[accessor].is_collapsed() {
                *entropy = entropy.converge(self.board[accessor]);
            }
        }
    }

    fn collapse(&mut self, i: usize) {
        println!("collapsing {} ({})", i, self.board[i]);
        let mut entropy = Entropy::default();

        self.maybe_collapse(i, Self::col_iter, Self::col_accessor, &mut entropy);
        self.maybe_collapse(i, Self::row_iter, Self::row_accessor, &mut entropy);
        self.maybe_collapse(i, Self::sect_iter, Self::sect_accessor, &mut entropy);

        println!("checked entropy {:#011b} (pos {})", entropy.0, i);
        self.board[i] = entropy.collapse(&mut self.rng);
        // self.board[i] = self.board[i].collapse(&mut self.rng);

    }

    /// update entropy for the column, row, and sector
    fn maybe_converge<I: Iterator<Item = usize>>(
        &mut self,
        i: usize,
        iter: fn() -> I,
        accessor: fn(usize, usize) -> usize,
    ) {
        for x in iter() {
            let accessor = accessor(i, x);
            
            if accessor == i || self.board[accessor].is_collapsed() {
                continue;
            }

            self.board[accessor] = self.board[accessor].converge(self.board[i]);
            
            if self.board[accessor].is_collapsed() {
                self.collapse(accessor);
                println!("{accessor} has collapsed");
                println!("{self}");
                // self.converge(accessor);
                self.updates.push(accessor);
            }
        }
    }

    /// update entropy for the column, row, and sector
    fn converge(&mut self, i: usize) {
        println!("converging {} ({})", i, self.board[i]);
        self.maybe_converge(i, Self::col_iter, Self::col_accessor);
        self.maybe_converge(i, Self::row_iter, Self::row_accessor);
        self.maybe_converge(i, Self::sect_iter, Self::sect_accessor);
        println!("{self}");
    }

    fn propagate(&mut self) {
        while let Some(i) = self.updates.pop() {
            println!("propagating {i}");
            self.converge(i);
        }
    }

    fn lowest_entropy(&self) -> Option<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, &entropy)| if entropy.is_collapsed() {
                None
            } else {
                Some((i, entropy))
            })
            .min_by(|a, b| a.1.info().cmp(&b.1.info()))
            .map(|(i, _)| i)
    }

    pub fn try_new(raw: &String) -> Result<Self, ParseError> {
        if raw.len() != 81 {
            return Err(ParseError::InvalidSize(raw.len()));
        }

        match raw
            .chars()
            .enumerate()
            .map(Entropy::try_from)
            .collect::<Result<Vec<Entropy>, ParseError>>()?
            .try_into()
        {
            Ok(board) => Ok(Self {
                board,
                updates: vec![],
                rng: rand::thread_rng(),
            }),
            Err(_) => Err(ParseError::InternalError),
        }
    }

    pub fn solve(&mut self) {
        for i in 0..self.board.len() {
            if self.board[i].is_collapsed() {
                self.converge(i);
            }
        }

        self.propagate();

        while let Some(i) = self.lowest_entropy() {
            self.collapse(i);
            self.converge(i);
            self.propagate();
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..self.board.len() {
            write!(f, "{} ", self.board[i])?;

            if (i + 1) % 9 == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..self.board.len() {
            write!(f, "{} ", self.board[i])?;

            if (i + 1) % 9 == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

trait Ent {
    fn new(n: u32) -> Self;

    fn converge(self, other: Self) -> Self;

    fn collapse(self) -> Self;

    fn info(&self) -> u32;

    fn is_collapsed(&self) -> bool;
}

impl Ent for u16 {
    fn new(n: u32) -> Self {
        1 << (n - 1)
    }

    fn converge(self, other: Self) -> Self {
        self & !other
    }

    fn collapse(self) -> Self {
        Self::new(16 - self.leading_zeros())
    }

    fn info(&self) -> u32 {
        self.count_ones()
    }

    fn is_collapsed(&self) -> bool {
        self.info() == 1
    }
}

struct Solution<T, const N: usize> {
    wave: [T; N],
    state_count: usize,
    weights: [f64; N],
    env: [T; N]
}