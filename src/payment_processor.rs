use crate::transaction::Transaction;

pub trait ProcessPayment {
    fn process(&mut self, tx: Transaction) -> Result<Transaction, String>;
}

#[derive(Clone)]
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

    pub fn process(&mut self) {
        for tx in self.pool.clone() {
            for bank in &mut self.banks {
                match bank.process(tx.clone()) {
                    Ok(tx) => {
                        println!("Succesfully transfered funds -> {}", tx);
                        let rest: Vec<Transaction> = self
                            .pool
                            .clone()
                            .into_iter()
                            .filter(|ptx| ptx.id != tx.id)
                            .collect();

                        self.pool = rest;
                    }
                    Err(err) => {
                        println!("Failed -> {}", err)
                    }
                }
            }
        }
    }
}
