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
            if let Ok(n) = s.read(&mut buffer) {
                let msg = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                println!("\x1b[33m[!] Handshake Received: {}\x1b[0m", msg);
                io::stdout().flush().unwrap();

                // --- BYZANTINE LOGIC ---
                let response = if identity.id == 3 {
                    println!("\x1b[31m[Byzantine] Node 3 sending MALICIOUS signature...\x1b[0m");
                    format!("ACK_SIG:MALICIOUS_TRASH_DATA_777")
                } else {
                    let signature = identity.sign_vote("VOTE_ACK", msg.as_bytes());
                    format!("ACK_SIG:{}", hex::encode(signature))
                };

                let _ = s.write_all(response.as_bytes());
                let _ = s.flush();

                // --- GOSSIP RING LOGIC ---
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
            let response = String::from_utf8_lossy(&buffer[..n]);
            return Some(response.to_string());
        }
    }
    None
}

