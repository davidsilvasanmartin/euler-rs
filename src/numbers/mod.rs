/**
Calculates the sum of the digits of a number

Handles negative numbers by summing the digits of their absolute value.
Returns 0 for input 0
*/
pub fn digits_sum(num: i128) -> i128 {
    // Use the absolute value to handle negative numbers correctly
    let mut n = num.abs();
    let mut sum = 0i128;

    while n > 0 {
        // Get the last digit using modulo 10
        sum += n % 10;
        // Remove the last digit using integer division
        n /= 10;
    }

    sum
}
