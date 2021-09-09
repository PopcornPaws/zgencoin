use crate::GossipResult;
use zgc_blockchain::Blockchain;

pub enum NodeStatus<'a> {
    Forked(Vec<Blockchain<'a>>),
    Mining,
    Syncing,
}

pub trait Node {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> GossipResult;
    fn listen(&mut self) -> GossipResult;
}
