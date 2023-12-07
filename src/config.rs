use std::path::PathBuf;

use clap::{arg, command};

/// An optimization level for PNG data.
pub enum OptLevel {
    /// Maximum oxipng level with alpha optimization and chunk stripping.
    Optimized,
}

/// Configuration data for Pico.
pub struct Config {
    /// The paths to the PNG input files.
    pub input_paths: Vec<PathBuf>,

    /// The path to the ICO output file.
    pub output_path: PathBuf,

    /// Whether to overwrite an existing ICO output file.
    pub force: bool,

    /// The PNG optimization level to use, if applicable.
    pub opt_level: Option<OptLevel>,
}

impl Config {
    /// Create a new config using command line arguments.
    pub fn new() -> Config {
        let args = command!()
            .arg(arg!(<input>... "One or more PNG input files"))
            .arg(arg!(-o --output <path> "ICO output file"))
            .arg(arg!(-f --force "Overwrite existing ICO output file"))
            .arg(arg!(-z --optimize ... "Optimize PNG input"))
            .get_matches();

        let input_paths: Vec<PathBuf> = args
            .get_many::<String>("input")
            .unwrap()
            .map(PathBuf::from)
            .collect();

        let output_path: PathBuf = match args.get_one::<String>("output") {
            Some(path) => PathBuf::from(path),
            None => input_paths[0].with_extension("ico"),
        };

        Config {
            input_paths,
            output_path,
            force: args.get_flag("force"),
            opt_level: match args.get_one::<u8>("optimize").unwrap() {
                0 => None,
                1 => Some(OptLevel::Optimized),
                _ => Some(OptLevel::Optimized),
            },
        }
    }
}
