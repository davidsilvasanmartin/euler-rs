use super::Problem;
use crate::numbers::digits_sum::digits_sum_biguint;
use crate::numbers::factorial::factorial_biguint;
use crate::numbers::type_conversions::biguint_to_i128;

pub struct Solution;

/**
https://projecteuler.net/problem=20
*/
impl Problem for Solution {
    fn run(&self) -> i128 {
        let fact_100 = factorial_biguint(100u128);
        let fact_100_digits_sum = digits_sum_biguint(&fact_100);
        biguint_to_i128(&fact_100_digits_sum)
    }
}
