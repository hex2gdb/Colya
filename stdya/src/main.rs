use std::env;
use std::io::{self, Write};
use stdya::{GREEN, BOLD, RESET, YELLOW, BLUE};
use stdya::crypto::NodeIdentity;
use ed25519_dalek::SigningKey;
use stdya::RED;




fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. Explicit Parsing
    let id_str = args.iter().position(|x| x.contains("id"))
        .map(|i| args[i + 1].trim().to_string()).unwrap_or("0".to_string());
    let port_str = args.iter().position(|x| x.contains("port"))
        .map(|i| args[i + 1].trim().to_string()).unwrap_or("5000".to_string());
    let id_num: i32 = id_str.parse().unwrap_or(0);

    if args.len() >= 3 {
        // 2. Identity Setup
        let seed_str = format!("node_seed_000000000000000000000_{}", id_str);
        let mut seed_fixed = [0u8; 32];
        let bytes = seed_str.as_bytes();
        seed_fixed[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
        
        let node_identity = NodeIdentity { 
            key: SigningKey::from_bytes(&seed_fixed),
            id: id_num,
        };

        // 3. Forced Visibility Log
        println!("{}[Node {}]{} Starting on port {}...", BOLD, id_str, RESET, port_str);
        io::stdout().flush().unwrap();

        // 4. Background Listener with Error Catching
        let lp = port_str.clone();
        std::thread::spawn(move || {
            if let Err(e) = stdya::network::start_listener(&lp, &node_identity) {
                eprintln!("{}[!] CRITICAL ERROR on port {}: {}{}", RED, lp, e, RESET);
            }
        });

        // 5. Handshake Trigger (Node 1)
        if id_str.trim() == "1" {
            println!("{}[Node 1]{} {}[!] Timer Activated. 7s to Handshake...{}", BOLD, RESET, YELLOW, RESET);
            io::stdout().flush().unwrap();

            std::thread::sleep(std::time::Duration::from_secs(7));

            println!("{}[*] Node 1 firing handshake to Node 2...{}", BLUE, RESET);
            stdya::network::send_handshake("5002", &id_str);
            io::stdout().flush().unwrap();
        }

        loop { std::thread::sleep(std::time::Duration::from_secs(10)); }
    }
}

