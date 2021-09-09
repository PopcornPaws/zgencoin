use crate::node::Node;
use crate::{GossipMessage, GossipResult, MessageToPeer};

use zgc_blockchain::{TxData, Wallet};
use zgc_common::Address;

use rand::seq::IteratorRandom;
use serde::Deserialize;

use std::net::{SocketAddrV4, TcpListener};

pub struct ThinNode {
    peers: Vec<SocketAddrV4>,
    listener: TcpListener,
    wallet: Wallet,
    tx_pool: Vec<TxData>,
}

impl ThinNode {
    pub fn new(own_ip: &str, ip_pool: &[&str], private_key: &str) -> Result<Self, String> {
        let listener = TcpListener::bind(own_ip)
            .map_err(|e| format!("failed to bind to tcp listener: {}", e))?;

        let peers = ip_pool
            .iter()
            .map(|ip| ip.parse().expect("invalid ip address format"))
            .collect();

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
        private_key: &str,
        timestamp: u128,
    ) -> Result<(), String> {
        let tx_data = self
            .wallet
            .new_transaction(amount, recipient, private_key, timestamp)?;
        self.tx_pool.push(tx_data);
        Ok(())
    }
}

impl Node for ThinNode {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> GossipResult {
        let peer = *self
            .peers
            .iter()
            .choose(rng)
            .expect("no peeers to connect to");

        if self.tx_pool.is_empty() {
            println!("[THIN NODE] no transaction to broadcast");
            Ok(MessageToPeer {
                msg: GossipMessage::Ping,
                peer,
            })
        } else {
            // send latest transaction
            Ok(MessageToPeer {
                msg: GossipMessage::Transaction(self.tx_pool[0]),
                peer,
            })
        }
    }

    fn listen(&mut self) -> GossipResult {
        let (incoming_stream, peer_address) = self
            .listener
            .accept()
            .map_err(|e| format!("failed to accept incoming stream: {}", e))?;

        let peer_address: SocketAddrV4 = peer_address
            .to_string()
            .parse()
            .map_err(|_| "invalid peer address format".to_string())?;

        // if new peer -> add to pool
        // in the blocking case, this is a problem
        //if !self.peers.contains(&peer_address) {
        //    self.peers.push(peer_address);
        //}

        let mut deserializer = serde_json::Deserializer::from_reader(incoming_stream);
        match GossipMessage::deserialize(&mut deserializer) {
            Ok(GossipMessage::Block(incoming_block)) => {
                if let Some(index) = self
                    .tx_pool
                    .iter()
                    .position(|&x| x.signature == incoming_block.data().tx.signature)
                {
                    let tx = self.tx_pool.remove(index);
                    println!(
                        "[THIN_NODE] removed mined tx with signature: {:?}",
                        tx.signature
                    );
                }
                Ok(MessageToPeer {
                    msg: GossipMessage::Ping,
                    peer: peer_address,
                })
            }
            Ok(_) => {
                // nothing received send ping to random peer
                Ok(MessageToPeer {
                    msg: GossipMessage::Ping,
                    peer: peer_address,
                })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
