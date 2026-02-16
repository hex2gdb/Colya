use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};

// 1. The Listener
pub fn start_listener(port: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    println!("\x1b[32m[+] S-BFT Listener online on port {}, awaiting handshakes...\x1b[0m", port);
    
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            let mut buffer = [0; 512];
            let _ = s.read(&mut buffer);
            let received = String::from_utf8_lossy(&buffer);
            println!("\x1b[33m[!] Handshake Received: {}\x1b[0m", received.trim_matches(char::from(0)));
        }
    }
    Ok(())
}

// 2. The Sender (Fixed Syntax)
pub fn send_handshake(target_port: &str, node_id: &str) {
    let address = format!("127.0.0.1:{}", target_port);
    // Use :: for the static method call
    if let Ok(mut stream) = TcpStream::connect(address) {
        let msg = format!("HELLO_FROM_{}", node_id);
        let _ = stream.write_all(msg.as_bytes());
        println!("\x1b[36m[Node {}] -> Handshake sent to port {}\x1b[0m", node_id, target_port);
    }
}
