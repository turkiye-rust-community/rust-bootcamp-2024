struct Bank {
    users: Vec<User>,
    safe: u128,
    fee: u8,
    transactions: Vec<Transaction>,
}

struct User {
    name: String,
    account_id: String,
    balance: u128,
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
        self.balance = self.balance + amount as u128;
    }
}

struct Transaction {
    sender: User,
    receiver: User,
    amount: u32,
    bank: Bank,
}
impl Transaction {
    fn new() -Self {}
}

struct PaymentProcessor {
    banks: Vec<Bank>,
}

fn main() {}
