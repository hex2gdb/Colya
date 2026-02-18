use ed25519_dalek::SigningKey;
use std::env;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use stdya::aggregator::QuorumCertificate;
use stdya::crypto::NodeIdentity;
use stdya::state::{PersistentLedger, Transaction}; // Updated to PersistentLedger
use stdya::{BLUE, BOLD, GREEN, RED, RESET, YELLOW};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. Parse arguments
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

        // 3. Initialize Persistent Shared State
        let qc = Arc::new(Mutex::new(QuorumCertificate::new("BLOCK_001")));
        // Every node gets its own local RocksDB folder based on its ID
        let db_path = format!("./ledger_db_{}", id_str);
        let ledger = Arc::new(Mutex::new(PersistentLedger::new(&db_path)));

        println!(
            "{}[Node {}]{} Starting on port {}...",
            BOLD, id_str, RESET, port_str
        );
        io::stdout().flush().unwrap();

        // 4. Spawn Async Listener
        let listener_qc = Arc::clone(&qc);
        let listener_port = port_str.clone();
        let listener_identity = node_identity.clone();
        tokio::spawn(async move {
            let _ = stdya::network::start_listener(&listener_port, &listener_identity, listener_qc)
                .await;
        });

        // 5. CONSENSUS LOGIC (Leader only)
        if id_str.trim() == "1" {
            tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;

            for peer_port in ["5002", "5003", "5004"] {
                // Check if Quorum was reached via Background Listener or Previous Request
                let quorum_reached = {
                    let lock = qc.lock().unwrap();
                    lock.signers.len() >= 3
                };

                if quorum_reached {
                    let mut p_ledger = ledger.lock().unwrap();
                    let demo_tx = Transaction {
                        sender: "Node_1".to_string(),
                        receiver: "Node_2".to_string(),
                        amount: 50,
                    };

                    // Apply to memory AND persist to RocksDB
                    p_ledger.apply_and_persist(vec![demo_tx]);

                    println!(
                        "{}[Ledger]{} Quorum Reached! Height: {}, Node_2 Balance: {}",
                        BLUE,
                        RESET,
                        p_ledger.state.block_height,
                        p_ledger.state.balances.get("Node_2").unwrap_or(&0)
                    );
                    break;
                }

                println!(
                    "{}[*] Requesting signature from {}...{}",
                    BLUE, peer_port, RESET
                );
                let _ = stdya::network::send_handshake(peer_port, "PROPOSE_BLOCK").await;
            }
        }

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
}
