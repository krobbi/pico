mod config;

use config::Config;

/// Display the configured source path.
fn main() {
    let config = Config::from_args();
    println!("Source file: {}", config.source_path.display());

    if config.source_path.is_file() {
        println!("Source file exists!");
    } else {
        println!("Source file does not exist!");
    }
}
