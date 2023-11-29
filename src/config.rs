use std::path::PathBuf;

use clap::{arg, command};

/// Configuration data for Pico.
pub struct Config {
    /// The paths to the PNG input files.
    pub input_paths: Vec<PathBuf>,

    /// The path to the ICO output file.
    pub output_path: PathBuf,

    /// Whether to overwrite the ICO output file.
    pub force: bool,

    /// Whether to optimize PNG input.
    pub optimize: bool,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(<input> "The PNG input file"))
            .arg(arg!(-o --output <path> "The ICO output file"))
            .arg(arg!(-f --force "Overwrite the ICO output file"))
            .arg(arg!(-z --optimize "Optimize PNG input"))
            .get_matches();

        let input_path: PathBuf = args.get_one::<String>("input").unwrap().into();

        let output_path: PathBuf = match args.get_one::<String>("output") {
            Some(path) => path.into(),
            None => input_path.with_extension("ico"),
        };

        Config {
            input_paths: vec![input_path],
            output_path,
            force: args.get_flag("force"),
            optimize: args.get_flag("optimize"),
        }
    }
}
