use simple_rsa_rust::rsa::*;

fn main() {
    let bits = 512;
    let (public_key, private_key) = new_keys(bits);
    let messages = vec![
        b"Hello, RSA!".to_vec(),
        b"Rust is great for cryptography.".to_vec(),
        b"Simple RSA implementation.".to_vec(),
    ];
    for message in messages {
        let encrypted = encrypt(&message, &public_key);
        let decrypted = decrypt(&encrypted, &private_key);
        assert_eq!(message.to_vec(), decrypted);
        println!("Original message: {:?}", String::from_utf8_lossy(&message));
        println!("Encrypted message: {}", encrypted);
        println!(
            "Decrypted message: {:?}",
            String::from_utf8_lossy(&decrypted)
        );
    }
}
