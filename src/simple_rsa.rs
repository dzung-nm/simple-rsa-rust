use crate::extended_gcd::extended_gcd;
use crate::mod_exp::mod_exp;
use crate::generate_prime_pair::generate_prime_pair;

fn mod_inverse(e: i64, phi: i64) -> Option<i64> {
    let (gcd, x, _) = extended_gcd(e, phi);

    // e and phi must be coprime
    if gcd != 1 {
        return None;
    }

    // ensure the positive result
    Some((x % phi + phi) % phi)
}

// 1 < e < ϕ(n) and gcd(e, ϕ(n)) = 1
fn encryption_exponent(phi: i64) -> i64 {
    let e = 65537; // Common choice for e
    if phi <= e {
        panic!("phi is too small");
    }
    if extended_gcd(e, phi).0 != 1 {
        panic!("e and phi must be coprime");
    }
    e
}

pub fn simple_rsa() {
    let (p, q) = generate_prime_pair(31);

    let n = p * q;
    let phi = (p - 1) * (q - 1);

    let e = encryption_exponent(phi);
    let d = mod_inverse(e, phi).expect("e and phi must be coprime");

    // test
    let messages: [i64; 1000] = std::array::from_fn(|i| i as i64 + 1);
    for message in messages {
        let ciphertext = mod_exp(message, e, n);
        let decrypted = mod_exp(ciphertext, d, n);
        assert_eq!(message, decrypted);
    }

    println!("All good!");
}
