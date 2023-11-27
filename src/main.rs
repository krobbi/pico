mod config;

/// Display the configured source path.
fn main() {
    let config = config::Config::from_args();
    println!("Source path: {}", config.source_path);
}
