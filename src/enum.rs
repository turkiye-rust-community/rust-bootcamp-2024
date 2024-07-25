pub enum Test {
    Kol1(u8),
    Kol2(u8),
    Kol3(u8),
    Kol4,
    Kol5,
    Kol6,
}

pub fn zort(test: Test) {
    match test {
        Test::Kol1(val) | Test::Kol3(val) => {
            println!("{}", val);
        }
        Test::Kol2(_) => {
            println!("Kol 2 ondan verisi umrumda degil");
        }
        _ => {
            println!("Fakir verisizler")
        }
    }
}

