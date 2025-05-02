use num_bigint::BigUint;
use num_traits::{One, Zero};

pub struct Fibonacci {
    prev: BigUint,
    curr: BigUint,
    index: u128,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence iterator
    /// Starts with F_1 = 1, F_2 = 1
    pub fn new() -> Self {
        Fibonacci {
            prev: BigUint::zero(),
            curr: BigUint::one(),
            index: 0,
        }
    }
}

impl Iterator for Fibonacci {
    // Item yielded is a tuple: (index, Fibonacci number)
    type Item = (u128, BigUint);

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        // Special case: the first time next() is called, return a hardcoded value
        // instead of starting calculations. The previous solution was holding F_{i+1}
        // in memory when returning F_i. This is far from ideal, because F_{i+1} may
        // never be used, but we are storing it in memory. The current solution just
        // keeps F_i and F_{i-1} in memory when returning F_i. To make this change
        // possible, I had to implement code for this special case when the index is 1
        if self.index == 1 {
            return Some((self.index, BigUint::one()));
        }

        // Standard Fibonacci calculation using swap
        let next_val = &self.prev + &self.curr;
        self.prev = std::mem::replace(&mut self.curr, next_val);

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
        assert_eq!(fib.next(), Some((12, 144u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((13, 233u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((14, 377u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((15, 610u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((16, 987u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((17, 1597u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((18, 2584u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((19, 4181u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((20, 6765u32.to_biguint().unwrap())));
        assert_eq!(fib.next(), Some((21, 10946u32.to_biguint().unwrap())));
    }
}
