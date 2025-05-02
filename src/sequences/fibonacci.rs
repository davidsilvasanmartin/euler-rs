use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct Fibonacci {
    curr: BigUint,
    next: BigUint,
    index: u128,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence iterator
    /// Starts with F_1 = 1, F_2 = 1
    pub fn new() -> Self {
        Fibonacci {
            // Start so that the first call to next() returns F_1 = 1
            curr: BigUint::zero(),
            next: BigUint::one(),
            index: 0,
        }
    }
}

impl Iterator for Fibonacci {
    // Item yielded is a tuple: (index, Fibonacci number)
    type Item = (u128, BigUint);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        // Standard Fibonacci calculation using swap
        let next_val = &self.curr + &self.next;
        self.curr = std::mem::replace(&mut self.next, next_val);

        Some((self.index, self.curr.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some((1, 1u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((2, 1u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((3, 2u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((4, 3u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((5, 5u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((6, 8u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((7, 13u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((8, 21u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((9, 34u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((10, 55u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((11, 89u32.to_biguint().unwrap())));
    }
}
