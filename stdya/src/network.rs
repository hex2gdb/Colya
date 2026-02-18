use crate::aggregator::QuorumCertificate;
use crate::crypto::NodeIdentity;
use ed25519_dalek::{Signer, Verifier};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

/// Starts the BFT network listener to collect signatures and forward gossip
pub fn start_listener(
    port: &str,
    identity: &NodeIdentity,
    qc: Arc<Mutex<QuorumCertificate>>,
) -> io::Result<()> {
    // Note: Use 127.0.0.1 for local binding
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("\x1b[32m[+] S-BFT Listener online on port {}\x1b[0m", port);
    io::stdout().flush().unwrap();

    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            let mut buffer = [0u8; 100]; // 4 (ID) + 32 (PubKey) + 64 (Sig)

            if let Ok(n) = s.read(&mut buffer) {
                if n >= 100 {
                    // 1. Extract components
                    let node_id = i32::from_be_bytes(buffer[0..4].try_into().unwrap());
                    let mut pub_key = [0u8; 32];
                    let mut sig = [0u8; 64];
                    pub_key.copy_from_slice(&buffer[4..36]);
                    sig.copy_from_slice(&buffer[36..100]);

                    // 2. Lock the QC and verify/add the signature
                    let mut lock = qc.lock().unwrap();
                    if lock.verify_and_add(node_id, pub_key, sig) {
                        println!(
                            "[Consensus] 2f+1 Quorum reached for block: {}!",
                            lock.block_hash
                        );
                    }
                }

                // 3. Handle Byzantine Response Logic
                let msg = String::from_utf8_lossy(&buffer[..n.min(100)])
                    .trim()
                    .to_string();
                let response = if identity.id == 3 {
                    println!("\x1b[31m[Byzantine] Node 3 sending MALICIOUS signature...\x1b[0m");
                    "ACK_SIG:MALICIOUS_TRASH_DATA_777".to_string()
                } else {
                    // Sign a vote acknowledgement
                    let signature = identity.key.sign(msg.as_bytes());
                    format!("ACK_SIG:{}", hex::encode(signature.to_bytes()))
                };

                let _ = s.write_all(response.as_bytes());
                let _ = s.flush();

                // 4. GOSSIP RING LOGIC
                if msg.contains("HELLO") && !msg.contains("GOSSIP") {
                    let my_id = identity.id;
                    let next_id = (my_id % 4) + 1;
                    let next_port = 5000 + next_id;
                    let forward_msg = format!("GOSSIP_FROM_{}:{}", my_id, msg);

                    std::thread::spawn(move || {
                        send_handshake(&next_port.to_string(), &forward_msg);
                    });
                }
            }
        }
    }
    Ok(())
}

pub fn send_handshake(target_port: &str, msg_content: &str) -> Option<String> {
    let address = format!("127.0.0.1:{}", target_port);
    if let Ok(mut stream) = TcpStream::connect(address) {
        let _ = stream.write_all(msg_content.as_bytes());
        let _ = stream.flush();

        let mut buffer = [0; 512];
        if let Ok(n) = stream.read(&mut buffer) {
            return Some(String::from_utf8_lossy(&buffer[..n]).to_string());
        }
    }
    None
}
