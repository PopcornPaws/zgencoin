use std::collections::HashMap;
use zgc_common::{Address, H256};
use zgc_crypto::{Hasher, Sha256};

pub struct Blockchain<'a> {
    height2hash: HashMap<usize, &'a str>,
    hash2block: HashMap<&'a str, Block>,
}

struct Block {
    heigth: usize,
    header: BlockHeader,
    data: TxData,
}

struct BlockHeader {
    created_at: u64,
    previous_hash: H256,
    nonce: u32,
}

pub struct TxData {
    signature: H256,
    sender: Address,
    recipient: Address,
    amount: u64,
}

pub struct Wallet {
    public_key: Address,
}

impl Wallet {
    pub fn new(private_key: String) -> Self {
        Self {
            public_key: keygen(private_key),
        }
    }

    pub fn new_transaction(
        &self,
        amount: u64,
        recipient: Address,
        private_key: String,
    ) -> Result<TxData, String> {
        if self.public_key != keygen(private_key.clone()) {
            return Err("Wrong private key provided, cannot sign transaction".to_string());
        }

        let tx_header = format!(
            "{},{:?},{:?},{}",
            amount, self.public_key, recipient, private_key
        );

        let hasher = Sha256::new();

        Ok(TxData {
            signature: hasher.digest(tx_header),
            sender: self.public_key,
            recipient,
            amount,
        })
    }
}

fn keygen(private_key: String) -> Address {
    let hasher = Sha256::new();
    let hash_result = hasher.digest(private_key).to_string();

    // uwrap is fine ...
    Address::try_from_str(&hash_result[..40]).unwrap()
}

#[test]
fn test_keygen() {
    let address = keygen(String::from("random2private#key"));
    assert_eq!(
        address.to_string(),
        "9ccf1c4cf49cb403f61aafff4c37a9b5a8f660cb"
    );
}
