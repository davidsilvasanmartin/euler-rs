use super::{Problem, ProblemResult};
use crate::sequences::fibonacci::Fibonacci;

pub struct Solution;

/**
https://projecteuler.net/problem=25
*/
impl Problem for Solution {
    fn run(&self) -> ProblemResult {
        let target_digits = 1000;
        let fib_iter = Fibonacci::new();

        for (index, fib_num) in fib_iter {
            if fib_num.to_string().len() >= target_digits {
                return ProblemResult::U128(index);
            }
        }

        panic!("Fibonacci iterator stopped unexpectedly");
    }
}
