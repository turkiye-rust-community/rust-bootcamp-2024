use std::iter::repeat_with;

use fastrand::alphanumeric;

use crate::transaction::Transaction;

pub struct User {
    pub account_id: String,
    balance: u64,
    transactions: Vec<Transaction>,
}

impl User {
    pub fn new() -> Self {
        let account_id: String = repeat_with(alphanumeric).take(10).collect();
        Self {
            account_id,
            balance: 0,
            transactions: vec![],
        }
    }

    pub fn push_tx(
        &mut self,
        amount: u64,
        receiver_id: String,
        transaction_pool: &mut Vec<Transaction>,
    ) -> Result<(), String> {
        if amount > self.balance {
            return Err("Cannot transfer more than you have".into());
        };

        let transaction = Transaction::new(&self.account_id, receiver_id, amount);
        self.transactions.push(transaction.clone());
        transaction_pool.push(transaction.clone());
        Ok(())
    }

    pub fn update_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        let (_, mut rest): (Vec<Transaction>, Vec<Transaction>) = self
            .transactions
            .clone()
            .into_iter()
            .partition(|tx| tx.id == transaction.id);

        rest.push(transaction);

        self.transactions = rest;
        Ok(())
    }

    pub fn get_balance(&self) -> String {
        format!("You have ${}", self.balance)
    }

    pub fn add_balance(&mut self, amount: u64) -> Result<(), String> {
        match self.balance.checked_add(amount) {
            Some(new_balance) => self.balance = new_balance,
            None => return Err("Overflow happened, you are rich enough buddy".into()),
        };

        Ok(())
    }
}
