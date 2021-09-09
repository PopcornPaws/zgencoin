use crate::node::{Node, NodeStatus};
use crate::tx_pool::TxPool;
use crate::{GossipMessage, GossipResult, MessageToPeer};

use zgc_blockchain::{Block, BlockData, BlockHeader, Blockchain, Wallet};
use zgc_common::H256;
use zgc_crypto::Hasher;

use rand::seq::IteratorRandom;
use rand::Rng;
use serde::Deserialize;

use std::net::{SocketAddrV4, TcpListener};

pub struct Miner<'bc, 'ns, 'tx, T> {
    peers: Vec<SocketAddrV4>,
    listener: TcpListener,
    difficulty: u8,
    decimals: u8,
    blockchain: Blockchain<'bc>,
    status: NodeStatus<'ns>,
    tx_pool: TxPool<'tx>,
    hasher: T,
    wallet: Wallet,
}

impl<T: Hasher> Miner<'_, '_, '_, T> {
    pub fn new(
        own_ip: &str,
        ip_pool: &[&str],
        hasher: T,
        difficulty: u8,
        decimals: u8,
        private_key: &str,
    ) -> Result<Self, String> {
        let listener = TcpListener::bind(own_ip)
            .map_err(|e| format!("failed to bind tcp listener: {}", e))?;

        let peers = ip_pool
            .iter()
            .map(|ip| ip.parse().expect("invalid ip address format"))
            .collect();

        Ok(Self {
            peers,
            listener,
            difficulty,
            decimals,
            blockchain: Blockchain::new(Block::genesis(), &hasher),
            status: NodeStatus::Syncing,
            tx_pool: TxPool::new(),
            hasher,
            wallet: Wallet::new(private_key),
        })
    }

    pub fn mine(&mut self, loops: usize, init_nonce: u32) -> Option<Block> {
        // TODO
        // mine in a loop
        // if block found, append to blockchain
        // throw out forks because our blockchain is the longest?
        // get the highest amount to mine first
        // mint money for ourselves as a fraction of the mined amount
        println!("[MINER] started mining with nonce: {}", init_nonce);
        if let Some(&tx) = self.tx_pool.peek_last() {
            let target_hash = H256::masked(self.difficulty);
            let mut new_block_header = BlockHeader::new(
                self.difficulty,
                *self.blockchain.last_block_hash(),
                self.hasher.digest(tx.as_string()),
                init_nonce,
            );
            let mut block: Option<Block> = None;
            for _ in 0..loops {
                let header_hash = self.hasher.digest(new_block_header.as_string());
                if header_hash < target_hash {
                    let self_mint = self.compute_self_mint_amount(tx.amount);
                    let new_mint_tx = self.wallet.new_self_mint(self_mint);
                    let block_data = BlockData {
                        tx: *tx,
                        mint_tx: new_mint_tx,
                    };
                    let new_block = Block::new(self.blockchain.len(), new_block_header, block_data);
                    println!("[MINER] successfully mined block with nonce: {}", new_block.nonce());
                    self.blockchain.insert(new_block, &self.hasher);
                    block = Some(new_block);
                    self.tx_pool.remove_last();
                    break;
                } else {
                    let nonce = new_block_header.nonce_mut();
                    *nonce = nonce.wrapping_add(1);
                }
            }
            block
        } else {
            None
        }
    }

    fn compute_self_mint_amount(&self, mined_amount: u64) -> u64 {
        let self_mint = mined_amount * self.decimals as u64 * self.difficulty as u64 / 100;
        self_mint / self.decimals as u64
    }
}

