mod config;
mod error;
mod image;
mod serialize;

use std::{fs, path::PathBuf, process};

use config::Config;
use error::Error;
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

    let images = read_images(&config.input_paths)?;
    let data = serialize::serialize_ico(&images);

    match fs::write(&config.output_path, data.as_slice()) {
        Ok(_) => Ok(()),
        Err(error) => Err(Error::IO(error)),
    }
}

/// Read a vector of images using a vector of paths.
fn read_images(paths: &Vec<PathBuf>) -> Result<Vec<Image>, Error> {
    let mut images = Vec::with_capacity(paths.len());

    for path in paths {
        images.push(Image::new(path)?);
    }

    Ok(images)
}
