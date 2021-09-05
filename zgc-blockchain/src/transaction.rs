use serde::{Deserialize, Serialize};
use zgc_common::{Address, H256};

use std::cmp::Ordering;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default)]
pub struct TxData {
    pub(crate) signature: H256,
    pub(crate) sender: Address,
    pub(crate) recipient: Address,
    pub(crate) amount: u64,
}

impl TxData {
    pub fn signature(&self) -> H256 {
        self.signature
    }
}

impl PartialEq for TxData {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl Eq for TxData {}

impl PartialOrd for TxData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.amount.cmp(&other.amount))
    }
}

impl Ord for TxData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.cmp(&other.amount)
    }
}
