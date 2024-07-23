// src/bank.rs

use crate::user::User;
use crate::transaction::Transaction;

pub struct Bank {
    pub users: Vec<User>,
    pub balance: u128,
    pub fee: u8,
    pub transactions: Vec<Transaction>,
}
