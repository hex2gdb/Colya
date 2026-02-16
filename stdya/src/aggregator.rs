use std::collections::HashSet;
use crate::crypto::NodeIdentity;

pub struct QuorumCertificate {
    pub msg_hash: String,
    pub signers: HashSet<i32>,
}

impl QuorumCertificate {
    pub fn new(hash: &str) -> Self {
        Self {
            msg_hash: hash.to_string(),
            signers: HashSet::new(),
        }
    }

    pub fn add_signature(&mut self, node_id: i32) -> bool {
        self.signers.insert(node_id);
        // Quorum reached if 3 or more nodes have signed (2f + 1)
        self.signers.len() >= 3
    }
}

