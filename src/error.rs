use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Decoding(png::DecodingError),
    Encoding(png::EncodingError),
    UnsupportedImageType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(error) => error.fmt(f),
            Error::Decoding(error) => error.fmt(f),
            Error::Encoding(error) => error.fmt(f),
            Error::UnsupportedImageType =>
                write!(f, "Only non-animated 8-bit RGB images are supported"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Decoding(err) => Some(err),
            Error::Encoding(err) => Some(err),
            Error::UnsupportedImageType => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Error {
        Error::Decoding(err)
    }
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Error {
        Error::Encoding(err)
    }
}
