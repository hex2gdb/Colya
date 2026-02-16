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

            // --- GOSSIP RING LOGIC ---
            // If it's a new HELLO, infect the next node
            if msg.contains("HELLO") && !msg.contains("GOSSIP") {
                let my_id = identity.id;
                let next_id = (my_id % 4) + 1; // 1->2, 2->3, 3->4, 4->1
                let next_port = 5000 + next_id;
                let forward_msg = format!("GOSSIP_FROM_{}:{}", my_id, msg);

                std::thread::spawn(move || {
                    println!("\x1b[35m[Gossip] Node {} Infecting Node {} on port {}...\x1b[0m", my_id, next_id, next_port);
                    io::stdout().flush().unwrap();
                    send_handshake(&next_port.to_string(), &forward_msg);
                });
            }

            // Sign and respond back to sender
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
        io::stdout().flush().unwrap();
    }
}

