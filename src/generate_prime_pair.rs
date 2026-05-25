use num_bigint::{BigInt, RandBigInt};
use num_traits::One;

use crate::miller_rabin::miller_rabin;

/// Generate a pair of primes (p, q) in 'bits' size
/// p < q, and both should be in range (2^(bits-1), 2^bits)
pub fn generate_prime_pair(bits: usize) -> (BigInt, BigInt) {
    if bits < 4 {
        panic!("Bits size must be at least 4 to ensure a reasonable distance between p and q");
    }

    if bits > 2048 {
        panic!("Bits size is too large for this implementation");
    }

    let mut rng = rand::thread_rng();

    let lower_bound = BigInt::one() << (bits - 1); // 2^(bits-1)
    let midpoint = &lower_bound + (BigInt::one() << (bits - 2)); // 2^(bits-1) + 2^(bits-2)
    // let upper_bound = BigInt::one() << bits; // 2^bits
    let distance = BigInt::one() << (bits - 3); // 2^(bits-3)

    let mut p = rng.gen_bigint_range(&lower_bound, &midpoint) | BigInt::one();
    while !miller_rabin(&p) {
        p += 2;
    }

    let mut q = &p + distance;
    while !miller_rabin(&q) {
        q += 2;
    }

    (p, q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::miller_rabin::miller_rabin;

    #[test]
    fn test_generate_prime_pair() {
        let bits = 128;
        for _ in 0..10 {
            let (p, q) = generate_prime_pair(bits);
            assert_eq!(miller_rabin(&p), true, "p should be prime");
            assert_eq!(miller_rabin(&q), true, "q should be prime");
            assert!(
                ((BigInt::one() << (bits - 1))..(BigInt::one() << bits)).contains(&q),
                "q should be in range (2^(bits-1), 2^bits)"
            );
            assert!(
                ((BigInt::one() << (bits - 1))..(BigInt::one() << bits)).contains(&p),
                "p should be in range (2^(bits-1), 2^bits)"
            );
            assert_eq!(q > p, true, "q should be greater than p");
            println!("Generated prime pair: p = {}, q = {}", p, q);
        }
    }
}
