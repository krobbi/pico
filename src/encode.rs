use crate::image::Image;

/// Consumes a vector of images and returns them encoded into ICO data.
pub fn encode_icon(images: Vec<Image>) -> Vec<u8> {
    let mut buffer = Buffer::new(6 + images.len() * 16);

    // Values must be written in order according to the ICO format:
    // https://en.wikipedia.org/wiki/ICO_(file_format)#Header
    buffer.write_u16(0); // Reserved. Must always be 0.
    buffer.write_u16(1); // Specifies image type: 1 for icon, 2 for cursor.

    buffer.into_data()
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
