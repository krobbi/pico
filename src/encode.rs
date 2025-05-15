use crate::{
    error::{Error, Result},
    image::Image,
};

/// Consumes a vector of images and returns them encoded into ICO data.
pub fn encode_icon(images: Vec<Image>) -> Result<Vec<u8>> {
    let mut ico = IcoBuffer::new(images.len())?;

    for image in images {
        ico.write_image(image)?;
    }

    Ok(ico.into_data())
}

/// A buffer that can write values as ICO data.
struct IcoBuffer {
    /// The directory data.
    dir: Vec<u8>,

    /// The size of the directory data.
    dir_size: usize,

    /// The image data.
    images: Vec<Vec<u8>>,

    /// The offset to the next image.
    next_image_offset: usize,
}

impl IcoBuffer {
    /// The size of an ICO header in bytes.
    const HEADER_SIZE: usize = 6;

    /// The size of an ICO directory entry in bytes.
    const ENTRY_SIZE: usize = 16;

    /// Creates a new buffer from an image count.
    fn new(image_count: usize) -> Result<Self> {
        let dir_size = Self::HEADER_SIZE + image_count * Self::ENTRY_SIZE;

        let mut ico = Self {
            dir: Vec::with_capacity(dir_size),
            dir_size,
            images: Vec::with_capacity(image_count),
            next_image_offset: dir_size,
        };

        let Ok(image_count) = u16::try_from(image_count) else {
            return Err(Error::EncodeFailed);
        };

        // Values must be written in order according to the ICO file format:
        // https://en.wikipedia.org/wiki/ICO_(file_format)#Header
        ico.write_u16(0); // Reserved. Must always be 0.
        ico.write_u16(1); // Specifies image type: 1 for icon, 2 for cursor.
        ico.write_u16(image_count);
        debug_assert_eq!(ico.dir.len(), Self::HEADER_SIZE);

        Ok(ico)
    }

    /// Consumes an image and writes it to the buffer.
    fn write_image(&mut self, image: Image) -> Result<()> {
        #[cfg(debug_assertions)]
        let expected_dir_size = self.dir.len() + Self::ENTRY_SIZE;

        self.write_dimension(image.width)?; // Specifies width in pixels.
        self.write_dimension(image.height)?; // Specifies height in pixels.

        // Specifies number of colors in the color palette. Should be 0 if
        // the image does not use a palette.
        match u8::try_from(image.palette_size.unwrap_or_default()) {
            Ok(palette_size) => self.write_u8(palette_size),
            Err(_) => return Err(Error::EncodeFailed),
        }

        self.write_u8(0); // Reserved. Should be 0.
        self.write_u16(1); // Specifies color planes. Should be 0 or 1.
        self.write_u16(u16::from(image.bits_per_pixel)); // Bits per pixel.

        // Specifies the size of the image's data in bytes.
        let image = image.data;
        let image_size = image.len();
        self.write_usize(image_size)?;

        // Specifies the offset of PNG data from the beginning of the ICO file.
        self.images.push(image);
        self.write_usize(self.next_image_offset)?;
        self.next_image_offset += image_size;

        debug_assert_eq!(self.dir.len(), expected_dir_size);
        Ok(())
    }

    /// Consumes the buffer and returns its underlying data.
    fn into_data(self) -> Vec<u8> {
        let mut data = self.dir;
        data.reserve(self.next_image_offset - self.dir_size);

        for mut image in self.images {
            data.append(&mut image);
        }

        data
    }

    /// Writes a `u8` value to the buffer.
    fn write_u8(&mut self, value: u8) {
        self.dir.push(value);
    }

    /// Writes a `u16` value to the buffer.
    fn write_u16(&mut self, value: u16) {
        self.dir.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a `u32` value to the buffer.
    fn write_u32(&mut self, value: u32) {
        self.dir.extend_from_slice(&value.to_le_bytes());
    }

    /// Writes a `usize` value to the buffer with a 32-bit range.
    fn write_usize(&mut self, value: usize) -> Result<()> {
        match u32::try_from(value) {
            Ok(value) => {
                self.write_u32(value);
                Ok(())
            }
            Err(_) => Err(Error::EncodeFailed),
        }
    }

    /// Writes a dimension value to the buffer.
    fn write_dimension(&mut self, value: u32) -> Result<()> {
        match value {
            value @ 1..=255 => {
                self.write_u8(
                    u8::try_from(value)
                        .expect("pattern should guarantee that `value` is valid as a `u8`"),
                );

                Ok(())
            }
            256 => {
                self.write_u8(0); // Value 0 means dimension is 256 pixels.
                Ok(())
            }
            _ => Err(Error::EncodeFailed),
        }
    }
}
