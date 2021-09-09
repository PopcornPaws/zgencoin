mod miner;
mod node;
mod tx_pool;

pub use miner::Miner;
pub use node::Node;

use serde::{Deserialize, Serialize};
use zgc_blockchain::{Block, TxData};

use std::io::Write;
use std::net::{SocketAddrV4, TcpStream};

#[derive(Serialize, Deserialize)]
pub enum GossipMessage {
    Transaction(TxData),
    Block(Block),
    BlockRequest(usize),
}

pub struct MessageToPeer {
    pub msg: GossipMessage,
    pub peer: SocketAddrV4,
}

pub fn send_message(msg_to_peer: MessageToPeer) -> Result<(), String> {
    let mut peer = TcpStream::connect(msg_to_peer.peer)
        .map_err(|e| format!("failed to establish tcp stream: {}", e))?;

    // unwrap is fine here because gossip msg can always be serialized into a vec
    peer.write_all(&serde_json::to_vec(&msg_to_peer.msg).unwrap())
        .map_err(|e| format!("failed to send block data: {}", e))?;

    Ok(())
}
