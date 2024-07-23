// src/main.rs

mod bank;
mod user;
mod transaction;

use bank::Bank;
use user::User;
use transaction::Transaction;

struct PaymentProcessor {
    banks: Vec<Bank>,
}

fn main() {
    //* Nothing here
}
