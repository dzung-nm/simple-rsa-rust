use rand::prelude::*;
use crate::miller_rabin::miller_rabin;

// Generate a prime pair: (p, q) so that
//   p and q are both prime numbers
//   p and q are of the same bit length, in range [2^(bits-1), 2^bits)
//   q > p and distance between p and q is not too close
pub fn generate_prime_pair(bits: u64) -> (i64, i64) {
    if bits < 4 || bits > 126 {
        panic!("Bits size must be between 4 and 126 to prevent overflow and underflow");
    }

    let mut rng = rand::rng();

    let lower_bound = 1 << (bits - 1); // 2^(bits-1)
    let midpoint = lower_bound + (1 << (bits - 2)); // 2^(bits-1) + 2^(bits-2)
    let distance = 1 << (bits - 3); // 2^(bits-3)

    let mut p = rng.random_range(lower_bound..midpoint) | 1;
    while !miller_rabin(p) {
        p += 2;
    }

    let mut q = p + distance;
    while !miller_rabin(q) {
        q += 2;
    }

    (p, q)
}

#[cfg(test)]
mod tests {
    use crate::miller_rabin::miller_rabin;
    use crate::generate_prime_pair::generate_prime_pair;

    #[test]
    fn test_generate_prime_pair() {
        let bits: u64 = 16;
        for _ in 0..100 {
            let (p, q) = generate_prime_pair(bits);
            assert_eq!(miller_rabin(p), true, "p should be prime");
            assert_eq!(miller_rabin(q), true, "q should be prime");
            assert!(((1 << (bits - 1))..(1 << bits)).contains(&q));
            assert!(((1 << (bits - 1))..(1 << bits)).contains(&p));
            assert_eq!(q > p, true, "q should be greater than p");
            println!("Generated prime pair: p = {}, q = {}", p, q);
        }
    }
}
