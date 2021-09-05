use crate::MessageToPeer;
use zgc_blockchain::Blockchain;

pub enum NodeStatus<'a> {
    Forked(Vec<Blockchain<'a>>),
    Mining,
    Syncing,
}

pub trait Node {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<MessageToPeer, String>;
    fn listen(&mut self) -> Result<Option<MessageToPeer>, String>;
}
