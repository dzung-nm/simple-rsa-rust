use rand::prelude::*;

/// Add padding to the input bytes according to PKCS#1 v1.5 for encryption.
pub fn pkcs1_padding(bytes: &[u8], k: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut padded = Vec::with_capacity(k);
    padded.push(0x00); // Leading zero
    padded.push(0x02); // Block type for encryption
    let padding_length = k as isize - 3 - bytes.len() as isize; // Calculate padding length
    if padding_length < 8 {
        panic!("Message is too long for the given key size");
    }
    for _ in 0..padding_length {
        padded.push(rng.gen_range(1..=255)); // Non-zero random bytes
    }
    padded.push(0x00); // Separator byte
    padded.extend_from_slice(bytes); // Append the original message
    padded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkcs1_padding() {
        let message = b"Hello, World!";
        let k = 32;
        let l = message.len();
        let padded = pkcs1_padding(message, k);
        assert_eq!(padded[0], 0x00);
        assert_eq!(padded[1], 0x02);
        assert!(padded[2..k - l - 1].iter().all(|&x| x != 0));
        assert_eq!(padded[k - l - 1], 0x00);
        assert_eq!(&padded[k - l..], message);
        println!("Original message: {message:?}");
        println!("Padded message: {padded:?}");
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let padded = pkcs1_padding(b"Hello, World!", 0);
        let _ = padded;
    }
}
