mod config;
mod error;
mod icon;
mod image;

use std::{
    cmp::Reverse,
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use error::{Error, Exit, Result, RunResult};
use icon::Icon;
use image::Image;

/// Runs Pico and exits.
fn main() -> Exit {
    try_run().into()
}

/// Runs Pico and returns a result.
fn try_run() -> RunResult {
    let config = Config::new()?;

    if !config.overwrite_output && config.output_path.is_file() {
        return Err(Error::OutputExists(config.output_path));
    }

    let input_paths = expand_dir_paths(config.input_paths)?;

    if input_paths.is_empty() {
        return Err(Error::NoInputPaths);
    }

    let mut images = read_images(input_paths)?;

    if config.sort_entries {
        images.sort_by_key(|image| Reverse(image.resolution()));
    }

    let data = Icon::new(images).encode()?;
    fs::write(&config.output_path, data.as_slice())?;
    Ok(())
}

/// Consumes a vector of paths and returns a new vector with its directory paths
/// expanded into their child PNG file paths.
fn expand_dir_paths(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut expanded_paths = Vec::with_capacity(paths.len());

    for path in paths {
        if path.is_dir() {
            expanded_paths.append(&mut expand_dir_path(&path)?);
        } else {
            expanded_paths.push(path);
        }
    }

    Ok(expanded_paths)
}

/// Expands a directory path into a sorted vector of its child paths to PNG
/// files.
fn expand_dir_path(dir_path: &Path) -> Result<Vec<PathBuf>> {
    let mut expanded_paths = vec![];

    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();

        if path.is_file()
            && path
                .extension()
                .unwrap_or_default()
                .eq_ignore_ascii_case("png")
        {
            expanded_paths.push(path);
        }
    }

    expanded_paths.sort_unstable();
    Ok(expanded_paths)
}

/// Consumes a vector of file paths and returns a new vector of images from disk
/// at the corresponding paths.
fn read_images(paths: Vec<PathBuf>) -> Result<Vec<Image>> {
    let mut images = Vec::with_capacity(paths.len());

    for path in paths {
        images.push(Image::new(path)?);
    }

    Ok(images)
}
