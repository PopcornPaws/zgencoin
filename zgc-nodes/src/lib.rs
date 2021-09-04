use zgc_blockchain::{Block, Blockchain, TxData, Wallet};
use zgc_common::{Address, H256};
use zgc_crypto::Hasher;

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io::Write;
use std::net::{SocketAddrV4, TcpListener, TcpStream};

pub struct TxPool<'a> {
    transactions: HashMap<H256, &'a TxData>, // tx amount - tx data
    amount_order: Vec<&'a TxData>,
}

impl TxPool<'_> {
    fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            amount_order: Vec::new(),
        }
    }

    fn contains(&self, hash: &H256) -> bool {
        self.transactions.contains_key(hash)
    }

    fn insert(&mut self, tx: TxData) {
        let tx_box = Box::leak(Box::new(tx));
        self.transactions.insert(tx.signature(), tx_box);
        self.amount_order.push(tx_box);
        self.amount_order.sort();
    }

    fn peek_last(&self) -> Option<&&TxData> {
        self.amount_order.last()
    }

    fn remove_last(&mut self) {
        if let Some(tx_data) = self.amount_order.pop() {
            self.transactions.remove(&tx_data.signature());
        }
    }
}

pub enum NodeStatus {
    Forked(Vec<Block>),
    Mining,
    Syncing,
}

#[derive(Serialize, Deserialize)]
pub enum GossipMessage {
    Transaction(TxData),
    Block(Block),
    BlockRequest(usize),
}

pub struct Miner<'bc, 'tx, T> {
    peers: Vec<SocketAddrV4>,
    listener: TcpListener,
    blockchain: Blockchain<'bc>,
    status: NodeStatus,
    tx_pool: TxPool<'tx>,
    hasher: T,
}

impl<T: Hasher> Miner<'_, '_, T> {
    pub fn new(own_ip: &str, ip_pool: Vec<String>, hasher: T) -> Result<Self, String> {
        let listener =
            TcpListener::bind(own_ip).map_err(|e| format!("failed to bind tcp listener: {}", e))?;

        let peers = ip_pool
            .into_iter()
            .map(|ip| ip.parse().expect("invalid ip address format"))
            .collect();

        Ok(Self {
            peers,
            listener,
            blockchain: Blockchain::new(Block::genesis(), &hasher),
            status: NodeStatus::Syncing,
            tx_pool: TxPool::new(),
            hasher,
        })
    }

    pub fn mine(&mut self, loops: usize) -> Option<Block> {
        // TODO
        // mine in a loop
        // if block found, append to blockchain
        // throw out forks because our blockchain is the longest?
        // get the highest amount to mine first
        // mint money for ourselves as a fraction of the mined amount
        todo!();
    }

    pub fn update_blockchain_with_fork(&mut self, blockchain: Blockchain<'_>, block: Block) {
        todo!();
    }
}

pub struct MessageToPeer {
    msg: GossipMessage,
    peer: SocketAddrV4,
}

impl<T: Hasher> Node for Miner<'_, '_, T> {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<MessageToPeer, String> {
        let gossip_msg = match self.status {
            NodeStatus::Forked(ref forks) => {
                GossipMessage::BlockRequest(self.blockchain.last_block().height() + 1)
            }
            NodeStatus::Mining => {
                if let Some(new_block) = self.mine(100) {
                    GossipMessage::Block(new_block)
                } else {
                    GossipMessage::Block(*self.blockchain.last_block())
                }
            }
            NodeStatus::Syncing => {
                GossipMessage::BlockRequest(self.blockchain.last_block().height() + 1)
            }
        };

        let peer = *self
            .peers
            .iter()
            .choose(rng)
            .expect("no peeers to connect to");

        Ok(MessageToPeer {
            msg: gossip_msg,
            peer,
        })
    }

