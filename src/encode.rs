use std::{
    error,
    fmt::{self, Display, Formatter},
    num,
};

use crate::image::Image;

/// Consumes a vector of images and returns them encoded into ICO data.
pub fn encode_icon(images: Vec<Image>) -> Result<Vec<u8>, Error> {
    const HEADER_SIZE: usize = 6;
    const ENTRY_SIZE: usize = 16;

    let image_count = images.len();

    let image_count = match u16::try_from(image_count) {
        Ok(image_count) => image_count,
        Err(error) => return Err(Error::TooManyImages(image_count, error)),
    };

    let mut buffer = Buffer::new(HEADER_SIZE + usize::from(image_count) * ENTRY_SIZE);

    // Values must be written in order according to the ICO format:
    // https://en.wikipedia.org/wiki/ICO_(file_format)#Header
    buffer.write_u16(0); // Reserved. Must always be 0.
    buffer.write_u16(1); // Specifies image type: 1 for icon, 2 for cursor.
    buffer.write_u16(image_count); // Specifies number of images in the file.
    debug_assert!(buffer.data.len() == HEADER_SIZE);

    Ok(buffer.into_data())
}

/// A buffer that can be written to with ICO values.
struct Buffer {
    /// The inner data.
    data: Vec<u8>,
}

impl Buffer {
    /// Creates a new buffer with a capacity.
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Writes a `u16` value to the buffer.
    fn write_u16(&mut self, value: u16) {
        self.data.extend_from_slice(&value.to_le_bytes());
    }

    /// Consumes the buffer and returns its underlying data.
    fn into_data(self) -> Vec<u8> {
        self.data
    }
}

/// An error encountered while encoding ICO data.
#[derive(Debug)]
pub enum Error {
    /// An error caused by having more images than are supported.
    TooManyImages(usize, num::TryFromIntError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::TooManyImages(_, error) => Some(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyImages(image_count, _) => write!(
                f,
                "{image_count} images were found, but only up to {} are supported",
                u16::MAX
            ),
        }
    }
}
