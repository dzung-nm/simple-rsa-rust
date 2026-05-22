// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

use rand::prelude::*;
use crate::mod_exp::mod_exp;

// Returns true if n is probably prime, false if n is composite
pub fn miller_rabin(n: i64) -> bool {
    if n <= 4 {
        panic!("n must be greater than 4");
    }

    // factoring out powers of 2 from n − 1
    // n − 1 = 2^s * d, where d is odd
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::rng();
    let k = 10; // the number of rounds of testing

    for _ in 0..k {
        let a = rng.random_range(2..=(n - 2)); // randomly chosen base in the range [2, n − 2]
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..s - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
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
    use std::time::Instant;
    use crate::miller_rabin::*;

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
    fn test_miller_rabin() {
        let start = Instant::now();
        let n = 1_000_000;
        let primes = get_base_primes(n);
        for i in 5..n {
            let is_prime = primes.binary_search(&i);
            match is_prime {
                Ok(..) => assert_eq!(miller_rabin(i), true, "{} should be a prime", i),
                Err(_) => assert_eq!(miller_rabin(i), false, "{} should not be a prime", i),
            }
        }
        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
    }
}
