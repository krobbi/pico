mod optimize;

use std::{fs, path::PathBuf};

use png;

/// PNG image data.
pub struct Image {
    /// The image's width in pixels.
    pub width: u32,

    /// The image's height in pixels.
    pub height: u32,

    /// The number of colors in the image's palette, if applicable.
    pub palette_size: Option<u16>,

    /// The number of bits per pixel in the image.
    pub bits_per_pixel: u8,

    /// The image's raw data.
    pub data: Vec<u8>,
}

impl Image {
    /// Create a new image using a path.
    pub fn new(path: &PathBuf) -> Result<Image, String> {
        if !path.is_file() {
            return Err(format!(
                "Source PNG file '{}' does not exist.",
                path.display()
            ));
        }

        let data = match fs::read(path) {
            Ok(data) => data,
            Err(error) => return Err(error.to_string()),
        };

        Image::from_data(data)
    }

    /// Create a new image from data.
    fn from_data(data: Vec<u8>) -> Result<Image, String> {
        let decoder = png::Decoder::new(data.as_slice());

        let reader = match decoder.read_info() {
            Ok(reader) => reader,
            Err(error) => return Err(error.to_string()),
        };

        let info = reader.info();

        Ok(Image {
            width: info.width,
            height: info.height,
            palette_size: match &info.palette {
                Some(palette) => Some(palette.len() as u16 / 3),
                None => None,
            },
            bits_per_pixel: info.bits_per_pixel() as u8,
            data,
        })
    }
}
