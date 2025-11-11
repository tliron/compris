use super::super::annotate::*;

use {
    depiction::*,
    derive_more::*,
    problemo::*,
    std::{fmt, io},
};

//
// MalformedError
//

/// Malformed error.
#[derive(Debug, Error, PartialEq)]
pub struct MalformedError {
    /// Type name.
    pub type_name: String,

    /// Reason.
    pub reason: String,
}

impl MalformedError {
    /// Constructor.
    pub fn new<TypeNameT, ReasonT>(type_name: TypeNameT, reason: ReasonT) -> Self
    where
        TypeNameT: ToString,
        ReasonT: ToString,
    {
        Self { type_name: type_name.to_string(), reason: reason.to_string() }
    }

    /// Constructor.
    pub fn as_problem<TypeNameT, ReasonT>(type_name: TypeNameT, reason: ReasonT) -> Problem
    where
        TypeNameT: ToString,
        ReasonT: ToString,
    {
        Self::new(type_name, reason)
            .into_problem()
            .with(AnnotatedCauseEquality::new::<Self>())
            .with(ErrorDepiction::new::<Self>())
    }
}

impl Depict for MalformedError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "malformed {}: {}", self.type_name, context.theme.error(&self.reason))
    }
}

impl fmt::Display for MalformedError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
    }
}
