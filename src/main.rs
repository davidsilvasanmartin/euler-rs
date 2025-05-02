use crate::problem::Problem;
use std::env;

mod p0020;
mod problem;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <problem-number>", args[0]);
        std::process::exit(1);
    }

    let solution: i128 = match args[1].parse::<u32>() {
        Ok(20) => p0020::Solution.run(),
        Ok(num) => {
            eprintln!("Problem {} not implemented yet", num);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Invalid problem number");
            std::process::exit(1);
        }
    };

    println!("SOLUTION:\n{}", solution);
}
