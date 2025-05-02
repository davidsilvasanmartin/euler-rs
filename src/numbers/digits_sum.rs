use num_bigint::BigUint;
use num_traits::Zero;

/// Calculates the sum of the digits of a number
///
/// Handles negative numbers by summing the digits of their absolute value.
/// Returns 0 for input 0
pub fn digits_sum(num: i128) -> i128 {
    // Use the absolute value to handle negative numbers correctly
    if num >= i128::MAX {
        panic!("Input must be less than i128::MAX");
    }
    if num <= i128::MIN {
        panic!("Input must be greater than i128::MIN");
    }

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

/// Calculates the sum of the digits of a BigUint number.
///
/// Returns BigUint::zero() for input BigUint::zero().
pub fn digits_sum_biguint(num: &BigUint) -> BigUint {
    let mut n = num.clone(); // Clone the number to modify it
    let mut sum = BigUint::zero();
    let ten = BigUint::from(10u32); // Constant for division/modulo
    let zero = BigUint::zero(); // Constant for comparison

    while n > zero {
        let last_digit = &n % &ten;
        sum += last_digit;
        // Remove the last digit using integer division
        n /= &ten;
    }

    sum
}

#[cfg(test)]
mod test_digits_sum {
    use super::digits_sum;

    #[test]
    fn test_zero() {
        assert_eq!(digits_sum(0), 0);
    }

    #[test]
    fn test_positive_number() {
        assert_eq!(digits_sum(123), 6);
        assert_eq!(digits_sum(9876), 30);
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(digits_sum(-123), 6);
        assert_eq!(digits_sum(-9876), 30);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(digits_sum(5), 5);
        assert_eq!(digits_sum(-8), 8);
    }

    #[test]
    fn test_large_number() {
        let large_positive: i128 = 123456789012345678901234567890;
        assert_eq!(digits_sum(large_positive), 135);

        let large_negative: i128 = -111111111111111111111111111111;
        assert_eq!(digits_sum(large_negative), 30);
    }

    #[test]
    #[should_panic(expected = "Input must be less than i128::MAX")]
    fn test_panic_on_max_i128() {
        digits_sum(i128::MAX);
    }

    #[test]
    #[should_panic(expected = "Input must be greater than i128::MIN")]
    fn test_panic_on_min_i128() {
        digits_sum(i128::MIN);
    }
}

#[cfg(test)]
mod test_digits_sum_biguint {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_digits_sum_biguint_zero() {
        assert_eq!(digits_sum_biguint(&BigUint::zero()), BigUint::zero());
    }

    #[test]
    fn test_digits_sum_biguint_small() {
        assert_eq!(
            digits_sum_biguint(&BigUint::from(123u32)),
            BigUint::from(6u32)
        );
    }

    #[test]
    fn test_digits_sum_biguint_large() {
        let large_num_str = "123456789012345678901234567890";
        let expected_sum = 135u128;
        let large_num = large_num_str.parse::<BigUint>().unwrap();
        assert_eq!(digits_sum_biguint(&large_num), BigUint::from(expected_sum));
    }

    #[test]
    fn test_digits_sum_biguint_single_digit() {
        assert_eq!(
            digits_sum_biguint(&BigUint::from(7u32)),
            BigUint::from(7u32)
        );
    }
}
