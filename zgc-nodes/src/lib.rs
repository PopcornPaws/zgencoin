mod miner;
mod node;
mod thin_node;
mod tx_pool;

pub use miner::Miner;
pub use node::Node;
pub use thin_node::ThinNode;

use serde::{Deserialize, Serialize};
use zgc_blockchain::{Block, TxData};

use std::io::Write;
use std::net::{SocketAddrV4, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
pub enum GossipMessage {
    Transaction(TxData),
    Block(Block),
    BlockRequest(usize),
    Ping,
}

#[derive(Debug)]
pub struct MessageToPeer {
    pub msg: GossipMessage,
    pub peer: SocketAddrV4,
}

pub type GossipResult = Result<MessageToPeer, String>;

pub fn send_message(msg_to_peer: MessageToPeer) -> Result<(), String> {
    let mut peer = TcpStream::connect(msg_to_peer.peer)
        .map_err(|e| format!("failed to establish tcp stream: {}", e))?;

    // unwrap is fine here because gossip msg can always be serialized into a vec
    peer.write_all(&serde_json::to_vec(&msg_to_peer.msg).unwrap())
        .map_err(|e| format!("failed to send block data: {}", e))?;

    Ok(())
}
