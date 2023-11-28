use std::path::PathBuf;

use clap::{arg, command, ArgMatches};

/// Configuration data for Pico.
pub struct Config {
    /// Whether to optimize the source PNG file.
    pub optimize: bool,

    /// The path to the source PNG file to read.
    pub source_path: PathBuf,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(-o --optimize "Optimize the source PNG file"))
            .arg(arg!(<source> "The source PNG file to read"))
            .get_matches();

        Config {
            optimize: args.get_flag("optimize"),
            source_path: get_arg_path(&args, "source"),
        }
    }
}

/// Get a path from parsed arguments by its ID.
fn get_arg_path(args: &ArgMatches, id: &str) -> PathBuf {
    args.get_one::<String>(id).unwrap().into()
}
