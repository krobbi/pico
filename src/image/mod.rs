mod decode;

pub use decode::Error as DecodeError;

use std::{fs, num::NonZeroU32, path::PathBuf};

use decode::PngCursor;

use crate::error::{Error, Result};

/// A PNG image.
pub struct Image {
    /// The width in pixels.
    pub width: NonZeroU32,

    /// The height in pixels.
    pub height: NonZeroU32,

    /// The number of colors in the optional palette.
    pub palette_size: Option<u16>,

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

        try_decode!(cursor.find_chunk(*b"IHDR"));
        let width = try_decode!(cursor.read_dimension());
        let height = try_decode!(cursor.read_dimension());

        let data = cursor.into_data();

        let reader = match png::Decoder::new(data.as_slice()).read_info() {
            Ok(reader) => reader,
            Err(error) => return Err(Error::InputDecodeFailed(path, error)),
        };

        let info = reader.info();

        Ok(Self {
            width,
            height,
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
        // Convert dimensions to `u64` to avoid overflow.
        u64::from(self.width.get()) * u64::from(self.height.get())
    }
}
