use std::{
    error,
    fmt::{self, Display, Formatter},
    io::{self, Cursor, Read},
    num::{self, NonZeroU32},
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

    /// Reads the next width or height value.
    pub fn read_dimension(&mut self) -> Result<NonZeroU32> {
        match NonZeroU32::try_from(self.read_u32()?) {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::ZeroDimension(error)),
        }
    }

    /// Consumes the cursor and returns its underlying data.
    pub fn into_data(self) -> Vec<u8> {
        self.cursor.into_inner()
    }

    /// Skips to the next chunk.
    fn next_chunk(&mut self) -> Result<()> {
        // Add 4 to skip the chunk's CRC field.
        self.cursor
            .set_position(self.chunk_position + u64::from(self.chunk_length) + 4);

        self.begin_chunk()
    }

    /// Begins a new current chunk.
    fn begin_chunk(&mut self) -> Result<()> {
        self.chunk_length = self.read_u32()?;
        self.chunk_type = self.read_4u8()?;
        self.chunk_position = self.cursor.position();
        Ok(())
    }

    /// Reads the next `u32` value.
    fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.read_4u8()?))
    }

    /// Reads the next four `u8` values.
    fn read_4u8(&mut self) -> Result<[u8; 4]> {
        let mut bytes = [0; 4];
        self.cursor.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

/// An error encountered while decoding PNG data.
#[derive(Debug)]
pub enum Error {
    /// An error caused by an I/O error.
    Io(io::Error),

    /// An error caused by PNG data's signature not being a PNG signature.
    SignatureNotPng,

    /// An error caused by PNG data's width or height being zero.
    ZeroDimension(num::TryFromIntError),
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
            Self::SignatureNotPng => None,
            Self::ZeroDimension(error) => Some(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::SignatureNotPng => f.write_str("signature is not a PNG image signature"),
            Self::ZeroDimension(_) => f.write_str("width or height is zero"),
        }
    }
}
