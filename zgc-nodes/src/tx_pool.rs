use zgc_blockchain::TxData;
use zgc_common::H256;

use std::collections::HashMap;

pub struct TxPool<'a> {
    transactions: HashMap<H256, &'a TxData>, // tx amount - tx data
    amount_order: Vec<&'a TxData>,
}

impl TxPool<'_> {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            amount_order: Vec::new(),
        }
    }

    pub fn contains(&self, hash: &H256) -> bool {
        self.transactions.contains_key(hash)
    }

    pub fn insert(&mut self, tx: TxData) {
        let tx_box = Box::leak(Box::new(tx));
        self.transactions.insert(tx.signature(), tx_box);
        self.amount_order.push(tx_box);
        self.amount_order.sort();
    }

    pub fn peek_last(&self) -> Option<&&TxData> {
        self.amount_order.last()
    }

    pub fn remove_last(&mut self) {
        if let Some(tx_data) = self.amount_order.pop() {
            self.transactions.remove(&tx_data.signature());
        }
    }
}
