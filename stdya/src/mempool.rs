use std::collections::{VecDeque, HashSet};
use crate::state::Transaction;

pub struct Mempool {
    pending: VecDeque<Transaction>,
    seen_nonces: HashSet<u64>, // Prevents re-submitting the same TX
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Self {
            pending: VecDeque::new(),
            seen_nonces: HashSet::new(),
            max_size,
        }
    }

    /// Adds a transaction to the pool if valid and not a duplicate
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if self.pending.len() >= self.max_size || self.seen_nonces.contains(&tx.nonce) {
            return false;
        }
        self.seen_nonces.insert(tx.nonce);
        self.pending.push_back(tx);
        true
    }

    /// Pulls a batch of transactions for the next block
    pub fn get_batch(&mut self, batch_size: usize) -> Vec<Transaction> {
        let mut batch = Vec::new();
        for _ in 0..batch_size {
            if let Some(tx) = self.pending.pop_front() {
                batch.push(tx);
            }
        }
        batch
    }
}

