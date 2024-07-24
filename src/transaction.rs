// src/transaction.rs

use crate::bank::Bank;
use crate::user::User;

#[derive(Clone)]
pub struct Transaction {
    pub sender: User,
    pub receiver: User,
    pub amount: u32,
    pub bank: Bank,
}
