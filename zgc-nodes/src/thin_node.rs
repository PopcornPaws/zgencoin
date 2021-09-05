pub struct ThinNode {
    peers: Vec<String>,
    listener: TcpListener,
    wallet: Wallet,
    tx_pool: Vec<H256>,
}

impl ThinNode {
    pub fn new(own_ip: &str, peers: Vec<String>, private_key: String) -> Result<Self, String> {
        let listener =
            TcpListener::bind(own_ip).map_err(|e| format!("failed to bind tcp listener: {}", e))?;
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
        private_key: String,
    ) -> Result<TxData, String> {
        let tx_data = self
            .wallet
            .new_transaction(amount, recipient, private_key)?;
        self.tx_pool.push(tx_data.signature());
        Ok(tx_data)
    }
}

impl Node for ThinNode {
    fn gossip(&mut self, rng: &mut dyn rand::RngCore) -> Result<MessageToPeer, String> {
        todo!();
    }

    fn listen(&mut self) -> Result<Option<MessageToPeer>, String> {
        todo!();
    }
}
