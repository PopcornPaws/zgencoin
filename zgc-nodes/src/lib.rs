use zgc_blockchain::{Block, Blockchain, TxData};
use zgc_crypto::Hasher;

use async_trait::async_trait;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub type TxPool = BTreeMap<u64, TxData>; // tx amount - tx data

pub enum NodeStatus<'a> {
    Forked(Vec<Blockchain<'a>>),
    Mining,
    Syncing,
}

pub enum GossipMessage {
    Transaction(TxData),
    LastBlock(Block),
    DiscoveredPeers(Vec<TcpStream>),
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
    pub async fn new(
        own_ip: &str,
        ip_pool: Vec<String>,
        hasher: T,
    ) -> Result<Miner<'a, 'b, T>, String> {
        let listener = TcpListener::bind(own_ip)
            .await
            .map_err(|e| format!("failed to bind tcp listener: {}", e))?;
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

#[async_trait]
impl<T> Node for Miner<'_, '_, T>
where
    T: std::marker::Send,
{
    async fn gossip(&mut self) -> Result<(), String> {
        let random_index = rand::thread_rng().gen_range(0..self.peers.len());
        let random_ip = &self.peers[random_index];
        let mut random_peer = TcpStream::connect(random_ip)
            .await
            .map_err(|e| format!("failed to establish tcp stream: {}", e))?;
        random_peer
            .write_all(
                serde_json::to_string(self.blockchain.last().unwrap())
                    .unwrap()
                    .as_bytes(),
            )
            .await
            .map_err(|e| format!("failed to send block data: {}", e))?;

        let mut buf = vec![0_u8; 1024];
        let (mut incoming_stream, _) = self
            .listener
            .accept()
            .await
            .map_err(|e| format!("failed to accept incoming stream: {}", e))?;

        let peer_last_block = incoming_stream
            .read(&mut buf)
            .await
            .map_err(|e| format!("failed to read block data: {}", e))?;

        println!("last block = {:#?}", peer_last_block);
        // TODO
        // if last block height is the same -> check hash to validate it
        // if hash is different -> Forked
        // if last block height is different -> break and set status to sync start syncing
        Ok(())
    }
}

#[async_trait]
pub trait Node {
    async fn gossip(&mut self) -> Result<(), String>;
}

struct Peers {
    connections: HashMap<String, TcpStream>,
}

impl Peers {
    pub async fn new(ip_pool: Vec<String>) -> Result<Self, String> {
        let mut connections = HashMap::with_capacity(ip_pool.len());
        for ip_address in ip_pool.into_iter() {
            let stream = TcpStream::connect(&ip_address)
                .await
                .map_err(|e| format!("failed to establish tcp stream: {}", e))?;
            connections.insert(ip_address, stream);
        }
        Ok(Self { connections })
    }

    pub fn get_random_peer(&mut self) -> Option<&mut TcpStream> {
        let mut rng = rand::thread_rng();
        self.connections.values_mut().choose(&mut rng)
    }
}
