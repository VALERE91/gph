use std::process::exit;

// This declares the `cli` module, which contains all our CLI-specific code.
mod cli;

fn main() {
    // Delegate all logic to the `run` function in our `cli` module.
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
        exit(1);
    }

    println!("Done.");
}