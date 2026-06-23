mod constants;
mod encryption_exponent;
mod find_prime_with_wheel;
mod generate_prime_pair;
mod miller_rabin;
mod pkcs1_padding;
mod rsa;

pub use constants::*;
pub use rsa::{decrypt, encrypt, new_keys};

// Suppress panic output for all tests
#[cfg(test)]
#[ctor::ctor(unsafe)]
fn init_test() {
    std::panic::set_hook(Box::new(|_| {}));
}
