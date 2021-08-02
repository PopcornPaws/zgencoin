use zgc_blockchain::{Blockchain, TxData};

use async_std::net::TcpStream;
use async_std::prelude::*;
use std::collections::BTreeMap;

pub type TxPool = BTreeMap<u64, TxData>; // tx amount - tx data

pub enum NodeStatus<'a> {
    Syncing,
    UpToDate,
    Forked(Vec<Blockchain<'a>>),
}


pub struct Node<'a, 'b> {
    peers: Vec<TcpStream>,
    blockchain: Blockchain<'a>,
    status: NodeStatus<'b>,
}

impl Node<'_, '_> {
    pub fn new(peers: Vec<TcpStream>) -> Self {
        Self {
            peers,
            blockchain: Blockchain::new(),
            status: NodeStatus::Syncing,
        }
    }

    pub async fn gossip(&mut self) -> Result<(), String> {
        // TODO choose peer randomly
        // rng.gen_range(0, self.peers.len())
        self.peers[0]
            .write_all(
                serde_json::to_string(self.blockchain.last().unwrap())
                    .unwrap()
                    .as_bytes(),
            )
            .await
            .map_err(|e| format!("failed to send block data: {}", e))?;

        let mut buf = vec![0_u8; 1024];
        let peer_last_block = self.peers[0]
            .read(&mut buf)
            .await
            .map_err(|e| format!("failed to read block data: {}", e))?;

        todo!();
    }

    pub async fn sync(&mut self) -> Result<(), String> {
        todo!();
    }
}
