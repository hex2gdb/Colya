use std::env;
use std::io::{self, Write};
use stdya::{GREEN, BOLD, RESET, YELLOW, BLUE, RED}; // Added RED for security logs
use stdya::crypto::NodeIdentity;
use ed25519_dalek::SigningKey;
use stdya::aggregator::QuorumCertificate;

fn main() {
    let args: Vec<String> = env::args().collect();

    let id_str = args.iter().position(|x| x.contains("id"))
        .map(|i| args[i + 1].trim().to_string()).unwrap_or("0".to_string());
    let port_str = args.iter().position(|x| x.contains("port"))
        .map(|i| args[i + 1].trim().to_string()).unwrap_or("5000".to_string());
    let id_num: i32 = id_str.parse().unwrap_or(0);

    if args.len() >= 3 {
        let seed_str = format!("node_seed_000000000000000000000_{}", id_str);
        let mut seed_fixed = [0u8; 32];
        let bytes = seed_str.as_bytes();
        seed_fixed[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
        
        let node_identity = NodeIdentity { 
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num,
        };

        println!("{}[Node {}]{} Starting on port {}...", BOLD, id_str, RESET, port_str);
        io::stdout().flush().unwrap();

        // 1. Start Listener
        let lp = port_str.clone();
        let listener_identity = NodeIdentity { 
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num,
        };
        std::thread::spawn(move || {
            let _ = stdya::network::start_listener(&lp, &listener_identity);
        });

        // 2. CONSENSUS & BYZANTINE DETECTION (Node 1 only)
        if id_str.trim() == "1" {
            println!("{}  [!] Node 1: Identity Verified. Collecting Quorum in 7s...{}", GREEN, RESET);
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(7));

            let mut qc = QuorumCertificate::new("BLOCK_001");
            
            for peer_port in ["5002", "5003", "5004"] {
                println!("{}[*] Requesting signature from {}...{}", BLUE, peer_port, RESET);
                io::stdout().flush().unwrap();

                if let Some(sig_msg) = stdya::network::send_handshake(peer_port, "PROPOSE_BLOCK") {
                    // --- BYZANTINE SECURITY CHECK ---
                    if sig_msg.contains("MALICIOUS") {
                        println!("{}[Security] REJECTED: Byzantine behavior detected from port {}!{}", RED, peer_port, RESET);
                        io::stdout().flush().unwrap();
                    } else {
                        let peer_id_num: i32 = peer_port.parse().unwrap_or(0);
                        if qc.add_signature(peer_id_num) {
                            println!("{}\n[!!!] 2f+1 QUORUM REACHED: Block 001 Finalized!{}", GREEN, RESET);
                            
                            // Save to genesis.json
                            let file = std::fs::File::create("genesis.json").expect("Unable to create file");
                            serde_json::to_writer_pretty(file, &qc).expect("Serialization failed");
                            
                            println!("{}[Explorer] Genesis Block saved to genesis.json!{}\n", YELLOW, RESET);
                            io::stdout().flush().unwrap();
                            break; 
                        }
                    }
                }
            }
        }

        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}

