use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;

/// Find e such that 1 < e < ϕ(n) and gcd(e, ϕ(n)) = 1
pub fn encryption_exponent(phi: &BigInt) -> BigInt {
    if *phi <= BigInt::from(3) {
        panic!("phi is too small!");
    }
    let mut e = BigInt::from(65537); // Common choice for e
    if *phi <= e {
        e = BigInt::from(3);
    }
    while e.gcd(phi) != BigInt::one() {
        e += BigInt::from(2);
        if &e >= phi {
            panic!("Could not find a valid encryption exponent smaller than phi");
        }
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_exponent() {
        assert_eq!(encryption_exponent(&BigInt::from(4)), BigInt::from(3));
        assert_eq!(encryption_exponent(&BigInt::from(5)), BigInt::from(3));
        assert_eq!(encryption_exponent(&BigInt::from(6)), BigInt::from(5));
        assert_eq!(encryption_exponent(&BigInt::from(15)), BigInt::from(7));
        assert_eq!(encryption_exponent(&BigInt::from(65537)), BigInt::from(3));
        assert_eq!(
            encryption_exponent(&BigInt::from(65538)),
            BigInt::from(65537)
        );
        assert_eq!(
            encryption_exponent(&BigInt::from(65537 * 2)),
            BigInt::from(65539)
        );
    }
}
