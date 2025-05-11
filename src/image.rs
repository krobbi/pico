use std::{fs, path::PathBuf};

use crate::error::{Error, Result};

/// A PNG image.
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
    /// Creates a new image from its path.
    pub fn from_path(path: PathBuf) -> Result<Self> {
        if !path.is_file() {
            return Err(Error::InputMissing(path));
        }

        let data = fs::read(&path)?;

        let reader = match png::Decoder::new(data.as_slice()).read_info() {
            Ok(reader) => reader,
            Err(error) => return Err(Error::InputDecodeFailed(path, error)),
        };

        let info = reader.info();

        if info.is_animated() {
            return Err(Error::InputAnimated(path));
        }

        Ok(Self {
            width: info.width,
            height: info.height,
            palette_size: info
                .palette
                .as_ref()
                .map(|palette| palette.len() as u16 / 3),
            bits_per_pixel: info.bits_per_pixel() as u8,
            data,
        })
    }

    /// Returns the image's resolution in pixels.
    pub fn resolution(&self) -> u64 {
        // Convert dimensions from `u32` to `u64` to avoid overflow.
        u64::from(self.width) * u64::from(self.height)
    }
}
