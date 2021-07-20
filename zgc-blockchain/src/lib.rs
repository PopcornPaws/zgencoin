use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct BlockFinder<'a> {
    height2hash: HashMap<usize, &'a str>,
    hash2block: HashMap<&'a str, Block>,
}

impl BlockFinder<'_> {
    pub fn new() -> Self {
        Self {
            height2hash: HashMap::new(),
            hash2block: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: Block) {
        // TODO use hashing function here
        let hash = Box::leak(Box::new(String::from("Some sha256 hash")));
        self.height2hash.insert(block.height, hash);
        self.hash2block.insert(hash, block);
    }

    pub fn find_hash(&self, hash: &str) -> Option<&Block> {
        self.hash2block.get(hash)
    }

    pub fn find_height(&self, height: usize) -> Option<&Block> {
        if let Some(hash) = self.height2hash.get(&height) {
            self.hash2block.get(*hash)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub struct Address([u8; 20]);

impl Address {
    pub fn from_str(string: &str) -> Result<Self, String> {
        let mut address = [0_u8; 20];

        string
            .trim_start_matches("0x")
            .as_bytes()
            .chunks(2)
            .enumerate()
            .for_each(|(i, bytes)| {
                let parsed = u8::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16)
                    .expect("input contains invalid data");
                address[i] = parsed;
            });

        Ok(Address(address))
    }

    pub fn to_string(&self) -> String {
        self.0.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

pub struct Blockchain<'a> {
    block_finder: BlockFinder<'a>,
    difficulty: u8,
}

impl Blockchain<'_> {
    pub fn new_with_difficulty(difficulty: u8) -> Self {
        let mut block_finder = BlockFinder::new();
        block_finder.insert(Block::genesis());

        Self {
            block_finder,
            difficulty,
        }
    }

    pub fn insert_block(&mut self, block: Block) {
        self.block_finder.insert(block)
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Block {
    height: usize,
    header: BlockHeader,
    data: TxData,
}

impl Block {
    fn genesis() -> Self {
        Self::default()
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct BlockHeader {
    created_at: u64,
    previous_hash: String,
    nonce: u32,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub struct TxData {
    sender: Address,
    recipient: Address,
    amount: usize,
}

impl TxData {
    pub fn new(sender: Address, recipient: Address, amount: usize) -> Self {
        Self {
            sender,
            recipient,
            amount,
        }
    }

    pub fn sender(&self) -> &Address {
        &self.sender
    }

    pub fn recipient(&self) -> &Address {
        &self.recipient
    }

    pub fn amount(&self) -> usize {
        self.amount
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn address() {
        let address_str = "0123456789abcdeffedcba9876543210aabbccdd";
        let address = Address::from_str(address_str).expect("failed to parse address string");
        assert_eq!(address.to_string(), address_str);

        let address_str_0x = String::from("0x") + address_str;
        let address = Address::from_str(&address_str_0x).expect("failed to parse address string");
        assert_eq!(address.to_string(), address_str);
    }
}
