use simple_rsa_rust::rsa::{decrypt, encrypt, new_keys};

fn main() {
    let bits = 1024;
    let (public_key, private_key) = new_keys(bits);
    let messages = vec![
        b"Hello, RSA!".to_vec(),
        b"4E636AF98E40F3ADCFCCB698F4E80B9F".to_vec(), // A random session key for AES-128
        b"A9993E364706816ABA3E25717850C26C9CD0D89D".to_vec(), // AN SHA-1 hash of a string
    ];
    for message in messages {
        println!("--------------------------------------------------");
        let encrypted = encrypt(&message, &public_key);
        let decrypted = decrypt(&encrypted, &private_key);
        assert_eq!(decrypted, message);
        println!("Original message: {:?}", String::from_utf8_lossy(&message));
        println!("Encrypted message in hex: {}", format!("{:X}", encrypted));
    }
}
