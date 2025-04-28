// src/main.rs

use std::collections::HashSet;
use std::io::{self, Write};
use tfhe::prelude::*;
use tfhe::{ConfigBuilder, generate_keys, set_server_key};
use ZamaHEBid::auction::Auction;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo,};

// Clears console on all OS
fn clear_console() {
    let mut out = io::stdout();
    execute!(out, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---------- Key Generation & Setup ----------
    let config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);  // Enable server key for operations

    // ---------- Initial User and Bid Setup ----------
    println!("*** Welcome to the Encrypted Auction CLI ***");

    print!("Enter the minimum bid (whole number): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let min_bid: u16 = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("Invalid minimum bid. Please restart and enter a positive number.");
            return Ok(());
        }
    };

    // Predefined set of valid user IDs.
    let valid_users: HashSet<String> = vec![
        "User1".to_string(),
        "User2".to_string(),
        "User3".to_string(),
        "User4".to_string(),
    ]
    .into_iter()
    .collect();
    // Track the user IDs that have already bid.
    let mut submitted_users: HashSet<String> = HashSet::new();

    let mut auction = Auction::new(&client_key, min_bid);

    println!("\nMinimum bid set.");
    println!("Enter 'x' at the User ID prompt to finish bidding early.\n");

    // ---------- Bidding Loop ----------
    // Continue prompting as long as there are users who haven't bid.
    while submitted_users.len() < valid_users.len() {
        print!("Please enter your user ID: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        let user_id = input.trim().to_string();
        if user_id.eq_ignore_ascii_case("x") {
            println!("Bidding terminated early.");
            break;
        }
        // Check if the entered user ID is valid.
        if !valid_users.contains(&user_id) {
            println!("User ID '{}' is not registered. Please try again.", user_id);
            continue;
        }
        // Check if the user already submitted a bid.
        if submitted_users.contains(&user_id) {
            println!("User ID '{}' has already submitted a bid.", user_id);
            continue;
        }
        // Prompt for this user's bid.
        print!("{} - enter your bid (must be a whole number above {}): ", user_id, min_bid);
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input)?;
        let bid_str = input.trim();
        let bid_value: u16 = match bid_str.parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid bid. Please enter a whole number.");
                continue;
            }
        };
        if bid_value <= min_bid {
            println!("Bid must be greater than the minimum bid ({}).", min_bid);
            continue;
        }
        // Accept and encrypt the bid.
        auction.add_bid(&user_id, bid_value);
        submitted_users.insert(user_id);
        println!("Bid accepted.\n");
        // Clear the console after a bid is entered.
        clear_console();
    }

    // If no bids were collected, exit.
    if auction.count_bids() == 0 {
        println!("No bids were placed. Auction terminated.");
        return Ok(());
    }

    // ---------- Handle Tie-breaks & Final Computation ----------
    let (mut max_ct, mut top_bidders) = auction.compute_max_encrypted_bid();
    while top_bidders.len() > 1 {
        let current_high: u16 = max_ct.decrypt(auction.client_key());
        println!("\n*** Tie detected! ***");
        print!("[");
        for (i, user) in top_bidders.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", user);
        }
        println!("] tied.");
        println!("Tie-breaker: these users must rebid to determine a single winner.");
        
        // For each tied user, prompt for a new bid.
        for user in top_bidders.clone() {
            loop {
                print!("{} - enter a new bid higher than {} (or 'x' to withdraw): ", user, current_high);
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input)?;
                let bid_input = input.trim();
                if bid_input.eq_ignore_ascii_case("x") {
                    println!("{} has withdrawn from the tie-break.", user);
                    auction.remove_bid(&user);
                    break;
                }
                let new_bid: u16 = match bid_input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid bid or 'x'.");
                        continue;
                    }
                };
                if new_bid <= current_high {
                    println!("Bid must be greater than {}.", current_high);
                    continue;
                }
                if new_bid <= min_bid {
                    println!("Bid must be greater than the minimum bid ({}).", min_bid);
                    continue;
                }
                // Accept the rebid.
                auction.add_bid(&user, new_bid);
                clear_console();
                break;
            }
        }
        let result = auction.compute_max_encrypted_bid();
        max_ct = result.0;
        top_bidders = result.1;
    }

    // Final winner and average computation.
    let winner = top_bidders
        .get(0)
        .cloned()
        .unwrap_or_else(|| "<none>".to_string());
    let highest_bid: u16 = max_ct.decrypt(auction.client_key());
    let avg_ct = auction.compute_average_encrypted();
    let avg_bid: u16 = avg_ct.decrypt(auction.client_key());

    println!("\n===== Auction Results =====");
    println!("Final Average Bid (rounded down): {}", avg_bid);
    if winner != "<none>" {
        println!("Highest Bid: {} (Winner: {})", highest_bid, winner);
    } else {
        println!("Highest Bid: {} (Unique winner not determined)", highest_bid);
    }
    println!("===========================\n");

    Ok(())
}
