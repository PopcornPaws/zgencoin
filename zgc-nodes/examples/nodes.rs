use zgc_common::Address;
use zgc_crypto::Sha256;
use zgc_nodes::send_message;
use zgc_nodes::{Miner, Node, ThinNode};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng as Cc8;

use std::time::SystemTime;

fn main() -> Result<(), String> {
    let ip_0 = "127.0.0.1:33999";
    let ip_1 = "127.0.0.1:33998";
    let ip_2 = "127.0.0.1:33997";
    let ip_3 = "127.0.0.1:33996";

    let hasher = Sha256::new();
    let thin_node_priv_key = "thin@priv_key";
    //let miner0_priv_key = "miner0@priv_key";
    //let miner1_priv_key = "miner1@priv_key";
    let miner2_priv_key = "miner2@priv_key";
    //let mut miner_0 = Miner::new(ip_0, &[ip_1, ip_2, ip_3], hasher, 1, 5, miner0_priv_key)?;
    //let mut miner_1 = Miner::new(ip_1, &[ip_0, ip_2, ip_3], hasher, 1, 5, miner1_priv_key)?;
    let mut miner_2 = Miner::new(ip_2, &[ip_3], hasher, 1, 5, miner2_priv_key)?;
    miner_2.set_status_to_mining();
    let mut thin_node = ThinNode::new(ip_3, &[ip_2], thin_node_priv_key)?;

    let recipient = Address::try_from_str("15b1b4630260bda6d9074fc398bae619aaaf561d").unwrap();
    let created_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros();
    thin_node.new_transaction(10_000, recipient, thin_node_priv_key, created_at)?;

    std::thread::spawn(move || {
        perform_loop(&mut thin_node);
    });

    //std::thread::spawn(move || {
    //    perform_loop(&mut miner_0);
    //});

    //std::thread::spawn(move || {
    //    perform_loop(&mut miner_1);
    //});

    perform_loop(&mut miner_2);

    Ok(())
}

fn perform_loop(node: &mut impl Node) {
    let mut rng = Cc8::from_seed(Default::default());
    for _ in 0..10 {
        match node.gossip(&mut rng) {
            Ok(msg) => if let Err(_e) = send_message(msg) {},
            Err(e) => println!("gossip error: {}", e),
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        match node.listen() {
            Ok(msg) => if let Err(_e) = send_message(msg) {},
            Err(e) => println!("listen error: {}", e),
        }
    }
}
