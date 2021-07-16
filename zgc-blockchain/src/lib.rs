use std::time::Instant;
use std::collections::HashMap;

type BlockFinder = HashMap<&'static str, &'static Block>;
type Address = [u8; 20];

pub struct Blockchain {
    block_finder: BlockFinder,
    difficulty: u8,
}

struct Block {
    created_at: Instant,
    height: usize,
    hash: String,
    previous_hash: &'static str,
    data: TxData,
    nonce: u32,
}

struct TxData {
    from_address: Address,
    to_address: Address,
    amount: usize,
}
