use num_bigint::BigInt;
use num_traits::One;

use crate::encryption_exponent::encryption_exponent;
use crate::generate_prime_pair::generate_prime_pair;
use crate::pkcs1_padding::pkcs1_padding;

pub struct PublicKey {
    n: BigInt,
    e: BigInt,
}

pub struct PrivateKey {
    n: BigInt,
    d: BigInt,
}

/// Generates a pair of RSA keys (public and private) based on the specified bit length.
pub fn new_keys(bits: usize) -> (PublicKey, PrivateKey) {
    if bits < 512 {
        panic!("Bits size must be at least 512 to ensure a reasonable security level");
    }

    if bits > 2048 {
        panic!("Bits size is too large for this implementation");
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
    if message.len() == 0 {
        panic!("Empty message");
    }

    let n_bits_len = public_key.n.bits();
    let padded_message = pkcs1_padding(message, ((n_bits_len + 7) / 8) as usize);

    let m = BigInt::from_bytes_be(num_bigint::Sign::Plus, &padded_message);
    if m >= public_key.n {
        panic!("Message is too long for the given key size");
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
    #[should_panic = "Bits size must be at least 512 to ensure a reasonable security level"]
    fn test_new_keys_bits_too_small_panic() {
        new_keys(511);
    }

    #[test]
    #[should_panic = "Bits size is too large for this implementation"]
    fn test_new_keys_bits_too_large_panic() {
        new_keys(2049);
    }

    #[test]
    fn test_new_keys() {
        let bits = 512;
        for _ in 0..10 {
            let (public_key, private_key) = new_keys(bits);
            let m = BigInt::from(25);
            let c = m.modpow(&public_key.e, &public_key.n);
            let m_decrypted = c.modpow(&private_key.d, &private_key.n);
            assert_eq!(m, m_decrypted);
        }
    }

    #[test]
    #[should_panic = "Message is too long for the given key size"]
    fn test_encrypt_message_too_long_panic() {
        let message = b"This message is too long for a 512-bit key\
        This message is too long for a 512-bit key"
            .to_vec();
        let (public_key, _) = new_keys(512);
        encrypt(&message.to_vec(), &public_key);
    }

    #[test]
    #[should_panic = "Empty message"]
    fn test_encrypt_message_empty_panic() {
        let message = b"".to_vec();
        let (public_key, _) = new_keys(512);
        encrypt(&message.to_vec(), &public_key);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let message = b"cool!".to_vec();
        let bits_sizes = [512, 1024];
        for bits in bits_sizes {
            let (public_key, private_key) = new_keys(bits);
            let encrypted = encrypt(&message, &public_key);
            let decrypted = decrypt(&encrypted, &private_key);
            assert_eq!(message, decrypted);
        }
    }
}
