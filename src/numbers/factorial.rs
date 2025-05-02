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
/// ```
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

#[cfg(test)]
mod tests {
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
