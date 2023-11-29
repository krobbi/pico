use std::path::PathBuf;

use clap::{arg, command};

/// Configuration data for Pico.
pub struct Config {
    /// Whether to optimize the source PNG file.
    pub optimize: bool,

    /// The path to the source PNG file.
    pub source_path: PathBuf,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(-o --opt "Optimize the source PNG file"))
            .arg(arg!(<source> "The source PNG file"))
            .get_matches();

        Config {
            optimize: args.get_flag("opt"),
            source_path: args.get_one::<String>("source").unwrap().into(),
        }
    }
}
