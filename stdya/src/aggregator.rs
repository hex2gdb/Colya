use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuorumCertificate {
    pub block_hash: String,
    pub signers: HashSet<i32>, // Stores unique node IDs to prevent double-voting
}

impl QuorumCertificate {
    pub fn new(hash: &str) -> Self {
        Self {
            block_hash: hash.to_string(),
            signers: HashSet::new(),
        }
    }

    /// Adds a signature and returns true if 2f+1 (3 nodes) is reached.
    pub fn add_signature(&mut self, node_id: i32) -> bool {
        self.signers.insert(node_id);
        
        // Threshold Math: 2f + 1
        // For 4 nodes, f = 1, so quorum = 3
        let quorum_threshold = 3;
        
        self.signers.len() >= quorum_threshold
    }
}

