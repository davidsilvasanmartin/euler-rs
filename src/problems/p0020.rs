use super::{Problem, ProblemResult};
use crate::numbers::digits_sum::digits_sum_biguint;
use crate::numbers::factorial::factorial_biguint;

pub struct Solution;

/**
https://projecteuler.net/problem=20
*/
impl Problem for Solution {
    fn run(&self) -> ProblemResult {
        let fact_100 = factorial_biguint(100u128);
        let fact_100_digits_sum = digits_sum_biguint(&fact_100);
        ProblemResult::BigUint(fact_100_digits_sum)
    }
}
