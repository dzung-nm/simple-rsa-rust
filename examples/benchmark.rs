use simple_rsa_rust::*;

fn main() {
    let k = 10; // number of rounds
    let bits = RSA_MAX_BITS;
    let mut total_time = 0.0;

    println!("=== RSA Key Generation Benchmark ===\n");
    println!(
        "Generating RSA keys with {} bits, repeated {} times...\n",
        bits, k
    );

    for i in 1..=k {
        let start = std::time::Instant::now();
        new_keys(bits);
        let duration = start.elapsed();
        total_time += duration.as_secs_f64();
        println!("Round {}: {:.3}ms", i, duration.as_secs_f64() * 1000.0);
    }

    println!("\n✅ RSA key generation benchmark completed!");
    println!(
        "Average execution time: {:.3}ms",
        total_time / k as f64 * 1000.0
    );
}
