use super::super::{annotate::*, normal::*};

use {
    depiction::*,
    derive_more::*,
    problemo::*,
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

impl_depict_error_function!(CastingError);

impl CastingError {
    /// Constructor.
    pub fn new<AnnotatedT, TypeNameT>(variant: Variant<AnnotatedT>, type_name: TypeNameT) -> Self
    where
        TypeNameT: ToString,
    {
        Self { variant: variant.remove_annotations(), type_name: type_name.to_string() }
    }

    /// Constructor.
    pub fn as_problem<AnnotatedT, TypeNameT>(variant: Variant<AnnotatedT>, type_name: TypeNameT) -> Problem
    where
        TypeNameT: ToString,
    {
        problem_with_depict_error!(Self::new(variant, type_name))
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

impl fmt::Display for CastingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} cannot be cast to a {}", self.variant, self.type_name)
    }
}
