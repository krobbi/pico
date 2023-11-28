extern crate oxipng;

use super::Image;

use oxipng::{Options, StripChunks};

impl Image {
    /// Create an optimized copy of the image.
    pub fn optimize(&self) -> Image {
        let options = get_options();

        match oxipng::optimize_from_memory(self.data.as_slice(), &options) {
            Ok(data) => Image::from_data(data).unwrap(),
            Err(error) => {
                eprintln!("{}", error);
                Image::from_data(self.data.clone()).unwrap()
            }
        }
    }
}

/// Get optimization options.
fn get_options() -> Options {
    let mut options = Options::max_compression();
    options.optimize_alpha = true;
    options.strip = StripChunks::All;
    options.fast_evaluation = false;
    options
}
