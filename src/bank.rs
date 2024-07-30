use crate::{user::User, Fee, Status, Transaction};

pub struct Bank {
    name: String,
    users: Vec<User>,
    safe: u128,
    fee: Fee,
    transactions: Vec<Transaction>,
}

pub trait Process {
    fn process(&mut self, tx: Transaction) -> Result<(), String>;
}

impl Process for Bank {
    fn process(&mut self, tx: Transaction) -> Result<(), String> {
        let Transaction {
            sender: _,
            receiver: _,
            amount,
            status,
        } = tx;

        if let Status::Waiting = status {
            let fee: f32 = match self.fee {
                Fee::Low => 1.0,
                Fee::Medium => 2.5,
                Fee::High => 4.7,
            };

            let fee_amount = amount as f32 / fee;
            self.safe += fee_amount as u128;
            //sender account'dan parayi kes
            //receiver'e ekle

            Ok(())
        } else {
            return Err("Cannot process this transaction".to_string());
        }
    }
}

impl Bank {
    pub fn new(name: String, fee: Fee, safe: u128) -> Self {
        Self {
            name,
            users: vec![],
            safe,
            fee,
            transactions: vec![],
        }
    }
}
