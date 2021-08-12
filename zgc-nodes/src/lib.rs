use zgc_blockchain::{Block, Blockchain, TxData, Wallet};
use zgc_crypto::Hasher;

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

pub type TxPool = BTreeMap<u64, TxData>; // tx amount - tx data

pub enum NodeStatus<'a> {
    Forked(Vec<Blockchain<'a>>),
    Mining,
    Syncing,
}

#[derive(Serialize, Deserialize)]
pub enum GossipMessage {
    Transaction(TxData),
    LastBlock(Block),
}

pub struct Miner<'a, 'b, T> {
    peers: Vec<String>,
    listener: TcpListener,
    blockchain: Blockchain<'a>,
    status: NodeStatus<'b>,
    tx_pool: TxPool,
    hasher: T,
}

impl<'a, 'b, T> Miner<'a, 'b, T>
where
    T: Hasher,
{
    pub fn new(own_ip: &str, ip_pool: Vec<String>, hasher: T) -> Result<Miner<'a, 'b, T>, String> {
        let listener =
            TcpListener::bind(own_ip).map_err(|e| format!("failed to bind tcp listener: {}", e))?;
        Ok(Self {
            peers: ip_pool,
            listener,
            blockchain: Blockchain::new(&hasher),
            status: NodeStatus::Syncing,
            tx_pool: TxPool::new(),
            hasher,
        })
    }
}

impl<T> Node for Miner<'_, '_, T> {
    fn gossip(&self) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        let random_ip = self
            .peers
            .iter()
            .choose(&mut rng)
            .expect("no peeers to connect to");
        let mut random_peer = TcpStream::connect(random_ip)
            .map_err(|e| format!("failed to establish tcp stream: {}", e))?;
        random_peer
            .write_all(
                &serde_json::to_vec(&GossipMessage::LastBlock(
                    self.blockchain.last().unwrap().to_owned(),
                ))
                .unwrap(),
            )
            .map_err(|e| format!("failed to send block data: {}", e))?;

        Ok(())
    }

    fn listen(&mut self) -> Result<(), String> {
        //let mut buf = vec![0_u8; 1024];
        let (incoming_stream, peer_address) = self
            .listener
            .accept()
            .map_err(|e| format!("failed to accept incoming stream: {}", e))?;

        let peer_address_string = peer_address.to_string();

        if !self.peers.contains(&peer_address_string) {
            self.peers.push(peer_address_string);
        }

        let mut deserializer = serde_json::Deserializer::from_reader(incoming_stream);
        match GossipMessage::deserialize(&mut deserializer) {
            Ok(GossipMessage::LastBlock(block)) => println!("block = {:#?}", block),
            Ok(GossipMessage::Transaction(tx_data)) => println!("tx data = {:#?}", tx_data),
            Err(e) => println!("deserialization error: {}", e),
        }
        // TODO
        // if last block height is the same -> check hash to validate it
        // if hash is different -> Forked
        // if last block height is different -> break and set status to sync start syncing
        Ok(())
    }
}

pub struct ThinNode {
    peers: Vec<String>,
    listener: TcpListener,
    wallet: Wallet,
    tx_pool: BTreeMap,
}

impl Node for ThinNode {}

pub trait Node {
    fn gossip(&self) -> Result<(), String>;
    fn listen(&mut self) -> Result<(), String>;
}
