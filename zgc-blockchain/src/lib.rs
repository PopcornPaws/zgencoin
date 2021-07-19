use std::collections::HashMap;
use std::time::Instant;

pub struct BlockFinder {
    height2hash: HashMap<usize, &'static str>,
    hash2block: HashMap<&'static str, &'static Block>,
}

impl BlockFinder {
    pub fn new() -> Self {
        Self {
            height2hash: HashMap::new(),
            hash2block: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: &'static Block) {
        // TODO use hashing function here
        let hash = Box::leak(Box::new(String::from("Some sha256 hash")));
        self.height2hash.insert(block.height, hash);
        self.hash2block.insert(hash, block);
    }

    pub fn find_hash(self, hash: &str) -> Option<&'static Block> {
        self.hash2block.get(hash).map(|block| *block)
    }

    pub fn find_height(self, height: usize) -> Option<&'static Block> {
        if let Some(hash) = self.height2hash.get(&height) {
            self.hash2block.get(*hash).map(|block| *block)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Address([u8; 20]);

impl Address {
    pub fn from_str(string: &str) -> Result<Self, String> {
        let mut address = [0_u8; 20];

        string
            .trim_start_matches("0x")
            .as_bytes()
            .chunks(2)
            .enumerate()
            .for_each(|(i, bytes)| {
                let parsed = u8::from_str_radix(std::str::from_utf8(bytes).unwrap(), 16)
                    .expect("input contains invalid data");
                address[i] = parsed;
            });

        Ok(Address(address))
    }

    pub fn to_string(&self) -> String {
        self.0.iter().map(|byte| format!("{:02x}", byte)).collect()
    }
}

pub struct Blockchain {
    block_finder: BlockFinder,
    difficulty: u8,
}

pub struct Block {
    height: usize,
    header: BlockHeader,
    data: TxData,
}

pub struct BlockHeader {
    created_at: Instant,
    previous_hash: &'static str,
    difficulty: u8,
    nonce: u32,
}

#[derive(Clone, Copy)]
pub struct TxData {
    sender: Address,
    recipient: Address,
    amount: usize,
}

impl TxData {
    pub fn new(sender: Address, recipient: Address, amount: usize) -> Self {
        Self {
            sender,
            recipient,
            amount,
        }
    }

    pub fn sender(&self) -> &Address {
        &self.sender
    }

    pub fn recipient(&self) -> &Address {
        &self.recipient
    }

    pub fn amount(&self) -> usize {
        self.amount
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn address() {
        let address_str = "0123456789abcdeffedcba9876543210aabbccdd";
        let address = Address::from_str(address_str).expect("failed to parse address string");
        assert_eq!(address.to_string(), address_str);

        let address_str_0x = String::from("0x") + address_str;
        let address = Address::from_str(&address_str_0x).expect("failed to parse address string");
        assert_eq!(address.to_string(), address_str);
    }
}
