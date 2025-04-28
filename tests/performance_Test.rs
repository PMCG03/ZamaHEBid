use std::time::Instant;
use rand::{thread_rng, Rng};
use tfhe::{ConfigBuilder, generate_keys, set_server_key};
use ZamaHEBid::auction::Auction;  
use tfhe::prelude::*;                  


#[test]
fn performance_test_benchmark() {
    let n: usize = 50; 

    println!("Running performance test with {} bidders", n);


    let t0 = Instant::now();
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);
    let keygen_ms = t0.elapsed().as_millis();


    let mut rng = thread_rng();
    let mut auction = Auction::new(&client_key, 0); // min_bid=0 for benchmark

    let t1 = Instant::now();
    for i in 0..n {
        let bid: u16 = rng.gen_range(1_000..=50_000);
        auction.add_bid(&format!("BIDDER{}", i), bid);
    }
    let enc_ms = t1.elapsed().as_millis();


    let t2 = Instant::now();
    let (max_ct, _) = auction.compute_max_encrypted_bid();
    let avg_ct    = auction.compute_average_encrypted();
    let comp_ms   = t2.elapsed().as_millis();


    let t3 = Instant::now();
    let max_plain: u16 = max_ct.decrypt(&client_key);
    let avg_plain: u16 = avg_ct.decrypt(&client_key);
    let dec_ms = t3.elapsed().as_millis();


    // ---------- Results ----------
    println!("\n=== FHE Benchmark ({} bidders) ===", n);
    println!("Key generation    : {:>5} ms", keygen_ms);
    println!(
        "Encryption+Store : {:>5} ms   (≈{:.1} ms / bid)",
        enc_ms,
        enc_ms as f64 / n as f64
    );
    println!("Max+Average comp : {:>5} ms", comp_ms);
    println!("Decryption        : {:>5} ms", dec_ms);
    println!("Highest bid       : {}", max_plain);
    println!("Average bid       : {}", avg_plain);
}
