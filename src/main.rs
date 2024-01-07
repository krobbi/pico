mod config;
mod error;
mod icon;
mod image;

use std::{fs, path::PathBuf, process};

use config::Config;
use error::Error;
use icon::Icon;
use image::Image;

/// Run Pico using command line arguments and exit on error.
fn main() {
    if let Err(error) = run_pico(&Config::new()) {
        eprintln!("{}", error);
        process::exit(1);
    }
}

/// Run Pico using configuration data.
fn run_pico(config: &Config) -> Result<(), Error> {
    if config.output_path.is_file() && !config.force {
        return Err(Error::OutputExists(config.output_path.clone()));
    }

    let paths = expand_paths(&config.input_paths)?;
    let images: Vec<Image> = read_images(&paths)?;
    let data = Icon::from_images(images).serialize();

    match fs::write(&config.output_path, data.as_slice()) {
        Ok(_) => Ok(()),
        Err(error) => Err(Error::IO(error)),
    }
}

/// Expand a vector of paths to PNG files and directories to a vector of paths
/// to PNG files.
fn expand_paths(paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>, Error> {
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
fn expand_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => return Err(Error::IO(error)),
    };

    let mut paths = Vec::new();

    for entry in entries {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(error) => return Err(Error::IO(error)),
        };

        if path.is_file() && path.extension().unwrap_or_default().to_ascii_lowercase() == "png" {
            paths.push(path);
        }
    }

    Ok(paths)
}

/// Read a vector of images using a vector of paths to PNG input files.
fn read_images(paths: &Vec<PathBuf>) -> Result<Vec<Image>, Error> {
    let mut images = Vec::with_capacity(paths.len());

    for path in paths {
        images.push(Image::new(path)?);
    }

    Ok(images)
}
