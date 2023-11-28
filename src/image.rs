use std::path::PathBuf;

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
    pub fn new(path: &PathBuf) -> Result<Image, String> {
        if !path.is_file() {
            return Err(format!(
                "Source PNG file '{}' does not exist.",
                path.display()
            ));
        }

        Ok(Image {
            width: 1,
            height: 2,
            palette_size: Some(3),
            bits_per_pixel: 4,
            data: vec![1, 2, 3, 4, 5],
        })
    }
}
