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

    pub fn mine(&mut self) -> Result<(), String> {
        todo!();
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

        // write a block

        random_peer
            .write_all(
                &serde_json::to_vec(&GossipMessage::Block(
                    self.blockchain.last().unwrap().to_owned(),
                ))
                .unwrap(),
            )
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
    pub fn new() -> Self {
        todo!();
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
    fn gossip(&self) -> Result<(), String> {
        todo!();
    }

    fn listen(&mut self) -> Result<(), String> {
        todo!();
    }
}

pub trait Node {
    fn gossip(&self) -> Result<(), String>;
    fn listen(&mut self) -> Result<(), String>;
}
