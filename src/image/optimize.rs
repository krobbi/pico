use super::Image;

impl Image {
    /// Create an optimized copy of the image.
    pub fn optimize(&self) -> Image {
        println!("TODO: Implement optimization.");
        Image::from_data(self.data.clone()).unwrap()
    }
}
