use std::path::PathBuf;

use clap::{arg, command};

/// Configuration data for Pico.
pub struct Config {
    /// Whether to optimize the source PNG file.
    pub optimize: bool,

    /// Whether to overwrite the target ICO file.
    pub force: bool,

    /// The path to the source PNG file.
    pub source_path: PathBuf,

    /// The path to the target ICO file.
    pub target_path: PathBuf,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(-o --opt "Optimize the source PNG file"))
            .arg(arg!(-f --force "Overwrite the target ICO file"))
            .arg(arg!(<source> "The source PNG file"))
            .arg(arg!(<target> "The target ICO file"))
            .get_matches();

        Config {
            optimize: args.get_flag("opt"),
            force: args.get_flag("force"),
            source_path: args.get_one::<String>("source").unwrap().into(),
            target_path: args.get_one::<String>("target").unwrap().into(),
        }
    }
}
