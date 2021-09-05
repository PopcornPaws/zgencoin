use crate::node::{Node, NodeStatus};
use crate::tx_pool::TxPool;
use crate::{GossipMessage, MessageToPeer};

use zgc_blockchain::{Block, Blockchain};
use zgc_crypto::Hasher;

use rand::seq::IteratorRandom;
use serde::Deserialize;

use std::net::{SocketAddrV4, TcpListener};

pub struct Miner<'bc, 'ns, 'tx, T> {
    peers: Vec<SocketAddrV4>,
    listener: TcpListener,
    blockchain: Blockchain<'bc>,
    status: NodeStatus<'ns>,
    tx_pool: TxPool<'tx>,
    hasher: T,
}

impl<T: Hasher> Miner<'_, '_, '_, T> {
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

    pub fn update_tx_pool(&self, block: &Block) {
        todo!();
        //if block.data()
    }

    pub fn update_blockchain_with_fork(&mut self, blockchain: Blockchain<'_>, block: Block) {
        todo!();
    }
}

impl<T: Hasher> Node for Miner<'_, '_, '_, T> {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<MessageToPeer, String> {
        let gossip_msg = match self.status {
            NodeStatus::Mining => {
                if let Some(new_block) = self.mine(100) {
                    GossipMessage::Block(new_block)
                } else {
                    GossipMessage::Block(*self.blockchain.last_block())
                }
            }
            NodeStatus::Forked(_) | NodeStatus::Syncing => {
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
                    NodeStatus::Forked(ref mut forks) => {
                        for fork in forks.into_iter() {
                            if fork.last_block_hash() == incoming_block.previous_hash() {
                                fork.insert(incoming_block, &self.hasher);
                                // probably not a long fork, so it's not
                                // expensive to clone it via to_owned
                                self.status = NodeStatus::Forked(vec![fork.to_owned()]);
                                break;
                            }
                        }
                        if self.blockchain.last_block_hash() == incoming_block.previous_hash() {
                            self.blockchain.insert(incoming_block, &self.hasher);
                            self.status = NodeStatus::Mining;
                        }
                        // check case when an n^th fork occurs from the same block
                        todo!();
                    }
                    NodeStatus::Mining => {
                        if self.blockchain.last_block_hash() == incoming_block.previous_hash() {
                            self.blockchain.insert(incoming_block, &self.hasher);
                        } else if self
                            .blockchain
                            .find_hash(incoming_block.previous_hash())
                            .is_some()
                        {
                            self.status = NodeStatus::Forked(vec![Blockchain::new(
                                incoming_block,
                                &self.hasher,
                            )]);
                        }
                    }
                    NodeStatus::Syncing => {
                        if self.blockchain.last_block_hash() == incoming_block.previous_hash() {
                            self.blockchain.insert(incoming_block, &self.hasher);
                        } else if self.blockchain.last_block().height() == incoming_block.height() {
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
