use zgc_common::Address;

fn main() {
    let address = Address::new([3; 20]);
    let address_2: Address =
        serde_json::from_str("\"0ffffaaabbcccccdddeeeffaaaa7faaa01345678\"").unwrap();
    println!("{:?}", serde_json::to_string(&address).unwrap());
    println!("{:?}", address_2);
}
