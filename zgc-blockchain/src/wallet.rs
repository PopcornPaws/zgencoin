use crate::TxData;

use zgc_common::Address;
use zgc_crypto::{Hasher, Sha256};

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
            return Err("Wrong private key provided, cannot sign transaction".to_owned());
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
    // unwrap is fine because we know the length of hash_result
    // and that it will always contain valid hex data
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
