mod config;
mod image;
mod serialize;

use std::fs;
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

    let mut image = match Image::new(&config.source_path) {
        Ok(image) => image,
        Err(message) => bail(&message),
    };

    if config.optimize {
        image = match image.optimize() {
            Ok(image) => image,
            Err(message) => bail(&message),
        };
    }

    let data = serialize::serialize_ico(&vec![image]);

    if let Err(error) = fs::write(config.target_path, data.as_slice()) {
        bail(&error.to_string());
    }
}

/// Exit with an error message.
fn bail(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}
