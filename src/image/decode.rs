use std::{
    error,
    fmt::{self, Display, Formatter},
    io::{self, Cursor, Read},
    result,
};

/// A result that may contain a decode error.
type Result<T> = result::Result<T, Error>;

/// A cursor that can read values from PNG data.
pub struct PngCursor {
    /// The inner cursor.
    cursor: Cursor<Vec<u8>>,

    /// The length of the current chunk's data field in bytes.
    chunk_length: u32,

    /// The type of the current chunk.
    chunk_type: [u8; 4],

    /// The position of the current chunk's data field.
    chunk_position: u64,
}

impl PngCursor {
    /// Creates a new cursor from PNG data.
    pub fn new(data: Vec<u8>) -> Result<Self> {
        const PNG_SIGNATURE: [u8; 8] = *b"\x89PNG\r\n\x1a\n";

        let mut cursor = Cursor::new(data);
        let mut signature = [0; 8];
        cursor.read_exact(&mut signature)?;

        if signature != PNG_SIGNATURE {
            return Err(Error::SignatureNotPng);
        }

        let mut cursor = Self {
            cursor,
            chunk_length: 0,
            chunk_type: [0; 4],
            chunk_position: 0,
        };

        cursor.begin_chunk()?;
        Ok(cursor)
    }

    /// Skips chunks until the current chunk matches a chunk type.
    pub fn find_chunk(&mut self, chunk_type: [u8; 4]) -> Result<()> {
        while self.chunk_type != chunk_type {
            self.next_chunk()?;
        }

        Ok(())
    }

    /// Returns the length of the current chunk's data field in bytes.
    pub fn chunk_length(&self) -> u32 {
        self.chunk_length
    }

    /// Reads the next `u8` value.
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut value = [0];
        self.cursor.read_exact(&mut value)?;
        Ok(value[0])
    }

    /// Reads the next `u32` value.
    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.read_four_bytes()?))
    }

    /// Reads the next color type value.
    pub fn read_color_type(&mut self) -> Result<ColorType> {
        let value = self.read_u8()?;

        match ColorType::from_u8(value) {
            Some(color_type) => Ok(color_type),
            None => Err(Error::InvalidColorType(value)),
        }
    }

    /// Consumes the cursor and returns its underlying data.
    pub fn into_data(self) -> Vec<u8> {
        self.cursor.into_inner()
    }

    /// Skips to the next chunk.
    fn next_chunk(&mut self) -> Result<()> {
        // Add 4 to skip the current chunk's CRC field.
        self.cursor
            .set_position(self.chunk_position + u64::from(self.chunk_length) + 4);

        self.begin_chunk()
    }

    /// Begins a new current chunk.
    fn begin_chunk(&mut self) -> Result<()> {
        self.chunk_length = self.read_u32()?;
        self.chunk_type = self.read_four_bytes()?;
        self.chunk_position = self.cursor.position();
        Ok(())
    }

    /// Reads the next four bytes.
    fn read_four_bytes(&mut self) -> Result<[u8; 4]> {
        let mut bytes = [0; 4];
        self.cursor.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

/// A color type allowed in a PNG image. Based on the `ColorType` enum from the
/// `png` crate.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ColorType {
    /// A color type where pixels have a grey sample.
    Greyscale = 0,

    /// A color type where pixels have red, green, and blue samples.
    Truecolor = 2,

    /// A color type where pixels have a palette index.
    IndexedColor = 3,

    /// A color type where pixels have a grey sample and an alpha sample.
    GreyscaleWithAlpha = 4,

    /// A color type where pixels have red, green, blue, and alpha samples.
    TruecolorWithAlpha = 6,
}

impl ColorType {
    /// Returns the number of samples per pixel provided by the color type.
    pub fn samples_per_pixel(self) -> u8 {
        match self {
            Self::Greyscale | Self::IndexedColor => 1,
            Self::Truecolor => 3,
            Self::GreyscaleWithAlpha => 2,
            Self::TruecolorWithAlpha => 4,
        }
    }

    /// Creates a new optional color type from a `u8` value.
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Greyscale),
            2 => Some(Self::Truecolor),
            3 => Some(Self::IndexedColor),
            4 => Some(Self::GreyscaleWithAlpha),
            6 => Some(Self::TruecolorWithAlpha),
            _ => None,
        }
    }
}

/// An error encountered while decoding PNG data.
#[derive(Debug)]
pub enum Error {
    /// An error caused by an I/O error.
    Io(io::Error),

    /// An error caused by PNG data's signature not being a PNG signature.
    SignatureNotPng,

    /// An error caused by PNG data having an invalid color type value.
    InvalidColorType(u8),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::SignatureNotPng => f.write_str("signature is not a PNG image signature"),
            Self::InvalidColorType(value) => write!(f, "invalid color type of {value}"),
        }
    }
}
