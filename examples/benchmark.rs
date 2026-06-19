use simple_rsa_rust::rsa::new_keys;

fn main() {
    println!("=== RSA Key Generation Benchmark ===\n");

    let k = 10; // number of rounds
    let bits = 2048;
    let mut total_time = 0.0;

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
