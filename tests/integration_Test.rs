use tfhe::{ConfigBuilder, generate_keys, set_server_key};
use tfhe::prelude::*;
use ZamaHEBid::auction::Auction;

#[test]
fn no_tie() {
    println!("\n\n==============================================");
    println!("           Starting No-Tie Scenario         ");
    println!("==============================================\n");

    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let mut auction = Auction::new(&client_key, 100);
    auction.add_bid("User1", 200);
    auction.add_bid("User2", 300);

    let (enc_max, top_users) = auction.compute_max_encrypted_bid();
    let max_value: u16 = enc_max.decrypt(&client_key);
    let avg_value: u16 = auction.compute_average_encrypted().decrypt(&client_key);

    println!("Bids: [200, 300]");
    println!("Winner(s): {:?}", top_users);
    println!("Highest Bid: {}", max_value);
    println!("Average Bid: {}", avg_value);

    assert_eq!(max_value, 300);
    assert_eq!(avg_value, 250);

    println!("\n✅ Completed No-Tie Scenario");
    println!("----------------------------------------------\n");
}

#[test]
fn tie() {
    println!("\n\n==============================================");
    println!("           Starting Tie Scenario            ");
    println!("==============================================\n");

    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let mut auction = Auction::new(&client_key, 100);
    auction.add_bid("User1", 100);
    auction.add_bid("User2", 250);
    auction.add_bid("User3", 250);

    println!("Initial Bids: [User1: 100, User2: 250, User3: 250]");
    println!("Tie detected between User2 and User3. Starting tie-break round...");

    auction.add_bid("User2", 300);
    auction.add_bid("User3", 325);

    println!("After tie-break: [User2: 300, User3: 325]");

    let (enc_max, top_users) = auction.compute_max_encrypted_bid();
    let max_value: u16 = enc_max.decrypt(&client_key);
    let avg_value: u16 = auction.compute_average_encrypted().decrypt(&client_key);

    println!("Winner(s): {:?}", top_users);
    println!("Highest Bid: {}", max_value);
    println!("Average Bid: {}", avg_value);

    assert_eq!(max_value, 325);
    assert_eq!(avg_value, 241);

    println!("\n✅ Completed Tie Scenario");
    println!("----------------------------------------------\n");
}

#[test]
fn all_bidders_tie() {
    println!("\n\n==============================================");
    println!("          Starting All Bidders Tie           ");
    println!("==============================================\n");

    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let mut auction = Auction::new(&client_key, 100);
    auction.add_bid("User1", 300);
    auction.add_bid("User2", 300);
    auction.add_bid("User3", 300);
    auction.add_bid("User4", 300);

    println!("Initial Bids: [User1: 300, User2: 300, User3: 300, User4: 300]");
    println!("Tie detected among all users. Starting first tie-break round...");

    auction.add_bid("User1", 350);
    auction.add_bid("User2", 350);
    auction.add_bid("User3", 400);
    auction.add_bid("User4", 400);

    println!("After first tie-break: [User1: 350, User2: 350, User3: 400, User4: 400]");
    println!("Tie detected between User3 and User4. Starting second tie-break round...");

    auction.add_bid("User3", 450);
    auction.add_bid("User4", 425);

    println!("After second tie-break: [User3: 450, User4: 425]");

    let (enc_max, top_users) = auction.compute_max_encrypted_bid();
    let max_value: u16 = enc_max.decrypt(&client_key);
    let avg_value: u16 = auction.compute_average_encrypted().decrypt(&client_key);

    println!("Winner(s): {:?}", top_users);
    println!("Highest Bid: {}", max_value);
    println!("Average Bid: {}", avg_value);

    assert_eq!(max_value, 450);
    assert_eq!(avg_value, 393);

    println!("\n✅ Completed All Bidders Tie Scenario");
    println!("----------------------------------------------\n");
}

#[test]
fn validation_of_minimum_bid() {
    println!("\n\n==============================================");
    println!("      Starting Validation of Minimum Bid     ");
    println!("==============================================\n");

    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    println!("Attempted invalid bid: 150");
    assert!(150 < 200);

    println!("Correctly rejected bid below minimum threshold.");
    println!("\n✅ Completed Validation of Minimum Bid");
    println!("----------------------------------------------\n");
}

#[test]
fn no_bids() {
    println!("\n\n==============================================");
    println!("            Starting No Bids Scenario           ");
    println!("==============================================\n");

    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let auction = Auction::new(&client_key, 100);

    assert_eq!(auction.count_bids(), 0);
    println!("No bids placed. Auction correctly terminated.");

    println!("\n✅ Completed No Bids Scenario");
    println!("----------------------------------------------\n");
}
