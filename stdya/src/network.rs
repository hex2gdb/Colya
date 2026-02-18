use crate::aggregator::QuorumCertificate;
use crate::crypto::NodeIdentity;
use ed25519_dalek::Signer;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn start_listener(
    port: String,
    identity: NodeIdentity, // Pass by value for async move
    qc: Arc<Mutex<QuorumCertificate>>,
) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!(
        "\x1b[32m[+] Async BFT Listener online on port {}\x1b[0m",
        port
    );

    loop {
        let (mut socket, _) = listener.accept().await?;
        let qc_clone = Arc::clone(&qc);
        let identity_clone = identity.clone();

        tokio::spawn(async move {
            let mut buffer = [0u8; 100];

            if let Ok(n) = socket.read(&mut buffer).await {
                if n >= 100 {
                    // 1. Extract and Verify
                    let node_id = i32::from_be_bytes(buffer[0..4].try_into().unwrap());
                    let mut pub_key = [0u8; 32];
                    let mut sig = [0u8; 64];
                    pub_key.copy_from_slice(&buffer[4..36]);
                    sig.copy_from_slice(&buffer[36..100]);

                    let mut lock = qc_clone.lock().unwrap();
                    if lock.verify_and_add(node_id, pub_key, sig) {
                        println!(
                            "[Consensus] 2f+1 Quorum reached for block: {}!",
                            lock.block_hash
                        );
                    }
                }

                // 2. Byzantine Logic
                let msg = String::from_utf8_lossy(&buffer[..n.min(100)])
                    .trim()
                    .to_string();
                let response = if identity_clone.id == 3 {
                    "ACK_SIG:MALICIOUS_TRASH_DATA_777".to_string()
                } else {
                    let signature = identity_clone.key.sign(msg.as_bytes());
                    format!("ACK_SIG:{}", hex::encode(signature.to_bytes()))
                };

                let _ = socket.write_all(response.as_bytes()).await;

                // 3. Gossip Ring Logic (Async)
                if msg.contains("HELLO") && !msg.contains("GOSSIP") {
                    let next_port = 5000 + (identity_clone.id % 4) + 1;
                    let forward_msg = format!("GOSSIP_FROM_{}:{}", identity_clone.id, msg);

                    tokio::spawn(async move {
                        let _ = send_handshake(&next_port.to_string(), &forward_msg).await;
                    });
                }
            }
        });
    }
}

pub async fn send_handshake(target_port: &str, msg_content: &str) -> Option<String> {
    let address = format!("127.0.0.1:{}", target_port);
    if let Ok(mut stream) = TcpStream::connect(address).await {
        let _ = stream.write_all(msg_content.as_bytes()).await;

        let mut buffer = [0; 512];
        if let Ok(n) = stream.read(&mut buffer).await {
            return Some(String::from_utf8_lossy(&buffer[..n]).to_string());
        }
    }
    None
}
