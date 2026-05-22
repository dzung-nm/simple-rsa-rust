pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    // Return (gcd, x, y)
    // so that: ax + by = gcd(a, b)

    if b == 0 {
        return (a, 1, 0);
    }

    let (gcd, x1, y1) = extended_gcd(b, a % b);

    let x = y1;
    let y = x1 - (a / b) * y1;

    (gcd, x, y)
}

#[cfg(test)]
mod tests {
    use crate::extended_gcd::extended_gcd;

    #[test]
    fn test_extended_gcd() {
        let (gcd, x, y) = extended_gcd(30, 21);
        assert_eq!(gcd, 3);
        assert_eq!(30 * x + 21 * y, gcd);
    }
}
