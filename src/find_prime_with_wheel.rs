use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::miller_rabin::miller_rabin_test;

///
/// Find the first prime number after 'seed' using a wheel factorization approach
/// with base primes = [2, 3, 5].
///
/// This function is designed to be used in a multithreaded context, where multiple
/// threads may be searching for prime numbers concurrently.
///
/// The function will return None if the stop_flag is set to true, indicating that
/// another thread has found enough primes and this thread should stop searching.
///
pub fn find_prime_with_wheel(seed: &BigInt, stop_flag: Arc<AtomicBool>) -> Option<BigInt> {
    let base_primes = [2, 3, 5];
    let product: i64 = base_primes.iter().product();

    if seed <= &BigInt::from(5) {
        panic!("seed is too small");
    }

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

    let mut start = seed.clone();

    // Align the initial candidate 'p' to the nearest valid wheel position
    let remainder = (&start % product).to_i64().unwrap();

    let mut idx = numbers
        .iter()
        .position(|&num| num >= remainder)
        .unwrap_or(0);

    if numbers[idx] < remainder {
        start += BigInt::from((product - remainder) + numbers[0]);
        idx = 0;
    } else if numbers[idx] > remainder {
        start += BigInt::from(numbers[idx] - remainder);
    }

    while !miller_rabin_test(&start) {
        if stop_flag.load(Ordering::Relaxed) {
            return None;
        }

        let step = gaps[idx];
        start += step;
        idx = (idx + 1) % gaps.len();
    }

    Some(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prime_with_wheel() {
        let mut start = BigInt::from(5);
        let stop_flag = Arc::new(AtomicBool::new(false));
        while start < BigInt::from(10000) {
            start += 1;
            let next = find_prime_with_wheel(&start, stop_flag.clone()).unwrap();
            while start < next {
                start += 1;
                assert_eq!(
                    find_prime_with_wheel(&start, stop_flag.clone()).unwrap(),
                    next
                );
            }
        }
    }

    #[test]
    #[should_panic = "seed is too small"]
    fn test_find_prime_with_panic() {
        find_prime_with_wheel(&BigInt::from(3), Arc::new(AtomicBool::new(false)));
    }
}
