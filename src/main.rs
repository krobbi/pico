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

    let images = match read_images(&config) {
        Ok(images) => images,
        Err(message) => bail(&message),
    };

    let data = serialize::serialize_ico(&images);

    if let Err(error) = fs::write(config.output_path, data.as_slice()) {
        bail(&error.to_string());
    }
}

/// Read a vector of images using configuration data.
fn read_images(config: &Config) -> Result<Vec<Image>, String> {
    let mut images = Vec::with_capacity(config.input_paths.len());

    for path in &config.input_paths {
        images.push(read_image(config, path)?);
    }

    Ok(images)
}

/// Read an image using configuration data and a path.
fn read_image(config: &Config, path: &PathBuf) -> Result<Image, String> {
    let image = Image::new(path)?;

    match config.optimize {
        true => image.optimize(),
        false => Ok(image),
    }
}

/// Exit with an error message.
fn bail(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}
