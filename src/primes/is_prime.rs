use super::sieve::PRIMES;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;

/// Checks whether a BigUint is prime
fn is_prime(n: &BigUint) -> bool {
    let two = 2u32.to_biguint().unwrap();
    if *n < two {
        return false;
    }

    let sqrt_n = n.sqrt();

    // Check if n is one of the primes in the sieve or divisible by them
    for p in &*PRIMES {
        // n cannot have any smaller prime factors, so it must be prime
        if p > &sqrt_n {
            return true;
        }
        // If n is divisible by a prime in the sieve, it's composite
        if n % p == BigUint::zero() {
            return false;
        }
    }

    // If n is larger than the largest prime in the sieve (29),
    // continue with trial division up to the square root of n.

    // Start checking potential divisors from the number after the largest sieve prime.
    // We can start checking from 31 and only check odd numbers,
    // since we already checked for divisibility by 2.
    let mut divisor = PRIMES.last().unwrap().clone();
    let two_biguint = 2u32.to_biguint().unwrap();

    while divisor <= sqrt_n {
        if n % &divisor == BigUint::zero() {
            // If n is divisible by the current divisor, it's not prime
            return false;
        }
        // Increment the divisor by 2 to check the next odd number
        divisor += &two_biguint;
    }

    // If the number wasn't divisible by any number up to its square root, it's prime
    true
}

#[cfg(test)]
mod test_is_prime {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(&0.to_biguint().unwrap()), false);
        assert_eq!(is_prime(&1.to_biguint().unwrap()), false);
        assert_eq!(is_prime(&2.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&3.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&4.to_biguint().unwrap()), false);
        assert_eq!(is_prime(&5.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&6.to_biguint().unwrap()), false);
        assert_eq!(is_prime(&7.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&8.to_biguint().unwrap()), false);
        assert_eq!(is_prime(&9999991.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&9999997.to_biguint().unwrap()), false);
        assert_eq!(is_prime(&15485863.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&15485867.to_biguint().unwrap()), true);
        assert_eq!(is_prime(&15485877.to_biguint().unwrap()), false);
    }
}
