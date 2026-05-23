use num_bigint::BigInt;
use num_traits::One;

use crate::encryption_exponent::encryption_exponent;
use crate::generate_prime_pair::generate_prime_pair;

pub struct PublicKey {
    n: BigInt,
    e: BigInt,
}

pub struct PrivateKey {
    n: BigInt,
    d: BigInt,
}

/// Generates a pair of RSA keys (public and private) based on the specified bit length.
/// ```
///     let bits = 1024;
///     let (public_key, private_key) = rsa::new_keys(bits);
///     assert_eq!(public_key.n, private_key.n);
/// ```
pub fn new_keys(bits: u16) -> (PublicKey, PrivateKey) {
    let one = BigInt::one();

    let (p, q) = generate_prime_pair(bits);

    let n = &p * &q;
    let phi = (&p - &one) * (&q - &one);

    let e = encryption_exponent(&phi); // e and phi should be coprime
    let d = e.modinv(&phi).unwrap();

    let public_key = PublicKey { n: n.clone(), e };
    let private_key = PrivateKey { n, d };

    (public_key, private_key)
}

pub fn encrypt(message: i64, public_key: &PublicKey) -> BigInt {
    BigInt::modpow(&message.into(), &public_key.e, &public_key.n)
}

pub fn decrypt(ciphertext: &BigInt, private_key: &PrivateKey) -> i64 {
    let decrypted = BigInt::modpow(ciphertext, &private_key.d, &private_key.n);
    decrypted.try_into().unwrap()
}
