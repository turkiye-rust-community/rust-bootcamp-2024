// src/user.rs

#[derive(Clone)]
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

    pub fn transfer(&mut self, receiver: &mut User, amount: u32) -> Result<(), &'static str> {
        if amount as u128 > self.balance {
            return Err("Insufficient balance");
        }
        self.balance -= amount as u128;
        receiver.add_balance(amount);
        Ok(())
    }
}
