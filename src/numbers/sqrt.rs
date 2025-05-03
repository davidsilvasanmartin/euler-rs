use num_bigint::BigUint;
use num_traits::{One, Zero};

// TODO

// Function to compute the square root of a BigUint
fn sqrt(n: &BigUint) -> u32 {
    // Binary search to approximate the square root
    if *n == BigUint::zero() {
        return 0;
    }
    if *n == BigUint::one() {
        return 1;
    }
    n.sqrt();

    let mut low = 1u32;
    let mut high = u32::MAX; // Begin with the maximal u32 value
    let mut mid;

    while low <= high {
        mid = (low + high) / 2;
        let mid_squared: BigUint = BigUint::from(mid) * BigUint::from(mid);

        if mid_squared == *n {
            return mid; // Found exact square root
        } else if mid_squared < *n {
            low = mid + 1; // Search the upper half
        } else {
            high = mid - 1; // Search the lower half
        }
    }
    high // `high` will be the floor(sqrt(n)) when the loop ends
}
