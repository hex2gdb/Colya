use rocksdb::{DB, Options};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    receiver: String,
    amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LedgerState {
    pub balances: HashMap<String, u128>,
    pub block_height: u64,
}

pub struct PersistentLedger {
    pub state: LedgerState,
    db: DB, // RocksDB instance for persistence
}

impl PersistentLedger {
    pub fn new(db_path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, db_path).expect("Failed to open RocksDB");

        // Attempt recovery: Check if "LATEST_STATE" exists in DB
        let state = match db.get(b"LATEST_STATE").unwrap() {
            Some(data) => serde_json::from_slice(&data).expect("Recovery failed"),
            None => LedgerState::init_default(),
        };

        Self { state, db }
    }

    pub fn apply_and_persist(&mut self, txs: Vec<Transaction>) {
        self.state.apply_block(txs);

        // Write-Ahead Log (WAL) equivalent: Persist state to disk
        let serialized = serde_json::to_vec(&self.state).unwrap();
        self.db
            .put(b"LATEST_STATE", serialized)
            .expect("Disk write failed");

        // Also log individual blocks for auditing
        self.db
            .put(self.state.block_height.to_be_bytes(), serialized)
            .ok();
    }
}

impl LedgerState {
    fn init_default() -> Self {
        let mut balances = HashMap::new();
        balances.insert("Node_1".to_string(), 1_000_000);
        Self {
            balances,
            block_height: 0,
        }
    }

    fn apply_block(&mut self, transactions: Vec<Transaction>) {
        for tx in transactions {
            let sender_bal = self.balances.entry(tx.sender.clone()).or_insert(0);
            if *sender_bal >= tx.amount {
                *sender_bal -= tx.amount;
                *self.balances.entry(tx.receiver).or_insert(0) += tx.amount;
            }
        }
        self.block_height += 1;
    }
}
