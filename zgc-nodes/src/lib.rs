use zgc_blockchain::{Blockchain, TxData};
use zgc_crypto::Hasher;

use async_std::net::TcpStream;
use async_std::prelude::*;
use std::collections::BTreeMap;

pub type TxPool = BTreeMap<u64, TxData>; // tx amount - tx data

pub enum NodeStatus<'a> {
    Forked(Vec<Blockchain<'a>>),
    Mining,
    Syncing,
}

pub struct Node<'a, 'b, T> {
    peers: Vec<TcpStream>,
    blockchain: Blockchain<'a>,
    status: NodeStatus<'b>,
    hasher: T,
}

impl<T> Node<'_, '_, T>
where
    T: Hasher,
{
    pub fn new(peers: Vec<TcpStream>, hasher: T) -> Self {
        Self {
            peers,
            blockchain: Blockchain::new(&hasher),
            status: NodeStatus::Syncing,
            hasher,
        }
    }

    pub async fn gossip(&mut self) -> Result<(), String> {
        // TODO choose peer randomly
        // rng.gen_range(0, self.peers.len())
        let random_peer = &mut self.peers[0];
        random_peer
            .write_all(
                serde_json::to_string(self.blockchain.last().unwrap())
                    .unwrap()
                    .as_bytes(),
            )
            .await
            .map_err(|e| format!("failed to send block data: {}", e))?;

        let mut buf = vec![0_u8; 1024];
        let peer_last_block = random_peer
            .read(&mut buf)
            .await
            .map_err(|e| format!("failed to read block data: {}", e))?;

        // TODO
        // if last block height is the same -> check hash to validate it
        // if hash is different -> Forked
        // if last block height is different -> break and set status to sync start syncing
        todo!();
    }

    pub async fn sync(&mut self) -> Result<(), String> {
        todo!();
    }
}
