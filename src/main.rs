mod config;
mod error;
mod icon;
mod image;

use std::{
    fs,
    path::PathBuf,
    process::{ExitCode, Termination},
};

use config::Config;
use error::{Error, Result};
use icon::Icon;
use image::Image;

/// Runs Pico and exits with an exit code.
fn main() -> ExitCode {
    // The result of `try_run()` could be returned instead, but this would cause
    // errors to be displayed with user-unfriendly debug printing.
    match try_run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => error.report(),
    }
}

/// Runs Pico and returns a result.
fn try_run() -> Result<()> {
    let config = Config::new();

    if config.output_path.is_file() && !config.force {
        return Err(Error::OutputExists(config.output_path.clone()));
    }

    let paths = expand_paths(&config.input_paths)?;
    let images = read_images(paths)?;
    let data = Icon::from_images(images, config.sort).encode()?;
    fs::write(&config.output_path, data.as_slice())?;
    Ok(())
}

/// Expand a vector of paths to PNG files and directories to a vector of paths
/// to PNG files.
fn expand_paths(paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut expanded = Vec::new();

    for path in paths {
        if path.is_dir() {
            expanded.append(&mut expand_dir(path)?);
        } else {
            expanded.push(path.clone());
        }
    }

    if expanded.is_empty() {
        Err(Error::NoInputs)
    } else {
        Ok(expanded)
    }
}

/// Expand a directory path to a vector of paths to PNG files.
fn expand_dir(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file()
            && path
                .extension()
                .unwrap_or_default()
                .eq_ignore_ascii_case("png")
        {
            paths.push(path);
        }
    }

    Ok(paths)
}

/// Read a vector of images using a vector of paths to PNG input files.
fn read_images(paths: Vec<PathBuf>) -> Result<Vec<Image>> {
    let mut images = Vec::with_capacity(paths.len());

    for path in paths {
        images.push(Image::from_path(path)?);
    }

    Ok(images)
}
