use super::super::super::{annotate::*, normal::*};

use {
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// InvalidKeyError
//

/// Invalid key error.
#[derive(Debug, Error, PartialEq)]
pub struct InvalidKeyError {
    /// Key.
    pub key: Variant<WithoutAnnotations>,
}

impl InvalidKeyError {
    /// Constructor.
    pub fn new<AnnotatedT>(key: Variant<AnnotatedT>) -> Self {
        Self { key: key.remove_annotations() }
    }

    /// Constructor.
    pub fn as_problem<AnnotatedT>(key: Variant<AnnotatedT>) -> Problem {
        Self::new(key).into_problem().with(AnnotatedCauseEquality::new::<Self>()).with(ErrorDepiction::new::<Self>())
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

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.key.to_string())
    }
}
