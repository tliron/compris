use super::super::super::{annotate::*, normal::*};

use {
    kutil::cli::depict::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MissingRequiredKeyError
//

/// Missing required key.
#[derive(Debug, Error)]
pub struct MissingRequiredKeyError<AnnotatedT> {
    /// Key.
    pub key: Variant<AnnotatedT>,
}

impl_annotated!(MissingRequiredKeyError, key);

impl<AnnotatedT> MissingRequiredKeyError<AnnotatedT> {
    /// Constructor.
    pub fn new(key: Variant<AnnotatedT>) -> Self {
        Self { key }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<MissingRequiredKeyError<NewAnnotatedT>>
    for MissingRequiredKeyError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> MissingRequiredKeyError<NewAnnotatedT> {
        MissingRequiredKeyError::new(self.key.into_annotated())
    }
}

impl<AnnotatedT> Depict for MissingRequiredKeyError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "missing required key: {}", context.theme.error(key))
    }
}

impl<AnnotatedT> fmt::Display for MissingRequiredKeyError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}
