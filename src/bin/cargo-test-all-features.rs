use std::env;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    if let Some(arg) = env::args().nth(1) {
        if arg == "--help" {
            println!("See https://crates.io/crates/cargo-all-features");
            return Ok(());
        }
    }
    cargo_all_features::run(cargo_all_features::test_runner::CargoCommand::Test)
}
