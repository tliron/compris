use super::super::super::{annotate::*, normal::*};

use {
    depiction::*,
    std::{fmt, io},
    thiserror::*,
};

//
// InvalidKeyError
//

/// Invalid key.
#[derive(Debug, Error)]
pub struct InvalidKeyError<AnnotatedT> {
    /// Key.
    pub key: Variant<AnnotatedT>,
}

impl_annotated!(InvalidKeyError, key);

impl<AnnotatedT> InvalidKeyError<AnnotatedT> {
    /// Constructor.
    pub fn new(key: Variant<AnnotatedT>) -> Self {
        Self { key }
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<InvalidKeyError<NewAnnotatedT>> for InvalidKeyError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> InvalidKeyError<NewAnnotatedT> {
        InvalidKeyError::new(self.key.into_annotated())
    }
}

impl<AnnotatedT> Depict for InvalidKeyError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let key = format!("{:?}", self.key.to_string());
        write!(writer, "invalid key: {}", context.theme.error(key))
    }
}

impl<AnnotatedT> fmt::Display for InvalidKeyError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}
