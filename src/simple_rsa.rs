use num_bigint::BigInt;
use num_traits::One;

use crate::encryption_exponent::encryption_exponent;
use crate::generate_prime_pair::generate_prime_pair;

pub fn simple_rsa() {
    let one = BigInt::one();

    let (p, q) = generate_prime_pair(512);

    let n: BigInt = &p * &q;
    let phi = (&p - &one) * (&q - &one);

    let e = encryption_exponent(&phi); // e and phi must be coprime
    let d = e.modinv(&phi).unwrap();

    // test
    let messages: [i64; 100] = std::array::from_fn(|i| i as i64 + 1);
    for message in messages {
        let m = BigInt::from(message);
        let ciphertext = m.modpow(&e, &n);
        let decrypted = ciphertext.modpow(&d, &n);
        assert_eq!(m, decrypted);
        println!("{:?} -> {:?}", m, ciphertext);
    }

    println!("All good!");
}
