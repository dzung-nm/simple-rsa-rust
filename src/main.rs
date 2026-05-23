use simple_rsa_rust::rsa::{decrypt, encrypt, new_keys};

fn main() {
    let bits = 512;
    let (public_key, private_key) = new_keys(bits);
    let messages: [i64; 10] = std::array::from_fn(|i| i as i64 + 1);
    for message in messages {
        let ciphertext = encrypt(message, &public_key);
        let decrypted_message = decrypt(&ciphertext, &private_key);
        assert_eq!(message, decrypted_message);
        println!("{:?} -> {:?}", message, ciphertext);
    }
}
