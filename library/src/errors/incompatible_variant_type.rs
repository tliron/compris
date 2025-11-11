use super::super::normal::*;

use {
    depiction::*,
    derive_more::*,
    kutil::std::string::*,
    problemo::*,
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
    pub fn new<TypeNameT>(type_name: TypeNameT, expected_type_names: Vec<String>) -> Self
    where
        TypeNameT: ToString,
    {
        Self { type_name: type_name.to_string(), expected_type_names }
    }

    /// Constructor.
    pub fn as_problem<TypeNameT>(type_name: TypeNameT, expected_type_names: Vec<String>) -> Problem
    where
        TypeNameT: ToString,
    {
        Self::new(type_name, expected_type_names).into_problem().with(ErrorDepiction::new::<Self>())
    }

    /// Constructor.
    pub fn as_problem_from<AnnotatedT>(variant: &Variant<AnnotatedT>, expected_type_names: &[&str]) -> Problem {
        Self::as_problem(
            variant.type_name(),
            expected_type_names.iter().map(|type_name| String::from(*type_name)).collect(),
        )
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

impl fmt::Display for IncompatibleVariantTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_names.join_conjunction("or"))
    }
}
