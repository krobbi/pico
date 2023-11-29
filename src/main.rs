mod config;
mod image;
mod serialize;

use std::fs;
use std::path::PathBuf;
use std::process;

use config::Config;
use image::Image;

/// Display the configured source path.
fn main() {
    let config = Config::new();

    if config.target_path.is_file() && !config.force {
        bail(&format!(
            "Target ICO file '{}' already exists. Use '--force' to overwrite.",
            config.target_path.display()
        ));
    }

    let data = serialize::serialize_ico(&read_images(&config));

    if let Err(error) = fs::write(config.target_path, data.as_slice()) {
        bail(&error.to_string());
    }
}

/// Read a vector of images from configuration data or bail.
fn read_images(config: &Config) -> Vec<Image> {
    vec![read_image(&config.source_path, config.optimize)]
}

/// Read an image from a path and optimization status or bail.
fn read_image(path: &PathBuf, optimize: bool) -> Image {
    let image = match Image::new(path) {
        Ok(image) => image,
        Err(message) => bail(&message),
    };

    if optimize {
        match image.optimize() {
            Ok(image) => image,
            Err(message) => bail(&message),
        }
    } else {
        image
    }
}

/// Exit with an error message.
fn bail(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}
