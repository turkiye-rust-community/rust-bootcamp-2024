// src/bank.rs

use crate::transaction::Transaction;
use crate::user::User;

#[derive(Clone)] // Derive Clone trait
pub struct Bank {
    pub users: Vec<User>,
    pub balance: u128,
    pub fee: u8,
    pub transactions: Vec<Transaction>,
}

impl Bank {
    pub fn new(fee: u8) -> Self {
        Self {
            users: Vec::new(),
            balance: 0,
            fee,
            transactions: Vec::new(),
        }
    }

    // Method to add a user to the bank
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    // Method to list all users in the bank
    pub fn list_users(&self) -> Vec<String> {
        self.users
            .iter()
            .map(|user| {
                format!(
                    "Name: {}, Account ID: {}, Balance: {}",
                    user.name, user.account_id, user.balance
                )
            })
            .collect()
    }
}
