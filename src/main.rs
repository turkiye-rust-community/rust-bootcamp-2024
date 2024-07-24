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
}
