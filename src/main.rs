mod solver;

use solver::SodukoSolver;
use std::{error, time};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let unsolved = SodukoSolver::try_new(&args[1])?;
    println!("Unsolved:\n{}", unsolved);
    let start_time = time::SystemTime::now();
    let solved = unsolved.solve()?;
    let elapsed = start_time.elapsed()?;
    print!("Solved:\n{}\n", solved);
    println!("Time-to-solve: {} seconds", elapsed.as_secs_f64());
    Ok(())
}
