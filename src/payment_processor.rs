use crate::transaction::Transaction;

pub trait ProcessPayment {
    fn process(self, tx: Transaction) -> Result<Transaction, String>;
}

pub struct PaymentProccesor<T: ProcessPayment> {
    pub pool: Vec<Transaction>,
    pub banks: Vec<T>,
}

impl<T: ProcessPayment> PaymentProccesor<T> {
    pub fn new() -> Self {
        Self {
            pool: vec![],
            banks: vec![],
        }
    }

    pub fn add_bank(&mut self, bank: T) {
        self.banks.push(bank)
    }
}
