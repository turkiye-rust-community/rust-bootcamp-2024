// src/user.rs

pub struct User {
    pub name: String,
    pub account_id: String,
    pub balance: u128,
}

impl User {
    pub fn new(name: String, account_id: String, balance: u128) -> Self {
        Self {
            name,
            account_id,
            balance,
        }
    }

    pub fn add_balance(&mut self, amount: u32) {
        self.balance += amount as u128;
    }
}
