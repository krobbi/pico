mod decode;

pub use decode::Error as DecodeError;

use std::{fs, path::PathBuf};

use decode::{ColorType, PngCursor};

use crate::error::{Error, Result};

/// A PNG image.
pub struct Image {
    /// The width in pixels.
    pub width: u32,

    /// The height in pixels.
    pub height: u32,

    /// The number of colors in the optional palette.
    pub palette_size: Option<u32>,

    /// The number of bits per pixel.
    pub bits_per_pixel: u8,

    /// The raw data.
    pub data: Vec<u8>,
}

impl Image {
    /// Creates a new image from its path.
    pub fn new(path: PathBuf) -> Result<Self> {
        if !path.is_file() {
            return Err(Error::InputMissing(path));
        }

        /// Evaluates an expression that may raise a PNG decoding error.
        macro_rules! try_decode {
            ($expr:expr) => {
                (match ($expr) {
                    Ok(value) => value,
                    Err::<_, DecodeError>(error) => return Err(Error::Decode(path, error)),
                })
            };
        }

        let mut cursor = try_decode!(PngCursor::new(fs::read(&path)?));

        // Values must be read in order according to the PNG specification:
        // https://www.w3.org/TR/png-3/#11IHDR
        let width = try_decode!(cursor.read_u32());
        let height = try_decode!(cursor.read_u32());

        let bit_depth = try_decode!(cursor.read_u8());
        let color_type = try_decode!(cursor.read_color_type());
        let bits_per_pixel = bit_depth * color_type.samples_per_pixel();

        let palette_size = match color_type {
            ColorType::IndexedColor => {
                try_decode!(cursor.find_chunk(*b"PLTE"));
                Some(cursor.chunk_length() / 3)
            }
            _ => None,
        };

        let data = cursor.into_data();

        Ok(Self {
            width,
            height,
            palette_size,
            bits_per_pixel,
            data,
        })
    }

    /// Returns the image's resolution in pixels.
    pub fn resolution(&self) -> u64 {
        // Convert dimensions from `u32` to `u64` to avoid overflow.
        u64::from(self.width) * u64::from(self.height)
    }
}
