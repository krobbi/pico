mod config;

use std::process;

use config::Config;

/// Display the configured source path.
fn main() {
    let config = Config::from_args();
    println!("Source file: {}", config.source_path.display());

    if !config.source_path.is_file() {
        bail("Source file does not exist!");
    }
}

/// Exit with an error message.
fn bail(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}
