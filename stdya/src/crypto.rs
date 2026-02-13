use ed25519_dalek::{SigningKey, Signer};

pub struct NodeIdentity {
    pub key: SigningKey,
}

impl NodeIdentity {
    pub fn sign_vote(&self, protocol_id: &str, state_hash: &[u8]) -> [u8; 64] {
        let mut msg = protocol_id.as_bytes().to_vec();
        msg.extend_from_slice(state_hash);
        self.key.sign(&msg).to_bytes()
    }
}

