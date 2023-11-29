mod config;
mod image;
mod serialize;

use std::fs;
use std::path::PathBuf;
use std::process;

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

    let data = serialize::serialize_ico(&read_images(&config));

    if let Err(error) = fs::write(config.output_path, data.as_slice()) {
        bail(&error.to_string());
    }
}

/// Read a vector of images using configuration data or bail.
fn read_images(config: &Config) -> Vec<Image> {
    let mut images = Vec::with_capacity(config.input_paths.len());

    for path in &config.input_paths {
        images.push(read_image(path, config.optimize));
    }

    images
}

/// Read an image using a path and optimization status or bail.
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
