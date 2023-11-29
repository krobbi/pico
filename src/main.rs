mod config;
mod image;

use std::process;

use config::Config;
use image::Image;

/// Display the configured source path.
fn main() {
    let config = Config::new();

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

    println!("Image info:");
    println!(" * {}x{} pixels.", image.width, image.height);

    match image.palette_size {
        Some(palette_size) => println!(" * {} color palette.", palette_size),
        None => println!(" * No palette."),
    }

    match image.bits_per_pixel {
        1 => println!(" * 1 bit per pixel."),
        bits_per_pixel => println!(" * {} bits per pixel.", bits_per_pixel),
    }

    println!(" * {} bytes of data.", image.data.len());
}

/// Exit with an error message.
fn bail(message: &str) -> ! {
    eprintln!("{}", message);
    process::exit(1)
}
