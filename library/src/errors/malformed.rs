use {
    depiction::*,
    derive_more::*,
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

impl MalformedError {
    /// Constructor.
    pub fn new(type_name: String, reason: String) -> Self {
        Self { type_name, reason }
    }
}

impl fmt::Display for MalformedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "malformed {}: {}", self.type_name, self.reason)
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
