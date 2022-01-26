use crate::tx_pool::TxPool;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Miner {
    tx_pool: Arc<Mutex<TxPool>>,
    thread_pool: ThreadPool,
}
