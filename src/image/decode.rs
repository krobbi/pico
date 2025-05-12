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
}

impl PngCursor {
    /// Creates a new cursor from PNG data.
    pub fn new(data: Vec<u8>) -> Result<Self> {
        const PNG_SIGNATURE: [u8; 8] = *b"\x89PNG\r\n\x1a\n";

        let mut cursor = Cursor::new(data);
        let mut signature = [0; 8];
        cursor.read_exact(&mut signature)?;

        if signature == PNG_SIGNATURE {
            Ok(Self { cursor })
        } else {
            Err(Error::SignatureNotPng)
        }
    }

    /// Consumes the cursor and returns its underlying data.
    pub fn into_data(self) -> Vec<u8> {
        self.cursor.into_inner()
    }
}

/// An error encountered while decoding PNG data.
#[derive(Debug)]
pub enum Error {
    /// An error caused by an I/O error.
    Io(io::Error),

    /// An error caused by PNG data's signature not being a PNG signature.
    SignatureNotPng,
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
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(f),
            Self::SignatureNotPng => f.write_str("signature is not a PNG image signature"),
        }
    }
}
