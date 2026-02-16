use std::env;
use stdya::{GREEN, BLUE, BOLD, RESET, YELLOW};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 5 {
        let port = &args[2];
        let id = &args[4];
        
        println!("{}[Node {}]{} {}Initializing S-BFT on port {}...{}", 
            BOLD, id, RESET, BLUE, port, RESET);
        
        println!("{}[*] S-BFT Core:{} {}ONLINE{}", 
            YELLOW, RESET, GREEN, RESET);

        // Call the listener
        let _ = stdya::network::start_listener(port);
    }
}

