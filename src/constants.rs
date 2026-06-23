// The minimum and maximum bit sizes for RSA key generation.
// These values can be adjusted based on security requirements and performance considerations.
pub const RSA_MIN_BITS: usize = 512;
pub const RSA_MAX_BITS: usize = 4096;

// the number of rounds of Miller Rabin testing to determine if a number is prime
pub const MILLER_RABIN_ROUNDS: usize = 20;

// the number of threads to use for prime generation
pub const PRIME_GENERATION_THREADS: usize = 8;
