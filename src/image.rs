use std::{fs, path::PathBuf};

use png;

use crate::error::Error;

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
    pub fn new(path: &PathBuf) -> Result<Image, Error> {
        if !path.is_file() {
            return Err(Error::InputMissing(path.clone()));
        }

        match fs::read(path) {
            Ok(data) => Image::from_data(data),
            Err(error) => Err(Error::IO(error)),
        }
    }

    /// Create a new image from data.
    fn from_data(data: Vec<u8>) -> Result<Image, Error> {
        let decoder = png::Decoder::new(data.as_slice());

        let reader = match decoder.read_info() {
            Ok(reader) => reader,
            Err(error) => return Err(Error::Message(error.to_string())),
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
