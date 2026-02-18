use rocksdb::{DB, Options};

pub struct Storage {
    db: DB,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        Self { db: DB::open(&opts, path).unwrap() }
    }

    pub fn persist_block(&self, height: u64, block_data: &[u8]) {
        self.db.put(height.to_be_bytes(), block_data).expect("WAL failure");
    }
}

