pub mod extended_gcd;
pub mod miller_rabin;
pub mod mod_exp;
pub mod simple_rsa;
mod generate_prime_pair;

fn main() {
    simple_rsa::simple_rsa();
}
