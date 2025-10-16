use {std::fmt, thiserror::*};

//
// CborWriteError
//

/// CBOR write error.
#[derive(Debug, Error)]
pub enum CborWriteError {
    /// Encode error.
    EncodeError(borc::errors::EncodeError),

    /// Custom error.
    Custom(String),
}

impl fmt::Display for CborWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncodeError(encode_error) => write!(formatter, "{:?}", encode_error),
            Self::Custom(custom) => fmt::Display::fmt(&custom, formatter),
        }
    }
}

impl serde::ser::Error for CborWriteError {
    fn custom<DisplayableT>(custom: DisplayableT) -> Self
    where
        DisplayableT: fmt::Display,
    {
        Self::Custom(format!("{}", custom))
    }
}

// Conversions

impl From<borc::errors::EncodeError> for CborWriteError {
    fn from(encode_error: borc::errors::EncodeError) -> Self {
        Self::EncodeError(encode_error)
    }
}
