use super::Problem;
use crate::numbers::digits_sum::digits_sum;

pub struct Solution;

/**
https://projecteuler.net/problem=20
*/
impl Problem for Solution {
    fn run(&self) -> i128 {
        digits_sum(999999999999i128)
    }
}
