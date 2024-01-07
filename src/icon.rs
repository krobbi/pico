mod serialize;

use crate::image::Image;

/// An ICO icon.
pub struct Icon {
    /// The icon's images.
    images: Vec<Image>,
}

impl Icon {
    /// Create a new icon from a vector of images.
    pub fn from_images(images: Vec<Image>) -> Icon {
        let mut icon = Icon {
            images: Vec::with_capacity(images.len()),
        };

        for image in images {
            icon.insert_image(image);
        }

        icon
    }

    /// Serialize the icon to ICO data.
    pub fn serialize(&self) -> Vec<u8> {
        // TODO: Refactor serialization and move to this module.
        serialize::serialize_ico(&self.images)
    }

    /// Insert an image into the icon.
    fn insert_image(&mut self, image: Image) {
        self.images.push(image);
    }
}
