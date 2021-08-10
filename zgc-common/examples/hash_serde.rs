use zgc_common::Address;

use std::fs::File;

fn main() {
    let path_to_json = "examples/hash_test.json";
    let reader = File::open(path_to_json).expect("Failed to open file");
    let address = Address::try_from_str("abcdef01234567899876543210abcdef01234567")
        .expect("invalid address string provided");
    println!(
        "serialized = {:?}",
        serde_json::to_string(&address).unwrap()
    );
    let address: Address = serde_json::from_reader(reader).expect("failed to deserialize address");

    println!("parsed = {:?}", address);
}
