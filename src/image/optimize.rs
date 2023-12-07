extern crate oxipng;

use std::num::NonZeroU8;

use crate::config::OptLevel;

use super::Image;

use oxipng::Options;

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
        OptLevel::Low => build_options(false, false),
        OptLevel::Medium => build_options(true, true),
        OptLevel::High => build_options(true, false),
    }
}

/// Build oxipng optimization options from Zopfli and fast evaluation settings.
fn build_options(zopfli: bool, fast: bool) -> Options {
    let mut options = Options::max_compression();
    options.optimize_alpha = true;
    options.strip = oxipng::StripChunks::All;

    if zopfli {
        options.deflate = oxipng::Deflaters::Zopfli {
            iterations: NonZeroU8::new(15).unwrap(),
        }
    }

    options.fast_evaluation = fast;
    options
}
