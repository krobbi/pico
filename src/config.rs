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
    pub sort_entries: bool,

    /// Whether to overwrite an existing ICO output file.
    pub overwrite_output: bool,
}

impl Config {
    /// Creates new configuration data from command line arguments.
    pub fn new() -> Result<Self> {
        let mut matches = command!()
            .arg(
                arg!(<INPUT>... "One or more PNG input file or directory paths")
                    .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(-o --output <PATH> "ICO output file path")
                    .value_parser(value_parser!(PathBuf)),
            )
            .arg(arg!(-s --sort "Sort ICO entries by descending resolution"))
            .arg(arg!(-f --force "Overwrite existing ICO output file"))
            .try_get_matches()?;

        let input_paths: Vec<PathBuf> = matches.remove_many("INPUT").unwrap_or_default().collect();

        let output_path = match matches.remove_one("output") {
            None => input_paths
                .first()
                .expect("usage string should guarantee that `input_paths` is not empty")
                .with_extension("ico"),
            Some(path) => path,
        };

        Ok(Self {
            input_paths,
            output_path,
            sort_entries: matches.get_flag("sort"),
            overwrite_output: matches.get_flag("force"),
        })
    }
}
