use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use crate::crypto::NodeIdentity;

pub fn start_listener(port: &str, identity: &NodeIdentity) -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    println!("\x1b[32m[+] S-BFT Listener online on port {}\x1b[0m", port);
    io::stdout().flush().unwrap();

    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            let mut buffer = [0; 512];
            let n = s.read(&mut buffer)?;
            let msg = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

            println!("\x1b[33m[!] Handshake Received: {}\x1b[0m", msg);
            io::stdout().flush().unwrap();

            // --- GOSSIP LOGIC ---
            // Prevent infinite loops: only gossip if it's a fresh HELLO
            if msg.contains("HELLO") && !msg.contains("GOSSIP_BY") {
                let next_id = (identity.id % 4) + 1;
                let next_port = 5000 + next_id;
                let forward_msg = format!("GOSSIP_BY_{}:{}", identity.id, msg);

                std::thread::spawn(move || {
                    println!("\x1b[35m[Gossip] Node forwarding to port {}...\x1b[0m", next_port);
                    send_handshake(&next_port.to_string(), &forward_msg);
                });
            }

            // Sign and respond
            let signature = identity.sign_vote("VOTE_ACK", msg.as_bytes());
            let response = format!("ACK_SIG:{}", hex::encode(signature));
            let _ = s.write_all(response.as_bytes());
            let _ = s.flush();
        }
    }
    Ok(())
}

pub fn send_handshake(target_port: &str, msg_content: &str) {
    let address = format!("127.0.0.1:{}", target_port);
    if let Ok(mut stream) = TcpStream::connect(address) {
        let _ = stream.write_all(msg_content.as_bytes());
        let _ = stream.flush();
        // Force flush to ensure visibility in simulator
        io::stdout().flush().unwrap();
    }
}

