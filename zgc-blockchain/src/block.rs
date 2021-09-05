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
        serde_json::to_string(&self.header).expect("failed to serialize block header")
    }

    pub fn tx_data(&self) -> &TxData {
        &self.data.tx
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
struct BlockHeader {
    created_at: u64,
    previous_hash: H256,
    nonce: u32,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default, Debug)]
struct BlockData {
    tx: TxData,
    mint: TxData,
}
