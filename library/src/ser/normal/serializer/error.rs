use {
    serde::*,
    std::{error::*, fmt},
};

//
// SerializeVariantError
//

/// Serialize variant error.
#[derive(Debug)]
pub struct SerializeVariantError(pub String);

impl fmt::Display for SerializeVariantError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl Error for SerializeVariantError {}

impl ser::Error for SerializeVariantError {
    fn custom<DisplayT>(message: DisplayT) -> Self
    where
        DisplayT: fmt::Display,
    {
        Self(message.to_string())
    }
}
