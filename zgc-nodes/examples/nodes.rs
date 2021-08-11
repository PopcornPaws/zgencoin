use zgc_crypto::Sha256;
use zgc_nodes::{Miner, Node};

#[tokio::main]
async fn main() -> Result<(), String> {
    let hasher = Sha256::new();

    let ip_0 = "127.0.0.1:7650";
    let ip_1 = "127.0.0.1:7651";

    let mut miner_0 = Miner::new(ip_0, vec![String::from(ip_1)], hasher).await?;
    let mut miner_1 = Miner::new(ip_1, vec![String::from(ip_0)], hasher).await?;

    miner_0.gossip().await?;
    miner_1.gossip().await?;

    Ok(())
}
