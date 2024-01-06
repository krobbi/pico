mod config;
mod image;
mod serialize;

use std::{fs, path::PathBuf, process};

use config::Config;
use image::Image;

/// Run Pico.
fn main() {
    let config = Config::new();

    if config.output_path.is_file() && !config.force {
        bail(&format!(
            "ICO output file '{}' already exists. Use '--force' to overwrite.",
            config.output_path.display()
        ));
    }

    let images = match read_images(&config.input_paths) {
        Ok(images) => images,
        Err(message) => bail(&message),
    };

    let data = serialize::serialize_ico(&images);

    if let Err(error) = fs::write(config.output_path, data.as_slice()) {
        bail(&error.to_string());
    }
}

/// Read a vector of images using a vector of paths.
fn read_images(paths: &Vec<PathBuf>) -> Result<Vec<Image>, String> {
    let mut images = Vec::with_capacity(paths.len());

    for path in paths {
        images.push(Image::new(path)?);
    }

    Ok(images)
}

/// Exit with an error message.
fn bail(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}
