use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use zgc_common::{Address, H256};
use zgc_crypto::{Hasher, Sha256};

pub struct Blockchain<'a> {
    height2hash: HashMap<usize, &'a str>,
    hash2block: HashMap<&'a str, Block>,
}

impl Blockchain<'_> {
    pub fn new(hasher: &impl Hasher) -> Self {
        let mut bc = Self {
            height2hash: HashMap::new(),
            hash2block: HashMap::new(),
        };
        bc.insert(Block::genesis(), hasher);
        bc
    }

    pub fn insert(&mut self, block: Block, hasher: &impl Hasher) {
        // expect/unwrap is fine here because the derived
        // Serialize will (hopefully) never fail
        let hash = Box::leak(Box::new(
            hasher
                .digest(
                    serde_json::to_string(&block.header).expect("failed to serialize block header"),
                )
                .to_string(),
        ));
        self.height2hash.insert(block.height, hash);
        self.hash2block.insert(hash, block);
    }

    pub fn find_hash(&self, hash: &str) -> Option<&Block> {
        self.hash2block.get(hash)
    }

    pub fn find_height(&self, height: usize) -> Option<&Block> {
        if let Some(hash) = self.height2hash.get(&height) {
            self.hash2block.get(hash)
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<&Block> {
        self.find_height(self.hash2block.len() - 1)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Block {
    height: usize,
    header: BlockHeader,
    data: TxData,
}

impl Block {
    fn genesis() -> Self {
        Self {
            height: 0,
            header: BlockHeader {
                created_at: 0,
                previous_hash: H256::zero(),
                nonce: 0,
            },
            data: TxData {
                signature: H256::zero(),
                sender: Address::zero(),
                recipient: Address::zero(),
                amount: 0,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Default, Eq, PartialEq, Debug)]
pub struct BlockHeader {
    created_at: u64,
    previous_hash: H256,
    nonce: u32,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
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
