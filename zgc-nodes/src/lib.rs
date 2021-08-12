use std::collections::BTreeMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

use rand::seq::IteratorRandom;
use serde::Serialize;

use zgc_blockchain::{Block, Blockchain, TxData, Wallet};
use zgc_common::{Address, H256};
use zgc_crypto::Hasher;

#[derive(Serialize)]
pub enum GossipMessage {
    Transaction(TxData),
    Block(Block),
    BlockRequest(usize),
}

pub trait Node {
    fn gossip(&self) -> Result<(), String>;
    fn listen(&self) -> Result<(), String>;
}

pub type TxPool = BTreeMap<u64, TxData>;

pub struct Miner<'a, T> {
    peers: Vec<String>, // 185.32.43.1:8999
    listener: TcpListener,
    blockchain: Blockchain<'a>,
    tx_pool: TxPool,
    hasher: T,
}

impl<T: Hasher> Miner<'_, T> {
    pub fn new(own_ip: &str, peers: Vec<String>, hasher: T) -> Result<Self, String> {
        let listener =
            TcpListener::bind(own_ip).map_err(|e| format!("failed to bind tcp listener: {}", e))?;

        Ok(Self {
            peers,
            listener,
            blockchain: Blockchain::new(&hasher),
            tx_pool: TxPool::new(),
            hasher,
        })
    }

    pub fn mine(&mut self) -> Result<(), String> {
        todo!();
    }
}

impl<T> Node for Miner<'_, T> {
    fn gossip(&self) -> Result<(), String> {
        // expect is fine here...
        let mut rng = rand::thread_rng();
        let random_ip = self
            .peers
            .iter()
            .choose(&mut rng)
            .expect("no peers to connect to");
        let mut random_peer = TcpStream::connect(random_ip)
            .map_err(|e| format!("failed to establish tcp stream: {}", e))?;

        random_peer
            .write_all(
                &serde_json::to_vec(&GossipMessage::Block(
                    self.blockchain.last().unwrap().to_owned(),
                ))
                .unwrap(),
            )
            .unwrap();

        Ok(())
    }

    fn listen(&self) -> Result<(), String> {
        todo!();
    }
}

pub struct ThinNode {
    peers: Vec<String>,
    listener: TcpListener,
    wallet: Wallet,
    tx_pool: BTreeMap<u64, H256>,
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