impl<T: Hasher> Node for Miner<'_, '_, '_, T> {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> GossipResult {
        let gossip_msg = match self.status {
            NodeStatus::Mining => {
                if let Some(new_block) = self.mine(100, rng.gen()) {
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

    fn listen(&mut self) -> GossipResult {
        //let incoming_stream = self.listener.incoming().last().unwrap().unwrap();
        //let peer_address = incoming_stream.peer_addr().map_err(|e| e.to_string())?;
        let (incoming_stream, peer_address) = self
            .listener
            .accept()
            .map_err(|e| format!("failed to accept incoming stream: {}", e))?;

        let peer_address: SocketAddrV4 = peer_address
            .to_string()
            .parse()
            .map_err(|_| "invalid peer address format".to_string())?;

        // if new peer -> add to pool
        // in the blocking case this is a problem
        //if !self.peers.contains(&peer_address) {
        //    self.peers.push(peer_address);
        //}

        let mut deserializer = serde_json::Deserializer::from_reader(incoming_stream);
        match GossipMessage::deserialize(&mut deserializer) {
            Ok(GossipMessage::Block(incoming_block)) => {
                // do this for all forks
                // check whether bloch height is the same -> do nothing
                // if not, check parent hash and our last block's hash -> append to our blockchain
                // if not, and parent hash doesn't match -> add a fork
                // switch to longest fork
                println!("[MINER] received block with height: {}", incoming_block.height()); 
                match self.status {
                    NodeStatus::Forked(ref mut forks) => {
                        for fork in forks.iter_mut() {
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
                            println!("[MINER] synced successfully"); 
                            self.status = NodeStatus::Mining
                        }
                    }
                }
            }
            Ok(GossipMessage::Transaction(tx_data)) => {
                // check whether tx_data is already in our TxPool
                // otherwise append it
                if !self.tx_pool.contains(&tx_data.signature) && tx_data.signature != H256::max() {
                    println!("[MINER] received tx with signature = {:?}", tx_data.signature);
                    self.tx_pool.insert(tx_data);
                }
            }
            Ok(GossipMessage::BlockRequest(height)) => {
                // set up a tcp stream to the incoming peer and send
                // our block at the given height, if any
                let requested_block = if let Some(block) = self.blockchain.find_height(height) {
                    block
                } else {
                    self.blockchain.last_block()
                };

                println!("[MINER] requested block height = {}", height);

                return Ok(MessageToPeer {
                    msg: GossipMessage::Block(requested_block.to_owned()),
                    peer: peer_address,
                });
            }
            Ok(GossipMessage::Ping) => println!("[MINER] received ping from {:?}", peer_address),
            Err(e) => return Err(e.to_string()),
        }

        Ok(MessageToPeer {
            msg: GossipMessage::Ping,
            peer: peer_address,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zgc_common::Address;
    use zgc_crypto::Sha256;

    #[test]
    fn test_easy_mine() {
        let own_ip = "127.0.0.1:7788";
        let ip_pool = &["127.0.0.1:7789"];
        let hasher = Sha256::new();
        let mut miner = Miner::new(own_ip, ip_pool, hasher, 1, 5, "miner_priv@key").unwrap();

        let peer_priv_key = "peer_priv@key";
        let peer_wallet = Wallet::new(peer_priv_key);
        let recipient = Address::try_from_str("9961003ec5189ff5bd86418247db65c1c36cadf2").unwrap();
        let new_tx = peer_wallet
            .new_transaction(150, recipient, peer_priv_key)
            .unwrap();
        miner.tx_pool.insert(new_tx);
        let block = miner.mine(100, 100_000).unwrap();

        assert_eq!(block.height(), 1); // comes right after the genesis block
        assert_eq!(
            block.previous_hash(),
            &miner.hasher.digest(Block::genesis().header_string())
        );
        assert_eq!(block.data().tx.signature, new_tx.signature);
        assert_eq!(block.data().mint_tx.signature, H256::max());
        assert_eq!(block.data().mint_tx.amount, 1);
        assert_eq!(miner.blockchain.len(), 2);
        assert_eq!(block.nonce(), 100_002);
    }

    #[test]
    fn test_medium_mine() {
        let own_ip = "127.0.0.1:7780";
        let ip_pool = &["127.0.0.1:7781"];
        let hasher = Sha256::new();
        let mut miner = Miner::new(own_ip, ip_pool, hasher, 2, 5, "miner_priv@key").unwrap();

        let peer_priv_key = "peer_priv@key";
        let peer_wallet = Wallet::new(peer_priv_key);
        let recipient = Address::try_from_str("9961003ec5189ff5bd86418247db65c1c36cadf2").unwrap();
        let new_tx = peer_wallet
            .new_transaction(15_000, recipient, peer_priv_key)
            .unwrap();
        miner.tx_pool.insert(new_tx);
        let block = miner.mine(1000, 100_000).unwrap();

        assert_eq!(block.height(), 1); // comes right after the genesis block
        assert_eq!(
            block.previous_hash(),
            &miner.hasher.digest(Block::genesis().header_string())
        );
        assert_eq!(block.data().tx.signature, new_tx.signature);
        assert_eq!(block.data().mint_tx.signature, H256::max());
        assert_eq!(block.data().mint_tx.amount, 300);
        assert_eq!(miner.blockchain.len(), 2);
        assert_eq!(block.nonce(), 100_229);
    }

    #[test]
    fn test_hard_mine() {
        let own_ip = "127.0.0.1:7782";
        let ip_pool = &["127.0.0.1:7783"];
        let hasher = Sha256::new();
        let mut miner = Miner::new(own_ip, ip_pool, hasher, 3, 5, "miner_priv@key").unwrap();

        let peer_priv_key = "peer_priv@key";
        let peer_wallet = Wallet::new(peer_priv_key);
        let recipient = Address::try_from_str("9961003ec5189ff5bd86418247db65c1c36cadf2").unwrap();
        let new_tx = peer_wallet
            .new_transaction(2000, recipient, peer_priv_key)
            .unwrap();
        miner.tx_pool.insert(new_tx);
        let block = miner.mine(10000, 100_000).unwrap();

        assert_eq!(block.height(), 1); // comes right after the genesis block
        assert_eq!(
            block.previous_hash(),
            &miner.hasher.digest(Block::genesis().header_string())
        );
        assert_eq!(block.data().tx.signature, new_tx.signature);
        assert_eq!(block.data().mint_tx.signature, H256::max());
        assert_eq!(block.data().mint_tx.amount, 60);
        assert_eq!(miner.blockchain.len(), 2);
        assert_eq!(block.nonce(), 102_300);
    }
}
