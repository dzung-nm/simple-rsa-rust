mod encryption_exponent;
mod generate_prime_pair;
mod miller_rabin;
mod pkcs1_padding;
pub mod rsa;

// Suppress panic output for all tests
#[cfg(test)]
#[ctor::ctor(unsafe)]
fn init_test() {
    std::panic::set_hook(Box::new(|_| {}));
}

