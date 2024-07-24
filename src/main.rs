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
    let mut aziz_bank = Bank::new(1);
    processor.add_bank(aziz_bank.clone()); // Use clone to avoid moving

    println!("Bank added to the payment processor.");

    // Create users
    let mut alice = User::new("Alice".to_string(), "alice123".to_string(), 1000);
    let mut bob = User::new("Bob".to_string(), "bob123".to_string(), 500);

    // Add users to the bank
    aziz_bank.add_user(alice.clone());
    aziz_bank.add_user(bob.clone());

    // List users in the bank
    println!("-------start USER LIST-------");
    for user_info in aziz_bank.list_users() {
        println!("{}", user_info);
    }
    println!("-------end USER LIST-------");

    // Perform a transfer
    match alice.transfer(&mut bob, 200) {
        Ok(_) => println!("Transfer successful!"),
        Err(err) => println!("Transfer failed: {}", err),
    }

    // Print the balances
    println!("Alice's balance: {}", alice.balance);
    println!("Bob's balance: {}", bob.balance);
}
