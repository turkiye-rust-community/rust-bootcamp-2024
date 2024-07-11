use std::{fmt::Display, iter::repeat_with};

use fastrand::alphanumeric;

#[derive(Clone)]
pub enum TransactionStatus {
    Active,
    Processing(String),
    Complete(String),
    Error(String),
}

#[derive(Clone)]
pub struct Transaction {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub amount_moved: u64,
    pub status: TransactionStatus,
}

impl Transaction {
    pub fn new(sender_id: &str, receiver_id: String, amount_moved: u64) -> Self {
        let id: String = repeat_with(alphanumeric).take(10).collect();
        Self {
            id,
            sender_id: sender_id.to_string(),
            receiver_id,
            amount_moved,
            status: TransactionStatus::Active,
        }
    }

    pub fn processing(&mut self, processor_id: &str) {
        self.status = TransactionStatus::Processing(processor_id.into());
    }

    pub fn complete(&mut self, processor_id: &str) {
        self.status = TransactionStatus::Complete(processor_id.into())
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {} send ${} to User {}", self.sender_id, self.amount_moved, self.receiver_id)
    }
}
