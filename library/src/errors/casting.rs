use super::super::{annotate::*, normal::*};

use {
    depiction::*,
    derive_more::*,
    std::{fmt, io},
};

//
// CastingError
//

/// Casting error.
#[derive(Debug, Error)]
pub struct CastingError {
    /// Variant.
    pub variant: Variant<WithoutAnnotations>,

    /// Type name.
    pub type_name: String,
}

impl CastingError {
    /// Constructor.
    pub fn new<AnnotatedT>(variant: Variant<AnnotatedT>, type_name: String) -> Self {
        Self { variant: variant.remove_annotations(), type_name }
    }
}

impl fmt::Display for CastingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} cannot be cast to a {}", self.variant, self.type_name)
    }
}

impl Depict for CastingError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{} cannot be cast to a {}", self.variant, context.theme.error(&self.type_name))
    }
}
