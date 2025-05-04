use std::path::PathBuf;

use clap::{arg, command, value_parser};

use crate::error::Result;

/// Configuration data for Pico.
pub struct Config {
    /// The paths to the PNG input files and directories.
    pub input_paths: Vec<PathBuf>,

    /// The path to the ICO output file.
    pub output_path: PathBuf,

    /// Whether to sort ICO entries by descending resolution.
    pub sort: bool,

    /// Whether to overwrite an existing ICO output file.
    pub force: bool,
}

impl Config {
    /// Creates new configuration data from command line arguments.
    pub fn new() -> Result<Self> {
        let mut matches = command!()
            .arg(
                arg!(<input>... "One or more PNG input files or directories")
                    .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(-o --output <path> "ICO output file")
                    .required(false)
                    .value_parser(value_parser!(PathBuf)),
            )
            .arg(arg!(-s --sort "Sort ICO entries by resolution"))
            .arg(arg!(-f --force "Overwrite existing ICO output file"))
            .try_get_matches()?;

        let input_paths: Vec<PathBuf> = matches.remove_many("input").unwrap_or_default().collect();
        assert!(
            !input_paths.is_empty(),
            "usage string guarantees that `input_paths` is not empty"
        );

        let output_path = match matches.remove_one("output") {
            Some(path) => path,
            None => input_paths
                .first()
                .expect("already asserted that `input_paths` is not empty")
                .with_extension("ico"),
        };

        Ok(Self {
            input_paths,
            output_path,
            sort: matches.get_flag("sort"),
            force: matches.get_flag("force"),
        })
    }
}
