use std::env;
use std::io::{self, Write};
use stdya::{GREEN, BOLD, RESET, YELLOW, BLUE};
use stdya::crypto::NodeIdentity;
use ed25519_dalek::SigningKey;


fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. Mission-Critical Argument Parsing
    let id_str = args.iter().position(|x| x.contains("id"))
        .map(|i| args[i + 1].trim().to_string()).unwrap_or("0".to_string());
    
    let port_str = args.iter().position(|x| x.contains("port"))
        .map(|i| args[i + 1].trim().to_string()).unwrap_or("5000".to_string());

    let id_num: i32 = id_str.parse().unwrap_or(0);

    if args.len() >= 3 {
        // 2. Identity Generation (Ed25519)
        let seed_str = format!("node_seed_000000000000000000000_{}", id_str);
        let mut seed_fixed = [0u8; 32];
        let bytes = seed_str.as_bytes();
        seed_fixed[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
        
        let identity = NodeIdentity { 
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num, 
        };

        println!("{}[Node {}]{} Starting on port {}...", BOLD, id_str, RESET, port_str);
        io::stdout().flush().unwrap();

        // 3. Start Background Listener
        let lp = port_str.clone();
        let node_identity = NodeIdentity { 
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num,
        };
        
        std::thread::spawn(move || {
            let _ = stdya::network::start_listener(&lp, &node_identity);
        });

        // 4. THE HANDSHAKE TRIGGER (Strictly Node 1)
        if id_str.trim() == "1" {
            println!("{}  [!] Node 1: Identity Verified. Sending HELLO in 7s...{}", GREEN, RESET);
            io::stdout().flush().unwrap();

            // Wait for network to stabilize
            std::thread::sleep(std::time::Duration::from_secs(7));

            // CRITICAL: Send "HELLO" prefix to trigger gossip in network.rs
            let handshake_payload = format!("HELLO_FROM_{}", id_str);
            println!("{}[*] Node 1 firing {} to Node 2...{}", BLUE, handshake_payload, RESET);
            io::stdout().flush().unwrap();
            
            stdya::network::send_handshake("5002", &handshake_payload);
        }

        // Keep process alive indefinitely
        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}

