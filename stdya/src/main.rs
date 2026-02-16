use std::env;
use stdya::{GREEN, BOLD, RESET, YELLOW, BLUE};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. ABSOLUTE DEBUG: See exactly what the OS is giving us
    println!("{}  [DEBUG] Received Args: {:?}{}", YELLOW, args, RESET);

    // 2. More flexible parsing
    let id = if let Some(pos) = args.iter().position(|x| x.contains("id")) {
        args[pos + 1].trim().to_string()
    } else {
        "unknown".to_string()
    };

    let port = if let Some(pos) = args.iter().position(|x| x.contains("port")) {
        args[pos + 1].trim().to_string()
    } else {
        "5000".to_string()
    };

    println!("{}[Node {}]{} Initializing S-BFT on port {}...", BOLD, id, RESET, port);

    // Start Listener
    let lp = port.clone();
    std::thread::spawn(move || {
        let _ = stdya::network::start_listener(&lp);
    });

    // 3. Trigger Handshake
    if id == "1" {
        println!("{}  [!] NODE 1 MATCHED! Sending handshake in 3s...{}", GREEN, RESET);
        std::thread::sleep(std::time::Duration::from_secs(3));
        stdya::network::send_handshake("5002", &id);
    }

    loop { std::thread::sleep(std::time::Duration::from_secs(1)); }
}

