use super::super::super::{annotate::*, normal::*};

use {
    depiction::*,
    derive_more::*,
    std::{fmt, io},
};

//
// InvalidKeyError
//

/// Invalid key error.
#[derive(Debug, Error)]
pub struct InvalidKeyError {
    /// Key.
    pub key: Variant<WithoutAnnotations>,
}

impl InvalidKeyError {
    /// Constructor.
    pub fn new<AnnotatedT>(key: Variant<AnnotatedT>) -> Self {
        Self { key: key.remove_annotations() }
    }
}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}

impl Depict for InvalidKeyError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "invalid key: {}", context.theme.error(key))
    }
}
