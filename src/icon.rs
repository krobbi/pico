use crate::{
    error::{Error, Result},
    image::Image,
};

/// An ICO icon.
pub struct Icon {
    /// The entries.
    images: Vec<Image>,
}

impl Icon {
    /// Creates a new icon from its entries.
    pub fn new(images: Vec<Image>) -> Self {
        Self { images }
    }

    /// Encode the icon to ICO data.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let Ok(image_count) = u16::try_from(self.images.len()) else {
            return Err(Error::EncodeFailed);
        };

        // Values must be written in the order of the ICO file format:
        // https://en.wikipedia.org/wiki/ICO_(file_format)#Header
        let mut ico = Vec::new();
        ico.put_u16(0); // Reserved. Must always be 0.
        ico.put_u16(1); // Specifies image type: 1 for icon, 2 for cursor.
        ico.put_u16(image_count);

        let mut data = Vec::new();
        let mut data_offset = 6 + usize::from(image_count) * 16;

        for image in &self.images {
            ico.put_dimension_checked(image.width)?;
            ico.put_dimension_checked(image.height)?;

            // Specifies number of colors in the color palette. Should be 0 if
            // the image does not use a palette.
            match u8::try_from(image.palette_size.unwrap_or_default()) {
                Ok(palette_size) => ico.put_u8(palette_size),
                Err(_) => return Err(Error::EncodeFailed),
            }

            ico.put_u8(0); // Reserved. Should be 0.
            ico.put_u16(1); // Specifies color planes. should be 0 or 1.
            ico.put_u16(u16::from(image.bits_per_pixel));
            let data_size = image.data.len();
            ico.put_usize_checked(data_size)?;
            ico.put_usize_checked(data_offset)?;
            data.append(&mut image.data.clone());
            data_offset += data_size;
        }

        ico.append(&mut data);
        Ok(ico)
    }
}

/// A buffer for encoding ICO data.
trait Buffer {
    /// Put a u8 value to the buffer.
    fn put_u8(&mut self, value: u8);

    /// Put a u16 value to the buffer.
    fn put_u16(&mut self, value: u16);

    /// Put a u32 value to the buffer.
    fn put_u32(&mut self, value: u32);

    /// Put a usize value to the buffer with a 32-bit range.
    fn put_usize_checked(&mut self, value: usize) -> Result<()> {
        match u32::try_from(value) {
            Ok(value) => {
                self.put_u32(value);
                Ok(())
            }
            Err(_) => Err(Error::EncodeFailed),
        }
    }

    /// Put an image dimension to the buffer with a range check.
    fn put_dimension_checked(&mut self, value: u32) -> Result<()> {
        match value {
            value @ 1..=255 => {
                self.put_u8(
                    u8::try_from(value)
                        .expect("pattern should guarantee that `value` is valid as a `u8`"),
                );

                Ok(())
            }
            256 => {
                self.put_u8(0);
                Ok(())
            }
            _ => Err(Error::EncodeFailed),
        }
    }
}

impl Buffer for Vec<u8> {
    fn put_u8(&mut self, value: u8) {
        self.push(value);
    }

    fn put_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn put_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }
}
