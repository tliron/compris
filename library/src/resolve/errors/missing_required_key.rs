use super::super::super::{annotate::*, normal::*};

use {
    depiction::*,
    derive_more::*,
    std::{fmt, io},
};

//
// MissingRequiredKeyError
//

/// Missing required key error.
#[derive(Debug, Error)]
pub struct MissingRequiredKeyError {
    /// Key.
    pub key: Variant<WithoutAnnotations>,
}

impl MissingRequiredKeyError {
    /// Constructor.
    pub fn new<AnnotatedT>(key: Variant<AnnotatedT>) -> Self {
        Self { key: key.remove_annotations() }
    }
}

impl fmt::Display for MissingRequiredKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}

impl Depict for MissingRequiredKeyError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "missing required key: {}", context.theme.error(key))
    }
}
