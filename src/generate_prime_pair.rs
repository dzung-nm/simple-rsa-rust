use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, ToPrimitive};

use crate::miller_rabin::miller_rabin_test;

fn find_prime_with_wheel(mut p: BigInt) -> BigInt {
    let base_primes = [2, 3, 5];
    let product: i64 = base_primes.iter().product();

    // Generate numbers coprime to the base primes within the range [1, product]
    // For [2, 3, 5], this yields: [1, 7, 11, 13, 17, 19, 23, 29]
    let numbers: Vec<i64> = (1..product)
        .filter(|&num| base_primes.iter().all(|&prime| num % prime != 0))
        .collect();

    // Pre-calculate the cyclic jump distances (gaps) between coprime numbers
    // For [2, 3, 5], gaps will be: [6, 4, 2, 4, 2, 4, 6, 2]
    let mut gaps = Vec::with_capacity(numbers.len());
    for i in 0..numbers.len() {
        let next_val = if i == numbers.len() - 1 {
            numbers[0] + product // Wrap around to the first element of the next cycle
        } else {
            numbers[i + 1]
        };
        gaps.push(next_val - numbers[i]);
    }

    // Align the initial candidate 'p' to the nearest valid wheel position
    let remainder = (&p % product).to_i64().unwrap();

    let mut idx = numbers
        .iter()
        .position(|&num| num >= remainder)
        .unwrap_or(0);

    if numbers[idx] < remainder {
        p += BigInt::from((product - remainder) + numbers[0]);
        idx = 0;
    } else if numbers[idx] > remainder {
        p += BigInt::from(numbers[idx] - remainder);
    }

    while !miller_rabin_test(&p) {
        let step = gaps[idx];
        p += step;
        idx = (idx + 1) % gaps.len();
    }

    p
}

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

    let p_start = rng.gen_bigint_range(&lower_bound, &midpoint) | BigInt::one();
    let p = find_prime_with_wheel(p_start);

    let q_start = &p + distance;
    let q = find_prime_with_wheel(q_start);

    (p, q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore]
    fn test_performance() {
        let k = 10; // number of rounds
        let bits = 1024;
        let mut total_time = 0.0;
        for _ in 1..=k {
            let start = Instant::now();
            generate_prime_pair(bits);
            total_time += start.elapsed().as_secs_f64();
        }
        println!("Average execution time: {:?}", total_time / k as f64);
    }

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
