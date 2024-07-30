use std::collections::HashMap;

pub fn vec_testing() {
    let mut dummy_vec = Vec::<u32>::new();

    dummy_vec.push(23);
    dummy_vec.push(24);
    dummy_vec.push(6);

    let last = dummy_vec.pop();
}

pub fn string_testing() {
    let mut dummy_string = String::new();
    dummy_string.push('A');
    dummy_string.push_str("merika");

    let zort = dummy_string + " is not great";
    let zart = "Hello World";

    let a = format!("this is zort -> {} ----- {}", zort, zart);
    print!("{}\n", a);
    println!("{}", zort);
    let s = "name: utku";
}

pub fn hashmap_test() {
    let mut hm = HashMap::<String, String>::new();
    hm.insert("utku".into(), "bu benim numaram degil".into());
    let utkunun_numara = hm.get("utku").unwrap();
    let prev = hm.insert("veli uysal".into(), "buda benim numarm degil".into());
    let asfasf = hm.remove("veli uysal".into());

    println!("{:?} \n {:?}", hm, prev)
}
