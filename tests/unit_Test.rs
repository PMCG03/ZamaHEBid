use tfhe::{ConfigBuilder, generate_keys, set_server_key, ClientKey};
use tfhe::prelude::*;
use ZamaHEBid::auction::Auction;

fn setup_fhe() -> ClientKey {
    let config = ConfigBuilder::default().build();
    let (ck, sk) = generate_keys(config);
    set_server_key(sk);
    ck
}

#[test]
fn two_different_comparison() {
    let ck = setup_fhe();
    let mut auction = Auction::new(&ck, 0);
    auction.add_bid("A", 200);
    auction.add_bid("B", 100);

    let (enc_max, winners) = auction.compute_max_encrypted_bid();
    let max: u16 = enc_max.decrypt(&ck);

    println!("Test two_different_comparison: bids [200,100], max = {}, winners = {:?}", max, winners);
    assert_eq!(max, 200);
    assert_eq!(winners, vec!["A".to_string()]);
}

#[test]
fn two_equal_comparison() {
    let ck = setup_fhe();
    let mut auction = Auction::new(&ck, 0);
    auction.add_bid("X", 300);
    auction.add_bid("Y", 300);

    let (enc_max, winners) = auction.compute_max_encrypted_bid();
    let max: u16 = enc_max.decrypt(&ck);

    println!("Test two_equal_comparison: bids [300,300], max = {}, winners = {:?}", max, winners);
    assert_eq!(max, 300);
    assert_eq!(winners.len(), 2);
}

#[test]
fn list_compare() {
    let ck = setup_fhe();
    let mut auction = Auction::new(&ck, 0);
    let vals = [100, 400, 250, 150];
    for (i, &v) in vals.iter().enumerate() {
        auction.add_bid(&format!("U{}", i), v);
    }

    let (enc_max, winners) = auction.compute_max_encrypted_bid();
    let max: u16 = enc_max.decrypt(&ck);

    println!("Test list_compare: bids {:?}, max = {}, winners = {:?}", vals, max, winners);
    assert_eq!(max, 400);
    assert_eq!(winners, vec!["U1".to_string()]);
}

#[test]
fn list_average() {
    let ck = setup_fhe();
    let mut auction = Auction::new(&ck, 0);
    let vals = [100, 250, 200, 350];
    for (i, &v) in vals.iter().enumerate() {
        auction.add_bid(&format!("V{}", i), v);
    }

    let cavg = auction.compute_average_encrypted();
    let avg: u16 = cavg.decrypt(&ck);

    // floor((100 + 250 + 200 + 350) / 4) = floor(900/4) = 225
    println!("Test list_average: bids {:?}, avg = {}", vals, avg);
    assert_eq!(avg, 225);
}

#[test]
fn single_value_average() {
    let ck = setup_fhe();
    let mut auction = Auction::new(&ck, 0);
    auction.add_bid("Solo", 200);

    let cavg = auction.compute_average_encrypted();
    let avg: u16 = cavg.decrypt(&ck);

    println!("Test single_value_average: bids [200], avg = {}", avg);
    assert_eq!(avg, 200);
}
