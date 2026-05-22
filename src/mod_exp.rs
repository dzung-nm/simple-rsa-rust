// return base^e mod n
pub fn mod_exp(mut base: i64, mut e: i64, n: i64) -> i64 {
    if n == 1 { return 0; }
    if e == 0 { return 1; }
    if base == 1 { return 1; }

    // Handle negative base safely
    base = base % n;
    if base < 0 {
        base += n;
    }

    let mut result = 1i128;
    let mut b = base as i128;
    let n_128 = n as i128;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % n_128;
        }
        b = (b * b) % n_128;
        e /= 2;
    }

    result as i64
}

#[cfg(test)]
mod tests {
    use crate::mod_exp::mod_exp;

    #[test]
    fn test() {
        assert_eq!(mod_exp(5, 2, 3), 1);
        assert_eq!(mod_exp(7, 2, 5), 4);
        assert_eq!(mod_exp(2, 65537, 1766559913591342853), 919227530862777093);
    }
}
