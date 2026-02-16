use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::Write;

// Ensure 'pub' is here so main.rs can see it!
pub fn start_listener(port: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    // Adding the color constants we just defined for consistency
    println!("\x1b[32m[+] S-BFT Listener online, awaiting handshakes...\x1b[0m");
    
    for stream in listener.incoming() {
        let _stream = stream?;
        println!("\x1b[33m[!] Handshake Received!\x1b[0m");
    }
    Ok(())
}

