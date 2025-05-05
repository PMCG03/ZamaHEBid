use std::collections::HashMap;
use tfhe::{FheUint16, FheUint32};
use tfhe::prelude::*;  // Bring traits like .eq, .max into scope
use tfhe::ClientKey;

pub struct Auction<'a> {
    client_key: &'a ClientKey,                 // Reference to the client's secret key (for encryption/decryption)
    min_bid: u16,                              // Minimum bid threshold
    bids: HashMap<String, FheUint16>,          // Encrypted bids mapped by user ID
}

impl<'a> Auction<'a> {
    /// Create new Auction with a given client key and minimum bid.
    pub fn new(client_key: &'a ClientKey, min_bid: u16) -> Self {
        Auction {
            client_key,
            min_bid,
            bids: HashMap::new(),
        }
    }

    // Get a reference to the client key (needed for decryption in main).
    pub fn client_key(&self) -> &'a ClientKey {
        self.client_key
    }

    // Encrypt a user's bid and store it (or update if the user already has a bid).
    // Assumes bid_value is validated to be > min_bid.
    pub fn add_bid(&mut self, user_id: &str, bid_value: u16) {
        // Encrypt the bid using the ClientKey. This produces an FHE ciphertext.
        let enc_bid = FheUint16::encrypt(bid_value, self.client_key);
        // Insert or update the bid in the HashMap
        self.bids.insert(user_id.to_string(), enc_bid);
    }

    // Remove a user's bid
    pub fn remove_bid(&mut self, user_id: &str) {
        self.bids.remove(user_id);
    }

    /// Return the number of bids currently stored.
    pub fn count_bids(&self) -> usize {
        self.bids.len()
    }

    /// Compute the encrypted maximum bid among all stored bids.
    pub fn compute_max_encrypted_bid(&self) -> (FheUint16, Vec<String>) {
        assert!(!self.bids.is_empty(), "No bids to compute max from");

        // Iterates through the bids and use the .max() operation pairwise.
        let mut iter = self.bids.iter();
        let (_, first_enc) = iter.next().unwrap();
        // Start with the first bid as current max
        let mut current_max = first_enc.clone();
        // Iterate through remaining bids and update the max ciphertext
        for (_, enc_bid) in iter {
            // homomorphic max operation
            current_max = current_max.max(enc_bid);
        }

        // Determine which user(s) have this max value
        let mut top_users: Vec<String> = Vec::new();
        for (user, enc_bid) in &self.bids {
            // Homomorphic comparison: check if enc_bid equals current_max
            let is_equal = enc_bid.eq(&current_max);
            // Decrypt the comparison result to a bool (true if equal)
            if is_equal.decrypt(self.client_key) {
                top_users.push(user.clone());
            }
        }
        (current_max, top_users)
    }

    /// Compute the encrypted average of all bids(always rounds down)
    pub fn compute_average_encrypted(&self) -> FheUint16 {
        assert!(!self.bids.is_empty(), "No bids to compute average");
        // Number of bids as a clear constant
        let count = self.bids.len() as u32;
    
        // Homomorphically sum all bids in a 32‑bit ciphertext
        let mut sum_enc = FheUint32::encrypt(0u32, self.client_key);
        for enc_bid in self.bids.values() {
            let enc_bid_32 = FheUint32::cast_from(enc_bid.clone());
            sum_enc = sum_enc + enc_bid_32;  // homomorphic addition
        }
    
        // Homomorphically divide the encrypted sum by the clear constant `count`
        let avg_enc_32 = sum_enc / count;
    
        // Cast the 32‑bit ciphertext back to 16‑bit
        FheUint16::cast_from(avg_enc_32)
    }
    
}
