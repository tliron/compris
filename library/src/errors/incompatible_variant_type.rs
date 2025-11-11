use super::super::normal::*;

use {
    depiction::*,
    derive_more::*,
    kutil::std::string::*,
    std::{fmt, io},
};

//
// IncompatibleVariantTypeError
//

/// Incompatible value type.
#[derive(Debug, Error)]
pub struct IncompatibleVariantTypeError {
    /// Type name.
    pub type_name: String,

    /// Expected type names.
    pub expected_type_names: Vec<String>,
}

impl IncompatibleVariantTypeError {
    /// Constructor.
    pub fn new(type_name: String, expected_type_names: Vec<String>) -> Self {
        Self { type_name, expected_type_names }
    }

    /// Constructor.
    pub fn new_from<AnnotatedT>(variant: &Variant<AnnotatedT>, expected_type_names: &[&str]) -> Self {
        Self::new(
            variant.type_name().into(),
            expected_type_names.iter().map(|type_name| String::from(*type_name)).collect(),
        )
    }
}

impl fmt::Display for IncompatibleVariantTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_names.join_conjunction("or"))
    }
}

impl Depict for IncompatibleVariantTypeError {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(
            writer,
            "incompatible variant type: is {}, expected {}",
            context.theme.error(&self.type_name),
            self.expected_type_names.join_conjunction("or")
        )
    }
}
