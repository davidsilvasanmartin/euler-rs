use lazy_static::lazy_static;
use num_bigint::{BigUint, ToBigUint};
use rayon::prelude::*;

/// List of primes as a string. See https://t5k.org/lists/small/millions/
const PRIMES_STR: &str = include_str!("./primes.txt");

lazy_static! {
    /// Lazily evaluated static vector of the first prime numbers as BigUint
    pub static ref PRIMES: Vec<BigUint> = {
        // We need to collect the lines into a Vec first, so that order is preserved
        let lines: Vec<&str> = PRIMES_STR.lines().collect();
        lines
        // par_iter preserves the order
        .par_iter()
        .map(|line| {
            line.parse::<BigUint>().expect("Failed to parse prime number")
        })
        .collect()
    };
}

/// Returns the next odd number after the largest prime in the PRIMES sieve.
pub fn get_trial_division_start() -> BigUint {
    let last_prime_plus_2 = PRIMES.last().unwrap() + 2u64;
    last_prime_plus_2.to_biguint().unwrap()
}
