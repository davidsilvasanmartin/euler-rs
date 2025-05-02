use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

/// Calculates the factorial of a non-negative integer: n!
///
/// Panics if the result overflows the u128 type (which occurs for n > 34)
///
/// # Arguments
///
/// * `n` - The number for which to calculate the factorial. Must be `u128`
///
/// # Returns
///
/// The factorial of `n` as `u128`
///
/// # Panics
///
/// Panics if `n` is greater than 34, as `35!` exceeds the maximum value of `u128`
///
/// # Examples
///
/// ```rust
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// ```should_panic
/// // This will panic because 35! overflows u128
/// let _ = factorial(35);
/// ```
pub fn factorial(n: u128) -> u128 {
    if n == 0 {
        return 1;
    }

    let mut result: u128 = 1;
    for i in 2..=n {
        // Use checked_mul to detect overflow
        result = match result.checked_mul(i) {
            Some(res) => res,
            None => {
                panic!("Factorial calculation overflowed for n={}", n)
            }
        }
    }

    result
}

/// Calculates, using BigUint, the factorial of a non-negative integer n: n!
///
/// # Arguments
///
/// * `n` - The number for which to calculate the factorial.
///
/// # Returns
///
/// The factorial of `n` as a `num_bigint::BigUint`.
///
/// # Examples
///
/// ```rust
/// use num_bigint::ToBigUint;
/// // Assuming factorial_biguint is the function name
/// assert_eq!(factorial_biguint(0).to_string(), "1");
/// assert_eq!(factorial_biguint(5).to_string(), "120");
/// assert_eq!(factorial_biguint(20).to_string(), "2432902008176640000");
/// // factorial_biguint(100) will produce a very large number
/// ```
pub fn factorial_biguint(n: u128) -> BigUint {
    if n == 0 {
        return BigUint::one();
    }

    let mut result = BigUint::one();

    for i in 2..=n {
        // Multiplication between BigUint uses the `*` operator
        // BigUint multiplication handles arbitrary size, so no overflow panic needed.
        result *= i.to_biguint().unwrap(); // Unwrap is safe as u128 always fits
    }

    result
}

#[cfg(test)]
mod test_factorial {
    use super::factorial;

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_factorial_small() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_factorial_medium() {
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn test_factorial_large() {
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn test_factorial_max_u128() {
        assert_eq!(
            factorial(34),
            295_232_799_039_604_140_847_618_609_643_520_000_000_u128
        );
    }

    #[test]
    #[should_panic(expected = "Factorial calculation overflowed for n=35")]
    fn test_factorial_overflow() {
        let _ = factorial(35);
    }

    #[test]
    #[should_panic] // Less specific panic check, useful if message changes
    fn test_factorial_overflow_generic() {
        // Test that calculating factorial(40) panics due to overflow (generic check)
        let _ = factorial(40);
    }
}

#[cfg(test)]
mod test_factorial_biguint {
    use super::factorial_biguint;
    use num_bigint::{BigUint, ToBigUint};
    // Required for BigUint::from_str
    use std::str::FromStr;

    #[test]
    fn test_biguint_zero() {
        assert_eq!(
            factorial_biguint(0),
            1u32.to_biguint().unwrap(),
            "Test Failed: BigUint Factorial of 0"
        );
    }

    #[test]
    fn test_biguint_one() {
        // Test 1! = 1
        assert_eq!(
            factorial_biguint(1),
            1u32.to_biguint().unwrap(),
            "Test Failed: BigUint Factorial of 1"
        );
    }

    #[test]
    fn test_biguint_small() {
        // Test 5! = 120
        assert_eq!(
            factorial_biguint(5),
            120u32.to_biguint().unwrap(),
            "Test Failed: BigUint Factorial of 5"
        );
    }

    #[test]
    fn test_biguint_medium() {
        let expected = BigUint::from_str("2432902008176640000").unwrap();
        assert_eq!(
            factorial_biguint(20),
            expected,
            "Test Failed: BigUint Factorial of 20"
        );
    }

    #[test]
    fn test_biguint_beyond_u128() {
        let expected = BigUint::from_str("10333147966386144929666651337523200000000").unwrap();
        assert_eq!(
            factorial_biguint(35),
            expected,
            "Test Failed: BigUint Factorial of 35 (overflows u128)"
        );
    }

    #[test]
    fn test_biguint_large() {
        let expected_str = "30414093201713378043612608166064768844377641568960512000000000000";
        let expected = BigUint::from_str(expected_str).unwrap();
        assert_eq!(
            factorial_biguint(50),
            expected,
            "Test Failed: BigUint Factorial of 50"
        );
    }

    // Note: We don't need a `should_panic` test here because `factorial_biguint`
    // is designed *not* to panic on overflow, as BigUint handles arbitrary size.
}
