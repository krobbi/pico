use std::{error, fmt, io, path::PathBuf};

/// An error caught by Pico.
#[derive(Debug)]
pub enum Error {
    /// A general error with a message string.
    Message(String),

    /// An error caused by an IO error.
    IO(io::Error),

    /// An error caused by having no valid input paths.
    NoInputs,

    /// An error caused by a PNG input file not existing.
    InputMissing(PathBuf),

    /// An error caused by the ICO output file existing without the '--force'
    /// option enabled.
    OutputExists(PathBuf),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(message) => message.fmt(f),
            Error::IO(error) => error.fmt(f),
            Error::NoInputs => write!(f, "No PNG input files were found."),
            Error::InputMissing(path) => {
                write!(f, "PNG input file '{}' does not exist.", path.display())
            }
            Error::OutputExists(path) => write!(
                f,
                "ICO output file '{}' already exists. Use '--force' to overwrite.",
                path.display()
            ),
        }
    }
}
