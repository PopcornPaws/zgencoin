use std::collections::HashMap;
use std::time::Instant;

pub type BlockFinder = HashMap<String, &'static Block>;

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
    created_at: Instant,
    height: usize,
    previous_hash: &'static str,
    data: TxData,
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
    }
}
