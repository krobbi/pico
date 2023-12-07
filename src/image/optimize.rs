extern crate oxipng;

use crate::config::OptLevel;

use super::Image;

use oxipng::{Options, StripChunks};

impl Image {
    /// Create an optimized copy of the image using a PNG optimization level.
    pub fn optimize(&self, opt_level: &OptLevel) -> Result<Image, String> {
        let options = level_options(opt_level);

        match oxipng::optimize_from_memory(self.data.as_slice(), &options) {
            Ok(data) => Image::from_data(data),
            Err(error) => Err(error.to_string()),
        }
    }
}

/// Build oxipng optimization options using a PNG optimization level.
fn level_options(opt_level: &OptLevel) -> Options {
    match opt_level {
        OptLevel::Optimized => default_options(),
    }
}

/// Build default oxipng optimization options.
fn default_options() -> Options {
    let mut options = Options::max_compression();
    options.optimize_alpha = true;
    options.strip = StripChunks::All;
    options
}
