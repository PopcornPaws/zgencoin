use crate::TxData;

use serde::{Deserialize, Serialize};
use zgc_common::H256;

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
pub struct Block {
    height: usize,
    header: BlockHeader,
    data: BlockData,
}

impl Block {
    pub fn new(height: usize, header: BlockHeader, data: BlockData) -> Self {
        Self {
            height,
            header,
            data,
        }
    }

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
        self.header.as_string()
    }

    pub fn data(&self) -> &BlockData {
        &self.data
    }

    pub fn nonce(&self) -> u32 {
        self.header.nonce
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
pub struct BlockHeader {
    difficulty: u8,
    previous_hash: H256,
    data_hash: H256,
    nonce: u32,
}

impl BlockHeader {
    pub fn new(difficulty: u8, previous_hash: H256, data_hash: H256, nonce: u32) -> Self {
        Self {
            difficulty,
            previous_hash,
            data_hash,
            nonce,
        }
    }

    pub fn as_string(&self) -> String {
        // NOTE serialization won't fail
        serde_json::to_string(&self).expect("failed to serialize block header")
    }

    pub fn nonce_mut(&mut self) -> &mut u32 {
        &mut self.nonce
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
pub struct BlockData {
    pub tx: TxData,
    pub mint_tx: TxData,
}
