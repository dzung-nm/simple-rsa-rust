use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, ToPrimitive};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::constants::*;
use crate::miller_rabin::miller_rabin_test;

fn find_prime_with_wheel(mut p: BigInt, stop_flag: Arc<AtomicBool>) -> Option<BigInt> {
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
        if stop_flag.load(Ordering::Relaxed) {
            return None;
        }

        let step = gaps[idx];
        p += step;
        idx = (idx + 1) % gaps.len();
    }

    Some(p)
}

/// Generate a pair of primes (p, q) in 'bits' size, satisfying the following conditions:
///  *) p < q, and both should be in range (2^(bits-1), 2^bits)
///  *) n = p * q should have a bit length of 2 * bits (exactly)
/// This version runs on multiple threads to speed up the prime generation process.
pub fn generate_prime_pair(bits: usize) -> (BigInt, BigInt) {
    if bits < 8 {
        panic!("Bits size must be at least 8");
    }

    if bits > RSA_MAX_BITS / 2 {
        panic!("Bits size is too large for this implementation");
    }

    let mut rng = rand::thread_rng();

    let lower_bound = (BigInt::one() << (bits - 1)) + (BigInt::one() << (bits - 2));
    let upper_bound = (BigInt::one() << bits) - BigInt::one();

    loop {
        // on my machine 8 threads seems to be optimal
        // adding more threads seems not improve performance
        let n_threads = 8;

        let rand_numbers: Vec<BigInt> = (0..n_threads)
            .map(|_| rng.gen_bigint_range(&lower_bound, &upper_bound) | BigInt::one())
            .collect();

        // This flag will be set to true if there are 2 enough primes
        // It helps other threads to stop searching for primes and exit early
        let stop_flag = Arc::new(AtomicBool::new(false));

        let results = Arc::new(Mutex::new(Vec::new()));
        let found_count = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];

        for start in rand_numbers {
            let stop_flag = Arc::clone(&stop_flag);
            let results = Arc::clone(&results);
            let found_count = Arc::clone(&found_count);

            let handle = thread::spawn(move || {
                if let Some(prime) = find_prime_with_wheel(start, stop_flag.clone()) {
                    let mut res = results.lock().unwrap();
                    res.push(prime);
                    drop(res);
                    let count = found_count.fetch_add(1, Ordering::SeqCst) + 1;
                    if count >= 2 {
                        stop_flag.store(true, Ordering::SeqCst);
                    }
                }
            });

            handles.push(handle);
        }

        // wait for all threads finish
        for handle in handles {
            handle.join().unwrap();
        }

        let primes = results.lock().unwrap();

        if primes.len() >= 2 {
            let p = primes[0].clone();
            let q = primes[1].clone();

            if p < q && q < upper_bound {
                return (p, q);
            }

            if q < p && p < upper_bound {
                return (q, p);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_generate_prime_pair() {
        let bits_sizes = [8, 9, 10, 16, 32, 64, 128, 256, 512];
        for bits in bits_sizes {
            let start = Instant::now();
            let (p, q) = generate_prime_pair(bits);
            let duration = start.elapsed();

            println!("--------------------------------------------------");
            println!("generate_prime_pair(bits={}) took {:?}", bits, duration);

            let n_bits = (&p * &q).bits();
            println!("The bit length of n = p * q is {}", n_bits);
            assert_eq!(n_bits as usize, bits * 2);

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
        generate_prime_pair(10000);
    }

    #[test]
    #[should_panic = "Bits size must be at least 8"]
    fn test_bits_size_too_small_panic() {
        generate_prime_pair(7);
    }
}
