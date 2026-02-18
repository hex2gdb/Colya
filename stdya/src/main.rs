use ed25519_dalek::SigningKey;
use std::env;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use stdya::aggregator::QuorumCertificate;
use stdya::crypto::NodeIdentity;
use stdya::{BLUE, BOLD, GREEN, RED, RESET, YELLOW};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. Parse arguments first so we have the ID and Port available
    let id_str = args
        .iter()
        .position(|x| x.contains("id"))
        .map(|i| args[i + 1].trim().to_string())
        .unwrap_or("0".to_string());
    let port_str = args
        .iter()
        .position(|x| x.contains("port"))
        .map(|i| args[i + 1].trim().to_string())
        .unwrap_or("5000".to_string());
    let id_num: i32 = id_str.parse().unwrap_or(0);

    if args.len() >= 3 {
        // 2. Setup Identity
        let seed_str = format!("node_seed_000000000000000000000_{}", id_str);
        let mut seed_fixed = [0u8; 32];
        let bytes = seed_str.as_bytes();
        seed_fixed[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);

        let node_identity = NodeIdentity {
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num,
        };

        println!(
            "{}[Node {}]{} Identity initialized. Public Key: {:?}",
            BOLD,
            id_str,
            RESET,
            node_identity.key.verifying_key()
        );
        println!(
            "{}[Node {}]{} Starting on port {}...",
            BOLD, id_str, RESET, port_str
        );
        io::stdout().flush().unwrap();

        // 3. Setup Shared Quorum Certificate
        let qc = Arc::new(Mutex::new(QuorumCertificate::new("BLOCK_001")));
        let qc_for_thread = Arc::clone(&qc);

        // 4. Start Network Listener
        let lp = port_str.clone();
        let listener_identity = NodeIdentity {
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num,
        };

        std::thread::spawn(move || {
            let _ = stdya::network::start_listener(&lp, &listener_identity, qc_for_thread);
        });

        // 5. CONSENSUS & BYZANTINE DETECTION (Node 1 only)
		        // 5. CONSENSUS & BYZANTINE DETECTION (Node 1 only)
        if id_str.trim() == "1" {
            println!(
                "{}  [!] Node 1: Leader Active. Collecting signatures...{}",
                GREEN, RESET
            );
            io::stdout().flush().unwrap();
            
            // Initial wait to let network stabilize
            std::thread::sleep(std::time::Duration::from_secs(7));

            for peer_port in ["5002", "5003", "5004"] {
                // --- 1. PRE-REQUEST QUORUM CHECK ---
                // Check if background listener already reached quorum
                {
                    let lock = qc.lock().unwrap();
                    if lock.signers.len() >= 3 {
                        println!("{}  [Quorum] Threshold already met ({} nodes).{}", GREEN, lock.signers.len(), RESET);
                        // Save and exit loop
                        let file = std::fs::File::create("genesis.json").expect("Unable to create file");
                        serde_json::to_writer_pretty(file, &*lock).expect("Serialization failed");
                        println!("{}[Explorer] Genesis Block saved to genesis.json!{}", YELLOW, RESET);
                        break;
                    }
                }

                println!("{}[*] Requesting signature from {}...{}", BLUE, peer_port, RESET);
                io::stdout().flush().unwrap();

                if let Some(_sig_msg) = stdya::network::send_handshake(peer_port, "PROPOSE_BLOCK") {
                    let mut lock = qc.lock().unwrap();
                    let peer_id: i32 = peer_port.parse().unwrap_or(0);

                    if lock.add_signature(peer_id) {
                        println!("{}\n[!!!] 2f+1 QUORUM REACHED: Block 001 Finalized!{}", GREEN, RESET);

                        let file = std::fs::File::create("genesis.json").expect("Unable to create file");
                        serde_json::to_writer_pretty(file, &*lock).expect("Serialization failed");

                        println!("{}[Explorer] Genesis Block saved to genesis.json!{}\n", YELLOW, RESET);
                        io::stdout().flush().unwrap();
                        break;
                    }
                }
            }
        }

        // Keep node alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    } // Closes if args.len() >= 3
} // Closes main

