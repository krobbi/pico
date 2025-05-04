use std::{
    error,
    fmt::{self, Display, Formatter},
    io::{self, Write},
    path::PathBuf,
    process::{ExitCode, Termination},
    result,
};

/// A result that may contain a Pico error.
pub type Result<T> = result::Result<T, Error>;

/// An error raised by Pico.
#[derive(Debug)]
pub enum Error {
    /// An error caused by an I/O error.
    Io(io::Error),

    /// An error caused by the ICO output file already existing without using
    /// the '--force' flag.
    OutputExists(PathBuf),

    /// An error caused by having no valid PNG input file paths.
    NoInputs,

    /// An error caused by a PNG input file not existing.
    InputMissing(PathBuf),

    /// An error caused by a PNG input file failing to be decoded.
    DecodeFailed(PathBuf),

    /// An error caused by the ICO output file failing to be encoded.
    EncodeFailed,
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
            Self::OutputExists(path) => write!(
                f,
                "ICO output file '{}' already exists, try '--force' to overwrite it",
                path.display()
            ),
            Self::NoInputs => f.write_str("no PNG input file paths were found"),
            Self::InputMissing(path) => {
                write!(f, "PNG input file '{}' does not exist", path.display())
            }
            Self::DecodeFailed(path) => write!(
                f,
                "PNG input file '{}' could not be decoded",
                path.display()
            ),
            Self::EncodeFailed => f.write_str("ICO output file could not be encoded"),
        }
    }
}

impl Termination for Error {
    fn report(self) -> ExitCode {
        // According to the standard library implementation of `Termination` for
        // `Result`, this function should avoid panicking. This line is
        // equivalent to `eprintln!("error: {self}");`, but any errors from
        // writing to stderr are ignored.
        let _ = writeln!(io::stderr(), "error: {self}");

        ExitCode::FAILURE
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
