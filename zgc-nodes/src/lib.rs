use zgc_blockchain::{Block, Blockchain, TxData, Wallet};
use zgc_common::{Address, H256};
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
    Block(Block),
    BlockRequest(usize),
}

pub struct Miner<'a, 'b, T> {
    peers: Vec<String>,
    listener: TcpListener,
    blockchain: Blockchain<'a>,
    status: NodeStatus<'b>,
    tx_pool: TxPool,
    hasher: T,
}

impl<'a, 'b, T: Hasher> Miner<'a, 'b, T> {
    pub fn new(own_ip: &str, ip_pool: Vec<String>, hasher: T) -> Result<Self, String> {
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

    pub fn mine(&mut self, loops: usize) -> Option<Block> {
        // TODO
        // mine in a loop
        // if block found, append to blockchain
        // throw out forks because our blockchain is the longest
        todo!();
    }
}

impl<T: Hasher> Node for Miner<'_, '_, T> {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<(), String> {
        let random_ip = self
            .peers
            .iter()
            .choose(rng)
            .expect("no peeers to connect to");

        let mut random_peer = TcpStream::connect(random_ip)
            .map_err(|e| format!("failed to establish tcp stream: {}", e))?;

        let gossip_msg = match self.status {
            NodeStatus::Forked(ref forks) => {
                GossipMessage::BlockRequest(forks[0].last().height() + 1)
            }
            NodeStatus::Mining => {
                if let Some(new_block) = self.mine(100) {
                    GossipMessage::Block(new_block)
                } else {
                    GossipMessage::Block(*self.blockchain.last())
                }
            }
            NodeStatus::Syncing => GossipMessage::BlockRequest(self.blockchain.last().height() + 1),
        };

        random_peer
            .write_all(&serde_json::to_vec(&gossip_msg).unwrap())
            .map_err(|e| format!("failed to send block data: {}", e))?;

        Ok(())
    }

    fn listen(&mut self) -> Result<(), String> {
        let (incoming_stream, peer_address) = self
            .listener
            .accept()
            .map_err(|e| format!("failed to accept incoming stream: {}", e))?;

        let peer_address_string = peer_address.to_string();

        // if new peer -> add to pool
        if !self.peers.contains(&peer_address_string) {
            self.peers.push(peer_address_string);
        }

        let mut deserializer = serde_json::Deserializer::from_reader(incoming_stream);
        match GossipMessage::deserialize(&mut deserializer) {
            Ok(GossipMessage::Block(block)) => {
                // do this for all forks
                // check whether bloch height is the same -> do nothing
                // if not, check parent hash and our last block's hash -> append to our blockchain
                // if not, and parent hash doesn't match -> add a fork
                // switch to longest fork
                println!("block = {:#?}", block);
            }
            Ok(GossipMessage::Transaction(tx_data)) => {
                // check whether tx_data is already in our TxPool
                // otherwise append it
                println!("tx data = {:#?}", tx_data);
            }
            Ok(GossipMessage::BlockRequest(height)) => {
                // set up a tcp stream to the incoming peer and send
                // our block at the given height, if any
                println!("requested block height = {:#?}", height);
            }
            Err(e) => println!("deserialization error: {}", e),
        }
        Ok(())
    }
}

pub struct ThinNode {
    peers: Vec<String>,
    listener: TcpListener,
    wallet: Wallet,
    tx_pool: BTreeMap<u64, H256>, // amount signature
}

impl ThinNode {
    pub fn new(own_ip: &str, peers: Vec<String>, private_key: String) -> Result<Self, String> {
        let listener =
            TcpListener::bind(own_ip).map_err(|e| format!("failed to bind tcp listener: {}", e))?;
        Ok(Self {
            peers,
            listener,
            wallet: Wallet::new(private_key),
            tx_pool: BTreeMap::new(),
        })
    }

    pub fn new_transaction(
        &mut self,
        amount: u64,
        recipient: Address,
        private_key: String,
    ) -> Result<TxData, String> {
        let tx_data = self
            .wallet
            .new_transaction(amount, recipient, private_key)?;
        self.tx_pool.insert(amount, tx_data.signature());
        Ok(tx_data)
    }
}

impl Node for ThinNode {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<(), String> {
        todo!();
    }

    fn listen(&mut self) -> Result<(), String> {
        todo!();
    }
}

pub trait Node {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<(), String>;
    fn listen(&mut self) -> Result<(), String>;
}
