use num_bigint::BigInt;
use num_traits::One;

use crate::encryption_exponent::encryption_exponent;
use crate::generate_prime_pair::generate_prime_pair;
use crate::pkcs1_padding::pkcs1_padding;

pub struct PublicKey {
    pub n: BigInt,
    pub e: BigInt,
}

pub struct PrivateKey {
    pub n: BigInt,
    pub d: BigInt,
}

/// Generates a pair of RSA keys (public and private) based on the specified bit length.
pub fn new_keys(bits: usize) -> (PublicKey, PrivateKey) {
    if bits < 16 {
        panic!("Bits size must be at least 16 to ensure a reasonable security level");
    }

    let one = BigInt::one();

    let (p, q) = generate_prime_pair(bits / 2);

    let n = &p * &q;
    let phi = (&p - &one) * (&q - &one);

    let e = encryption_exponent(&phi); // e and phi should be coprime
    let d = e.modinv(&phi).unwrap();

    let public_key = PublicKey { n: n.clone(), e };
    let private_key = PrivateKey { n, d };

    (public_key, private_key)
}

pub fn encrypt(message: &Vec<u8>, public_key: &PublicKey) -> BigInt {
    let n_bits_len = public_key.n.bits();
    let padded_message = pkcs1_padding(message, ((n_bits_len + 7) / 8) as usize);
    let m = BigInt::from_bytes_be(num_bigint::Sign::Plus, &padded_message);
    if m >= public_key.n {
        panic!("Message is too long for the given key size");
    }
    if m <= BigInt::from(1) {
        panic!("Message must be greater than 1");
    }
    BigInt::modpow(&m, &public_key.e, &public_key.n)
}

pub fn decrypt(ciphertext: &BigInt, private_key: &PrivateKey) -> Vec<u8> {
    let m = BigInt::modpow(ciphertext, &private_key.d, &private_key.n);
    let (_, padded_message) = m.to_bytes_be();
    // Remove PKCS#1 padding
    let padding_length = padded_message.iter().position(|&x| x == 0).unwrap_or(0);
    padded_message[padding_length + 1..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let message = b"hello world";
        let bits = 512;
        let (public_key, private_key) = new_keys(bits);
        let encrypted = encrypt(&message.to_vec(), &public_key);
        let decrypted = decrypt(&encrypted, &private_key);
        assert_eq!(message.to_vec(), decrypted);
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic1() {
        let message = b"hello world";
        let bits = 16;
        let (public_key, _) = new_keys(bits);
        encrypt(&message.to_vec(), &public_key);
    }

    #[test]
    #[should_panic]
    fn test_encrypt_panic2() {
        let bits = 64;
        let (public_key, _) = new_keys(bits);
        encrypt(&[1].to_vec(), &public_key);
    }
}
