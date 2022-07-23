pub mod cell;
pub mod naive;
pub mod wfc;

use std::{env::args, error::Error, time::SystemTime};

use naive::SudokuSolver;
use wfc::Board;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let naive = SudokuSolver::try_new(&args[1])?;
    let mut wfc = Board::try_new(&args[1])?;

    println!("Unsolved:\n{}", naive);

    let start_time = SystemTime::now();
    wfc.solve();
    let elapsed = start_time.elapsed()?;
    println!(
        "WFC | {} backtracks | {:.2} seconds\n{}",
        wfc.backtracks(),
        elapsed.as_secs_f64(),
        wfc
    );

    let start_time = SystemTime::now();
    let solved = naive.solve()?;
    let elapsed = start_time.elapsed()?;
    println!(
        "Naive | {} backtracks | {:.2} seconds\n{}",
        solved.1,
        elapsed.as_secs_f64(),
        solved
    );

    Ok(())
}
