use num_bigint::BigUint;
use std::fmt;
use std::time::Instant;

pub mod p0020;
pub mod p0025;

/// Enum to represent the possible result types of a problem
#[derive(Debug)]
pub enum ProblemResult {
    I128(i128),
    U128(u128),
    BigUint(BigUint),
}

impl fmt::Display for ProblemResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProblemResult::I128(val) => write!(f, "{}", val),
            ProblemResult::U128(val) => write!(f, "{}", val),
            ProblemResult::BigUint(val) => write!(f, "{}", val),
        }
    }
}

pub trait Problem {
    /**
    The core logic function. Must be implemented by each problem.

    NOTE: This method is intended for internal use by the `solve_and_report`
    method. Users of the trait should typically call `solve_and_report`.
     */
    #[doc(hidden)]
    fn run(&self) -> ProblemResult;

    /**
    Runs the problem's solution, measures execution time, and prints the result.
    This is the intended entry point for solving a problem.
    */
    fn solve_and_report(&self) {
        println!("Solving...");
        let start_time = Instant::now();
        let solution = self.run();
        let elapsed_time = start_time.elapsed();

        println!("--------------------");
        println!("Solution: {}", solution);
        println!("Time elapsed: {:?}", elapsed_time);
        println!("--------------------");
    }
}
