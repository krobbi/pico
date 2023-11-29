use crate::image::Image;

/// The size of an ICO header in bytes.
const HEADER_SIZE: usize = 6;

/// The size of an ICO entry in bytes.
const ENTRY_SIZE: usize = 16;

/// Serialize little-endian unsigned integers to a mutable serializer.
trait Serialize {
    /// Put a u8 value to the serializer.
    fn put_u8(&mut self, value: u8);

    /// Put a u16 value to the serializer.
    fn put_u16(&mut self, value: u16) {
        self.put_u8((value & 0xff) as u8);
        self.put_u8((value >> 8 & 0xff) as u8);
    }

    /// Put a u32 value to the serializer.
    fn put_u32(&mut self, value: u32) {
        self.put_u16((value & 0xffff) as u16);
        self.put_u16((value >> 16 & 0xffff) as u16);
    }
}

impl Serialize for Vec<u8> {
    fn put_u8(&mut self, value: u8) {
        self.push(value);
    }
}

/// Serialize ICO data from a vector of images.
pub fn serialize_ico(images: &Vec<Image>) -> Vec<u8> {
    let mut ico = Vec::new();

    // Reserved, must always be 0.
    ico.put_u16(0);

    // Image type, value 1 means icon, value 2 means cursor.
    ico.put_u16(1);

    // Number of images in the icon.
    ico.put_u16(images.len() as u16);

    debug_assert_eq!(
        ico.len(),
        HEADER_SIZE,
        "ICO header is not {} bytes long.",
        HEADER_SIZE
    );

    let mut offset = HEADER_SIZE + images.len() * ENTRY_SIZE;
    let mut data = Vec::new();

    for image in images {
        ico.append(&mut serialize_entry(image, offset));
        data.append(&mut image.data.clone());
        offset += image.data.len();
    }

    ico.append(&mut data);
    ico
}

/// Serialize an ICO entry using an image with a data offset.
fn serialize_entry(image: &Image, offset: usize) -> Vec<u8> {
    let mut entry = Vec::with_capacity(ENTRY_SIZE);

    // Image width in pixels, value 0 means 256 pixels.
    entry.put_u8(match image.width {
        ..=0xff => image.width as u8,
        _ => 0,
    });

    // Image height in pixels, value 0 means 256 pixels.
    entry.put_u8(match image.height {
        ..=0xff => image.height as u8,
        _ => 0,
    });

    // Number of colors in the color palette, value 0 means no color palette.
    entry.put_u8(match image.palette_size {
        Some(..=0xff) => image.palette_size.unwrap() as u8,
        _ => 0,
    });

    // Reserved, should be 0.
    entry.put_u8(0);

    // Color planes, should be 0 or 1. Exact usage is unclear.
    entry.put_u16(0);

    // Bits per pixel.
    entry.put_u16(image.bits_per_pixel as u16);

    // Image data size in bytes.
    entry.put_u32(image.data.len() as u32);

    // Image data offset from start of file.
    entry.put_u32(offset as u32);

    debug_assert_eq!(
        entry.len(),
        ENTRY_SIZE,
        "ICO entry is not {} bytes long.",
        ENTRY_SIZE
    );

    entry
}
