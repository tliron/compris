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
#[derive(Debug, Error)]
pub struct MalformedError {
    /// Type name.
    pub type_name: String,

    /// Reason.
    pub reason: String,
}

impl_depict_error_function!(MalformedError);

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
        problem_with_depict_error!(Self::new(type_name, reason))
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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
    }
}
