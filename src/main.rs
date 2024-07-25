use bank::Bank;
use r#enum::{zort, Test};
use user::User;

pub mod bank;
pub mod r#enum;
pub mod user;

pub enum Status {
    Waiting,
    Processing,
    Processed(String),
}

pub struct Transaction {
    sender: User,
    receiver: User,
    amount: u32,
    status: Status,
}

pub enum Fee {
    Low,
    Medium,
    High,
}

fn main() {
    let fee = Fee::Low;
    let _bank = Bank::new("Bank of America".to_string(), fee, 900000000000000);
    let kol2 = Test::Kol1(42);
    zort(kol2);
}
