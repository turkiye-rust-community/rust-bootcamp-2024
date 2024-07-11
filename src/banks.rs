use std::iter::repeat_with;

use fastrand::alphanumeric;

use crate::{payment_processor::ProcessPayment, transaction::Transaction, user::User};

pub struct Bank {
    id: String,
    fee: u8,
    vault: u64,
    users: Vec<User>,
}

impl ProcessPayment for Bank {
    fn process(&mut self, mut tx: Transaction) -> Result<Transaction, String> {
        let receiver = self
            .users
            .iter_mut()
            .find(|user| user.account_id == tx.receiver_id)
            .take();

        let receiver = match receiver {
            Some(user) => user,
            None => return Err("Cannot process this user isn't on my bank".into()),
        };
        tx.processing(&self.id);
        let fee = tx.amount_moved.div_ceil(self.fee.into());
        if let Err(err) = receiver.add_balance(tx.amount_moved - fee) {
            return Err(format!("{}", err));
        };

        self.vault += fee;

        tx.complete(&self.id);
        Ok(tx)
    }
}

impl Bank {
    pub fn new(fee: u8) -> Self {
        let id: String = repeat_with(alphanumeric).take(10).collect();
        Self {
            id,
            fee,
            vault: 0,
            users: vec![],
        }
    }

    pub fn add_user(&mut self, user: &User) {
        self.users.push(user.clone())
    }
}
