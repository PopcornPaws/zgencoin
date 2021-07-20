use zgc_blockchain::{Blockchain, TxData};

use async_std::net::TcpStream;
use std::collections::BTreeMap;

pub struct Node<'a> {
    peers: Vec<TcpStream>,
    blockchain: Blockchain<'a>,
    tx_pool: BTreeMap<usize, TxData>,
}

impl Node<'_> {
    pub fn new(peers: Vec<TcpStream>) -> Self {
        Self {
            peers,
            blockchain: Blockchain::new(),
            tx_pool: BTreeMap::new(),
        }
    }

    pub async fn sync(&mut self) -> Result<(), String> {
        // TODO choose peer randomly
        // rng.gen_range(0, self.peers.len())
        self.peers[0]
            .write_all(serde_json::to_string(self.blockchain.last().unwrap()))
            .await
            .map_err(|e| format!("failed to send block data: {}", e))?;

        let mut buf = vec![0_u8; 1024];
        let peer_last_block = self
            .read(&mut buf)
            .await
            .map_err(|e| format!("failed to read block data: {}"))?;
    }
}