    fn listen(&mut self) -> Result<Option<MessageToPeer>, String> {
        let (incoming_stream, peer_address) = self
            .listener
            .accept()
            .map_err(|e| format!("failed to accept incoming stream: {}", e))?;

        // if new peer -> add to pool
        let peer_address: SocketAddrV4 = peer_address
            .to_string()
            .parse()
            .map_err(|_| "invalid peer address format".to_string())?;

        if !self.peers.contains(&peer_address) {
            self.peers.push(peer_address);
        }

        let mut deserializer = serde_json::Deserializer::from_reader(incoming_stream);
        match GossipMessage::deserialize(&mut deserializer) {
            Ok(GossipMessage::Block(incoming_block)) => {
                // do this for all forks
                // check whether bloch height is the same -> do nothing
                // if not, check parent hash and our last block's hash -> append to our blockchain
                // if not, and parent hash doesn't match -> add a fork
                // switch to longest fork
                match self.status {
                    NodeStatus::Forked(ref forks) => {
                        for block in forks.into_iter() {
                            if &self.hasher.digest(block.header_string())
                                == incoming_block.previous_hash()
                            {
                                //self.update_blockchain_with_fork(fork.to_owned(), incoming_block);
                                self.status = NodeStatus::Mining;
                                break;
                            }
                        }
                        if self.blockchain.last_block_hash() == incoming_block.previous_hash() {
                            self.blockchain.insert(block.to_owned(), &self.hasher);
                            self.status = NodeStatus::Mining;
                        }
                        // check case when an n^th fork occurs from the same block
                        todo!();
                    }
                    NodeStatus::Mining => {
                        if self.blockchain.last_block_hash() == block.previous_hash() {
                            self.blockchain.insert(block.to_owned(), &self.hasher);
                        } else if self.blockchain.find_hash(block.previous_hash()).is_some() {
                            todo!()
                            //self.status = NodeStatus::Forked()
                        }
                    }
                    NodeStatus::Syncing => {
                        if self.blockchain.last_block_hash() == block.previous_hash() {
                            self.blockchain.insert(block.to_owned(), &self.hasher);
                        } else if self.blockchain.last_block().height() == block.height() {
                            self.status = NodeStatus::Mining
                        }
                    }
                }
            }
            Ok(GossipMessage::Transaction(tx_data)) => {
                // check whether tx_data is already in our TxPool
                // otherwise append it
                if !self.tx_pool.contains(&tx_data.signature()) {
                    self.tx_pool.insert(tx_data);
                }
                println!("tx data = {:#?}", tx_data);
            }
            Ok(GossipMessage::BlockRequest(height)) => {
                // set up a tcp stream to the incoming peer and send
                // our block at the given height, if any
                let requested_block = if let Some(block) = self.blockchain.find_height(height) {
                    block
                } else {
                    self.blockchain.last_block()
                };

                println!("requested block height = {:#?}", height);

                return Ok(Some(MessageToPeer {
                    msg: GossipMessage::Block(requested_block.to_owned()),
                    peer: peer_address,
                }));
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(None)
    }
}

fn send_message(msg_to_peer: MessageToPeer) -> Result<(), String> {
    let mut peer = TcpStream::connect(msg_to_peer.peer)
        .map_err(|e| format!("failed to establish tcp stream: {}", e))?;

    // unwrap is fine here because gossip msg can always be serialized into a vec
    peer.write_all(&serde_json::to_vec(&msg_to_peer.msg).unwrap())
        .map_err(|e| format!("failed to send block data: {}", e))?;

    Ok(())
}

pub struct ThinNode {
    peers: Vec<String>,
    listener: TcpListener,
    wallet: Wallet,
    tx_pool: Vec<H256>,
}

impl ThinNode {
    pub fn new(own_ip: &str, peers: Vec<String>, private_key: String) -> Result<Self, String> {
        let listener =
            TcpListener::bind(own_ip).map_err(|e| format!("failed to bind tcp listener: {}", e))?;
        Ok(Self {
            peers,
            listener,
            wallet: Wallet::new(private_key),
            tx_pool: Vec::new(),
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
        self.tx_pool.push(tx_data.signature());
        Ok(tx_data)
    }
}

impl Node for ThinNode {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<MessageToPeer, String> {
        todo!();
    }

    fn listen(&mut self) -> Result<Option<MessageToPeer>, String> {
        todo!();
    }
}

pub trait Node {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<MessageToPeer, String>;
    fn listen(&mut self) -> Result<Option<MessageToPeer>, String>;
}
