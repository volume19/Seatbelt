//! Seatbelt binary entry point

use seatbelt::seatbelt::Seatbelt;
use std::process;

fn main() {
    // Initialize logging (controlled by RUST_LOG environment variable)
    env_logger::init();

    // Create and run Seatbelt
    let app = Seatbelt::from_args();

    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
