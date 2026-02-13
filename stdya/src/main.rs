use stdya; // Imports your library logic

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--version" {
        println!("Colya Standard Library (stdya) v{}", stdya::version());
    } else {
        println!("Colya CMT: Use --version or compile a .ya file.");
    }
}

