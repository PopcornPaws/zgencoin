use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct Blockchain<'a> {
    height2hash: HashMap<usize, &'a str>,
    hash2block: HashMap<&'a str, Block>,
}

impl Blockchain<'_> {
    pub fn new() -> Self {
        let bc = Self {
            height2hash: HashMap::new(),
            hash2block: HashMap::new(),
        };
        bc.insert(Block::genesis());
        empty
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

    pub fn last(&self) -> Option<&Block> {
        self.find_height(self.hash2block.len())
    }
}

pub type Address = Hash<20>;
pub type H256 = Hash<32>;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Hash<const N: usize>([u8; N]);

impl<const N: usize> Hash<N> {
    pub fn from_str(string: &str) -> Result<Self, String> {
        let mut array = [0_u8; N];

        string
            .trim_start_matches("0x")
            .as_bytes()
            .chunks(2)
            .enumerate()
            .for_each(|(i, bytes)| {
                let parsed = u8::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16)
                    .expect("input contains invalid data");
                array[i] = parsed;
            });

        Ok(Self(array))
    }

    pub fn to_string(&self) -> String {
        self.0.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

//pub struct Blockchain<'a> {
//    block_finder: BlockFinder<'a>,
//    difficulty: u8,
//}
//
//impl Blockchain<'_> {
//    pub fn new_with_difficulty(difficulty: u8) -> Self {
//        let mut block_finder = BlockFinder::new();
//        block_finder.insert(Block::genesis());
//
//        Self {
//            block_finder,
//            difficulty,
//        }
//    }
//
//    pub fn insert_block(&mut self, block: Block) {
//        self.block_finder.insert(block)
//    }
//}

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
    previous_hash: H256,
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
