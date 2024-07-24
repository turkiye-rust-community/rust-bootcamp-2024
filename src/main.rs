// src/main.rs

mod bank;
mod transaction;
mod user;

use bank::Bank;
use transaction::Transaction;
use user::User;

struct PaymentProcessor {
    banks: Vec<Bank>,
}

impl PaymentProcessor {
    pub fn new() -> Self {
        Self { banks: Vec::new() }
    }
    
    pub fn add_bank(&mut self, bank: Bank) {
        self.banks.push(bank);
    }
}

fn main() {
    let mut processor = PaymentProcessor::new();
    let bank = Bank {
        users: Vec::new(),
        balance: 0,
        fee: 0,
        transactions: Vec::new(),
    };

    processor.add_bank(bank);

    println!("Bank added to the payment processor.");
    
    // Create users
    let mut alice = User::new("Alice".to_string(), "alice123".to_string(), 1000);
    let mut bob = User::new("Bob".to_string(), "bob123".to_string(), 500);
    
    // Perform a transfer
    match alice.transfer(&mut bob, 200) {
        Ok(_) => println!("Transfer successful!"),
        Err(err) => println!("Transfer failed: {}", err),
    }
    
    // Print the balances
    println!("Alice's balance: {}", alice.balance);
    println!("Bob's balance: {}", bob.balance);
}
