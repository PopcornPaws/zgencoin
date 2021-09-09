use crate::TxData;

use zgc_common::{Address, H256};
use zgc_crypto::{Hasher, Sha256};

pub struct Wallet {
    public_key: Address,
}

impl Wallet {
    pub fn new(private_key: &str) -> Self {
        Self {
            public_key: keygen(private_key),
        }
    }

    pub fn pubkey(&self) -> &Address {
        &self.public_key
    }

    pub fn new_transaction(
        &self,
        amount: u64,
        recipient: Address,
        private_key: &str,
        created_at: u128,
    ) -> Result<TxData, String> {
        if self.public_key != keygen(private_key) {
            return Err("Wrong private key provided, cannot sign transaction".to_owned());
        }

        let tx_header = format!(
            "{},{:?},{:?},{}",
            amount, self.public_key, recipient, created_at,
        );

        let hasher = Sha256::new();

        Ok(TxData {
            signature: hasher.digest(tx_header),
            sender: self.public_key,
            recipient,
            amount,
        })
    }

    pub fn new_self_mint(&self, amount: u64) -> TxData {
        TxData {
            signature: H256::max(),
            sender: self.public_key,
            recipient: self.public_key,
            amount,
        }
    }
}

fn keygen(private_key: &str) -> Address {
    let hasher = Sha256::new();
    let hash_result = hasher.digest(private_key).as_string();
    // unwrap is fine because we know the length of hash_result
    // and that it will always contain valid hex data
    Address::try_from_str(&hash_result[24..]).unwrap()
}

#[test]
fn test_keygen() {
    let address = keygen("random2private#key");
    assert_eq!(
        address.as_string(),
        "4c37a9b5a8f660cb937af4b13310eeaee5b594b5" // last 20 bytes
    );
    //"9ccf1c4cf49cb403f61aafff4c37a9b5a8f660cb" // first 20 bytes
}
