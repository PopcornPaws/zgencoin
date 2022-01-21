use crate::{GossipResult, Node};
use std::net::{SocketAddrV4, TcpListener};

pub struct Miner {
    peers: Vec<SocketAddrV4>,
    listener: TcpListener,
    counter: u32,
}

impl Node for Miner {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> GossipResult {
        let peer = *self
            .peer
            .iter()
            .choose(rng)
            .expect("no peers to connect to");

        thread::sleep::
        Ok(MessageToPeer {
            msg: GossipMessage::Ping,
            peer,
        })
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
    }
}

pub struct ThinNode {
    peers: Vec<SocketAddrV4>,
    listener: TcpListener,
    tx_id: u32,
}

impl Node for ThinNode {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> GossipResult {
        todo!();
    }

    fn listen(&mut self) -> GossipResult {
        todo!();
    }
}
