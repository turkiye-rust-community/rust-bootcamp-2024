pub struct User {
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
