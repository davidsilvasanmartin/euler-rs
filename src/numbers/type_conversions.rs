use num_bigint::BigUint;
use num_traits::ToPrimitive;

/// Converts a BigUint to i128 if it fits within the positive range of i128.
///
/// Panics if the BigUint value is greater than i128::MAX.
///
/// # Arguments
///
/// * `num` - The BigUint number to convert.
///
/// # Returns
///
/// The i128 representation of the number.
///
/// # Panics
///
/// Panics if `num` > i128::MAX.
pub fn biguint_to_i128(num: &BigUint) -> i128 {
    num.to_i128()
        .unwrap_or_else(|| panic!("BigUint value {:?} exceeds i128::MAX", num))
}

#[cfg(test)]
mod test_biguint_to_i128 {
    use super::biguint_to_i128;
    use num_bigint::{BigUint, ToBigUint};
    use num_traits::Zero;
    use std::str::FromStr;

    #[test]
    fn test_convert_zero() {
        let zero_biguint = BigUint::zero();
        assert_eq!(biguint_to_i128(&zero_biguint), 0i128);
    }

    #[test]
    fn test_convert_small() {
        let small_biguint = 12345u64.to_biguint().unwrap();
        assert_eq!(biguint_to_i128(&small_biguint), 12345i128);
    }

    #[test]
    fn test_convert_max_i128() {
        let max_i128_val = i128::MAX;
        let max_i128_biguint = BigUint::from_str(&max_i128_val.to_string()).unwrap();
        assert_eq!(biguint_to_i128(&max_i128_biguint), max_i128_val);
    }

    #[test]
    #[should_panic(expected = "exceeds i128::MAX")]
    fn test_panic_just_above_max_i128() {
        let max_i128_val = i128::MAX;
        let max_i128_biguint = BigUint::from_str(&max_i128_val.to_string()).unwrap();
        let one = BigUint::from(1u32);
        let just_above_max = max_i128_biguint + one;

        // This call should panic
        let _ = biguint_to_i128(&just_above_max);
    }

    #[test]
    #[should_panic(expected = "exceeds i128::MAX")]
    fn test_panic_large_biguint() {
        let large_num_str = "9999999999999999999999999999999999999999999999999";
        let very_large_biguint = BigUint::from_str(large_num_str).unwrap();

        // This call should panic
        let _ = biguint_to_i128(&very_large_biguint);
    }
}
