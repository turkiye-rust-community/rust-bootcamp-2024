struct Student {
    isim: String,
    yas: u8,
}

impl Student {
    fn print(&self) {
        println!("Isim: {}, Yas: {}", self.isim, self.yas)
    }

    // self
    pub fn new(isim: String, yas: u8) -> Self {
        Self { isim, yas }
    }

}

fn main() {
    let utku = Student::new("Utku".into(), 25);
    utku.print();
}
