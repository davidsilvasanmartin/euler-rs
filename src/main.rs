/**
"use crate::problems::Problem" brings a specific item, the Problem trait, from
within the "problems" module into the current scope. It brings the definition
of the trait and the associated methods into scope. "crate" refers to the root
of this project. I need to import the Problem trait because I'm using a method
that belongs to that trait: "solve_and_report"
*/
use crate::problems::Problem;
use std::env;


mod numbers;
mod primes;
/**
"mod problems" is a module declaration. It tells the Rust compiler that
there is a module named "problems" and that it should look for its definition.
Based on Rust's module system rules and this project's structure, the compiler
finds the code for the "problems" module in "src/problems/mod.rs". The
boilerplate code defined in that mod.rs (including the submodules it
declares, such as p0020) is incorporated into this project's module tree,
making things inside the problems module accessible via the path "problems::...".
This line basically makes the namespace "problems" available.
*/
mod problems;
mod sequences;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <problem-number>", args[0]);
        std::process::exit(1);
    }

    match args[1].parse::<u32>() {
        Ok(20) => problems::p0020::Solution.solve_and_report(),
        Ok(25) => problems::p0025::Solution.solve_and_report(),
        Ok(num) => {
            eprintln!("Problem {} not implemented yet", num);
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("Invalid problem number");
            std::process::exit(1);
        }
    };
}
