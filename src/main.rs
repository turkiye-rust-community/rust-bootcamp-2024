use rust_bootcamp_project::{banks::Bank, payment_processor::PaymentProccesor, user::User};

fn main() {
    let mut bank = Bank::new(4);
    let mut utku = User::new(12125125124);
    let mut emre = User::new(121241251251);
    let mut veli = User::new(12512412512512);

    bank.add_user(&utku);
    bank.add_user(&emre);
    bank.add_user(&veli);

    let mut processor = PaymentProccesor::<Bank>::new();

    processor.add_bank(bank);

    match veli.push_tx(121241243, &utku.account_id, &mut processor.pool) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    };
    match utku.push_tx(12323, &emre.account_id, &mut processor.pool) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    };

    match emre.push_tx(14000, &utku.account_id, &mut processor.pool) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    };
    match utku.push_tx(120000, &veli.account_id, &mut processor.pool) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    };

    processor.process();
}
