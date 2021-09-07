mod block;
mod transaction;
mod wallet;

pub use block::{Block, BlockHeader};
pub use transaction::TxData;
pub use wallet::Wallet;

use std::collections::HashMap;
use zgc_common::H256;
use zgc_crypto::Hasher;

#[derive(Clone)]
pub struct Blockchain<'a> {
    height2hash: HashMap<usize, &'a H256>,
    hash2block: HashMap<&'a H256, Block>,
}

impl Blockchain<'_> {
    pub fn new(genesis: Block, hasher: &impl Hasher) -> Self {
        let mut bc = Self {
            height2hash: HashMap::new(),
            hash2block: HashMap::new(),
        };
        bc.insert(genesis, hasher);
        bc
    }

    pub fn insert(&mut self, block: Block, hasher: &impl Hasher) {
        // expect/unwrap is fine here because the derived
        // Serialize will (hopefully) never fail
        let hash = Box::leak(Box::new(hasher.digest(block.header_string())));
        self.height2hash.insert(block.height(), hash);
        self.hash2block.insert(hash, block);
    }

    pub fn find_hash(&self, hash: &H256) -> Option<&Block> {
        self.hash2block.get(hash)
    }

    pub fn find_height(&self, height: usize) -> Option<&Block> {
        if let Some(hash) = self.height2hash.get(&height) {
            self.hash2block.get(hash)
        } else {
            None
        }
    }

    pub fn last_block(&self) -> &Block {
        // NOTE unwrap is fine because there's at least the genesis block
        self.find_height(self.hash2block.len() - 1).unwrap()
    }

    pub fn last_block_hash(&self) -> &H256 {
        let last_height = self.height2hash.len() - 1;
        // NOTE unwrap is fine because there's at least the genesis block
        self.height2hash.get(&last_height).unwrap()
    }
}
