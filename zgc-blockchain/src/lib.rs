use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use zgc_common::{Address, H256};

pub struct Blockchain<'a> {
    height2hash: HashMap<usize, &'a str>,
    hash2block: HashMap<&'a str, Block>,
}

impl Blockchain<'_> {
    pub fn new() -> Self {
        let mut bc = Self {
            height2hash: HashMap::new(),
            hash2block: HashMap::new(),
        };
        bc.insert(Block::genesis());
        bc
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

#[derive(Deserialize, Serialize)]
pub struct Block {
    height: usize,
    header: BlockHeader,
    data: TxData,
}

impl Block {
    fn genesis() -> Self {
        // TODO derive default?
        Self {
            height: 0,
            header: BlockHeader {
                created_at: 0,
                previous_hash: H256::zero(),
                nonce: 0,
            },
            data: TxData {
                sender: Address::zero(),
                recipient: Address::zero(),
                amount: 0,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct BlockHeader {
    created_at: u64,
    previous_hash: H256,
    nonce: u32,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
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
