pub mod aggregator;
pub mod crypto;
pub mod math;
pub mod network;
pub mod traits;

pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m";

pub fn version() -> &'static str {
    "1.0.0-mission-critical"
}
