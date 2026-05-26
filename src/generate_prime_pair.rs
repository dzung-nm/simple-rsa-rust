use num_bigint::{BigInt, RandBigInt};
use num_traits::One;

use crate::miller_rabin::miller_rabin_test;

/// Generate a pair of primes (p, q) in 'bits' size
/// p < q, and both should be in range (2^(bits-1), 2^bits)
pub fn generate_prime_pair(bits: usize) -> (BigInt, BigInt) {
    if bits < 4 {
        panic!("Bits size must be at least 4");
    }

    if bits > 1024 {
        panic!("Bits size is too large for this implementation");
    }

    let mut rng = rand::thread_rng();

    let lower_bound = BigInt::one() << (bits - 1); // 2^(bits-1)
    let midpoint = &lower_bound + (BigInt::one() << (bits - 2)); // 2^(bits-1) + 2^(bits-2)
    // let upper_bound = BigInt::one() << bits; // 2^bits
    let distance = BigInt::one() << (bits - 3); // 2^(bits-3)

    let mut p = rng.gen_bigint_range(&lower_bound, &midpoint) | BigInt::one();
    while !miller_rabin_test(&p) {
        p += 2; // Todo: This can be improved to increase performance.
    }

    let mut q = &p + distance;
    while !miller_rabin_test(&q) {
        q += 2; // Todo: This can be improved to increase performance.
    }

    (p, q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_generate_prime_pair() {
        let bits_sizes = [4, 8, 16, 32, 64, 128, 256, 512, 1024];
        for bits in bits_sizes {
            let start = Instant::now();
            let (p, q) = generate_prime_pair(bits);
            let duration = start.elapsed();

            println!("--------------------------------------------------");
            println!("generate_prime_pair(bits={}) took {:?}", bits, duration);
            println!("The bit length of n = p * q is {}", (&p * &q).bits());
            println!("Generated prime pair: p = {}, q = {}", p, q);

            assert_eq!(miller_rabin_test(&p), true, "p should be prime");
            assert_eq!(miller_rabin_test(&q), true, "q should be prime");
            assert!(
                ((BigInt::one() << (bits - 1))..(BigInt::one() << bits)).contains(&q),
                "q should be in range (2^(bits-1), 2^bits)"
            );
            assert!(
                ((BigInt::one() << (bits - 1))..(BigInt::one() << bits)).contains(&p),
                "p should be in range (2^(bits-1), 2^bits)"
            );
            assert_eq!(q > p, true, "q should be greater than p");
        }
    }

    #[test]
    #[should_panic = "Bits size is too large for this implementation"]
    fn test_bits_size_too_large_panic() {
        generate_prime_pair(1025);
    }

    #[test]
    #[should_panic = "Bits size must be at least 4"]
    fn test_bits_size_too_small_panic() {
        generate_prime_pair(3);
    }
}
