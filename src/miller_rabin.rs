use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};

use crate::constants::*;

/// Returns true if n is probably prime, false if n is composite
/// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
pub fn miller_rabin_test(n: &BigInt) -> bool {
    let zero = BigInt::zero();
    let one = BigInt::one();
    let two = &zero + 2;
    let three = &zero + 3;

    if *n <= one {
        panic!("n must be greater than 1");
    }

    if *n <= three {
        return true;
    }

    if n % 2u8 == zero {
        return false;
    }

    let n_minus_one: BigInt = n - 1;

    // factoring out powers of 2 from n − 1
    // n − 1 = 2^s * d, where d is odd
    let mut d = n_minus_one.clone();
    let mut s = 0;
    while (&d & &one) == zero {
        d >>= 1;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..MILLER_RABIN_ROUNDS {
        // randomly chosen base in the range [2, n − 2]
        let a = rng.gen_bigint_range(&two, &n_minus_one);

        let mut x = BigInt::modpow(&a, &d, n);
        if x == one || x == n_minus_one {
            continue;
        }
        let mut composite = true;
        for _ in 0..(s - 1) {
            x = BigInt::modpow(&x, &two, n);
            if x == n_minus_one {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn get_base_primes(n: i64) -> Vec<i64> {
        let mut primes: Vec<i64> = Vec::new();
        for i in 2..n {
            let root = (i as f64).sqrt() as i64;
            let is_prime = primes
                .iter()
                .take_while(|&&p| p <= root)
                .all(|&p| i % p != 0);
            if is_prime {
                primes.push(i);
            }
        }
        primes
    }

    #[test]
    fn test_miller_rabin_test() {
        let start = Instant::now();
        let n = 20_000;
        let primes = get_base_primes(n);
        for i in 5..n {
            let is_prime = primes.binary_search(&i);
            match is_prime {
                Ok(..) => assert_eq!(
                    miller_rabin_test(&BigInt::from(i)),
                    true,
                    "{} should be a prime",
                    i
                ),
                Err(_) => assert_eq!(
                    miller_rabin_test(&BigInt::from(i)),
                    false,
                    "{} should not be a prime",
                    i
                ),
            }
        }
        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
    }

    #[test]
    #[should_panic = "n must be greater than 1"]
    fn test_n_invalid_panic() {
        miller_rabin_test(&BigInt::from(1));
    }
}
