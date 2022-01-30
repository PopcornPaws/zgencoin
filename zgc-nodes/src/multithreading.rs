use crate::tx_pool::TxPool;
use zgc_blockchain::{BlockHeader, TxData};
use zgc_common::H256;
use zgc_crypto::Hasher;

const MAX_MINING_LOOPS: usize = 1000;

pub struct Miner<'tx, T> {
    tx_pool: TxPool<'tx>,
    difficulty: u8,
    last_hash: H256,
    hasher: T,
}

// listen to incoming transactions on the main thread
// mine continuously on a worker thread

impl<T: Hasher> Miner<'_, T> {
    pub fn new(difficulty: u8, hasher: T) -> Self {
        Self {
            tx_pool: TxPool::new(),
            difficulty,
            last_hash: H256::zero(),
            hasher,
        }
    }

    pub fn add_tx(&mut self, tx_data: &TxData) {
        if !self.tx_pool.contains(&tx_data.signature) && tx_data.signature != H256::max() {
            println!(
                "[MINER] received tx with signature = {:?}",
                tx_data.signature
            );
            self.tx_pool.insert(*tx_data);
        }
    }

    pub fn mine(&mut self, init_nonce: u32) {
        if let Some(&tx) = self.tx_pool.peek_last() {
            println!("[MINER] started mining with nonce: {}", init_nonce);
            let target_hash = H256::masked(self.difficulty);
            let mut new_block_header = BlockHeader::new(
                self.difficulty,
                H256::zero(), // TODO last block's hash
                self.hasher.digest(tx.as_string()),
                init_nonce,
            );

            for _ in 0..MAX_MINING_LOOPS {
                let header_hash = self.hasher.digest(new_block_header.as_string());
                if header_hash < target_hash {
                    self.last_hash = header_hash;
                    println!("[MINER] found hash: {:?}", self.last_hash);
                    break;
                } else {
                    let nonce = new_block_header.nonce_mut();
                    *nonce = nonce.wrapping_add(1);
                }
            }
        }
    }
}
