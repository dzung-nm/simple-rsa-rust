# Simple RSA Rust

## Overview

A very simple RSA implementation in Rust.

**This is not meant to be used in production**, but rather as a learning tool about Rust and RSA cryptography.

This is an educational implementation of the RSA encryption algorithm in Rust. It demonstrates the core concepts of
public-key cryptography through a clean, minimal codebase.

## Usage

### Basic Example

Example code (from `src/main.rs`):

```rust
use simple_rsa_rust::{decrypt, encrypt, new_keys};

fn main() {
    let bits = 2048;
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
```

### Benchmarking Key Generation

To measure RSA key generation performance on your system:

```bash
cargo run -r --example benchmark_new_keys
```

This runs a benchmark that generates 10 RSA-4096 key pairs and measures execution time per round.

**Example Output (M2 MacBook Air)**:

```
=== RSA Key Generation Benchmark ===

Generating RSA keys with 4096 bits, repeated 10 times...

Round 1: 968.798ms
Round 2: 474.460ms
Round 3: 336.550ms
Round 4: 407.107ms
Round 5: 989.885ms
Round 6: 586.438ms
Round 7: 390.452ms
Round 8: 706.646ms
Round 9: 390.452ms
Round 10: 320.493ms

✅ RSA key generation benchmark completed!
Average execution time: 556.828ms
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.