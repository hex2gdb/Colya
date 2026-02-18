use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuorumCertificate {
    pub block_hash: String,
    pub signers: HashSet<i32>, // Stores unique node IDs to prevent double-voting
}

impl QuorumCertificate {
    /// Creates a new Quorum Certificate for a specific block hash
    pub fn new(hash: &str) -> Self {
        Self {
            block_hash: hash.to_string(),
            signers: HashSet::new(),
        }
    }

    /// Verifies a signature against a public key before adding to the quorum.
    pub fn verify_and_add(
        &mut self,
        node_id: i32,
        pub_key_bytes: [u8; 32],
        signature_bytes: [u8; 64],
    ) -> bool {
        // 1. Reconstruct the public key and signature objects
        let public_key = match VerifyingKey::from_bytes(&pub_key_bytes) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let signature = Signature::from_bytes(&signature_bytes);

        // 2. Verify the signature against the block hash stored in this QC
        if public_key
            .verify(self.block_hash.as_bytes(), &signature)
            .is_ok()
        {
            self.signers.insert(node_id);
            println!("[Security] Signature verified for Node {}", node_id);
        } else {
            println!(
                "[Security] INVALID signature detected from Node {}",
                node_id
            );
        }

        // Return true if quorum (3) is reached
        self.signers.len() >= 3
    }

    /// Adds a signature ID and returns true if 2f+1 (3 nodes) is reached.
    pub fn add_signature(&mut self, node_id: i32) -> bool {
        self.signers.insert(node_id);

        // Threshold Math: 2f + 1 (For 4 nodes, f = 1, so quorum = 3)
        let quorum_threshold = 3;
        self.signers.len() >= quorum_threshold
    }
}
