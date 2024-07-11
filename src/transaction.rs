use std::iter::repeat_with;

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
    sender_id: String,
    receiver_id: String,
    amount_moved: u64,
    status: TransactionStatus,
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

    pub fn processing(&mut self, processor_id: String) {
        self.status = TransactionStatus::Processing(processor_id);
    }

    pub fn complete(&mut self, processor_id: String) {
        self.status = TransactionStatus::Complete(processor_id)
    }
}
