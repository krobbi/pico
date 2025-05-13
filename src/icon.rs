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
        let image_count = self.images.len();

        if image_count > u16::MAX as usize {
            return Err(Error::EncodeFailed);
        }

        let mut ico = Vec::new();
        ico.put_u16(0); // Reserved, should be 0.
        ico.put_u16(1); // Image type, 1 for icon, 2 for cursor.
        ico.put_u16(image_count as u16);

        let mut data = Vec::new();
        let mut data_offset = 6 + image_count * 16;

        for image in &self.images {
            ico.put_dimension_checked(image.width)?;
            ico.put_dimension_checked(image.height)?;

            match image.palette_size {
                Some(1..=255) => ico.put_u8(image.palette_size.unwrap() as u8),
                None => ico.put_u8(0),
                _ => return Err(Error::EncodeFailed),
            }

            ico.put_u8(0); // Reserved, should be 0.
            ico.put_u16(1); // Color planes, should be 0 or 1.
            ico.put_u16(image.bits_per_pixel as u16);
            let data_size = image.data.len();
            ico.put_u32_checked(data_size)?;
            ico.put_u32_checked(data_offset)?;
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

    /// Put a u32 value to the buffer with a range check.
    fn put_u32_checked(&mut self, value: usize) -> Result<()> {
        if value <= u32::MAX as usize {
            self.put_u32(value as u32);
            Ok(())
        } else {
            Err(Error::EncodeFailed)
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
