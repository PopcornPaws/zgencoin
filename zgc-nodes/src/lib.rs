use zgc_blockchain::{Blockchain, TxData};

use async_std::net::TcpStream;
use std::collections::BTreeMap;

pub struct Node<'a> {
    sockets: Vec<TcpStream>,
    blockchain: Blockchain<'a>,
    tx_pool: BTreeMap<usize, TxData>,
}
