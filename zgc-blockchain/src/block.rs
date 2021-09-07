use crate::TxData;

use serde::{Deserialize, Serialize};
use zgc_common::{Address, H256};

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
pub struct Block {
    height: usize,
    header: BlockHeader,
    data: BlockData,
}

impl Block {
    pub fn new(height: usize, header: BlockHeader, data: BlockData)
    pub fn genesis() -> Self {
        Self::default()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn previous_hash(&self) -> &H256 {
        &self.header.previous_hash
    }

    pub fn header_string(&self) -> String {
        self.header.to_string()
    }

    pub fn tx_data(&self) -> &TxData {
        &self.data.tx
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
pub struct BlockHeader {
    created_at: u128,
    difficulty: u8,
    previous_hash: H256,
    data_hash: H256,
    nonce: u32,
}

impl BlockHeader {
    pub fn new(created_at: u128, difficulty: u8, previous_hash: H256, nonce: u32) -> Self {
        Self {
            created_at,
            difficulty,
            previous_hash,
            nonce,
        }
    }

    pub fn to_string(&self) -> String {
        // NOTE serialization won't fail
        serde_json::to_string(&self).expect("failed to serialize block header")
    }

    pub fn set_nonce(&mut self, nonce: u32) {
        self.nonce = nonce
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
pub struct BlockData {
    pub tx: TxData,
    pub mint: TxData,
}
