use super::super::super::annotate::*;

use {
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// MalformedError
//

/// Malformed error.
#[derive(Debug, Error)]
pub struct MalformedError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Reason.
    pub reason: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(MalformedError);

impl<AnnotatedT> MalformedError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String, reason: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, reason, annotated: Default::default() }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<MalformedError<NewAnnotatedT>> for MalformedError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> MalformedError<NewAnnotatedT> {
        MalformedError::new(self.type_name, self.reason).with_annotations_from(&self.annotated)
    }
}

impl<AnnotatedT> Depict for MalformedError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "malformed {}: {}", self.type_name, context.theme.error(&self.reason))
    }
}

impl<AnnotatedT> fmt::Display for MalformedError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
    }
}
