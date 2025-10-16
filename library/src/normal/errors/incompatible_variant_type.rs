use super::super::{super::annotate::*, variant::*};

use {
    depiction::*,
    kutil::std::string::*,
    std::{fmt, io},
    thiserror::*,
};

//
// IncompatibleVariantTypeError
//

/// Incompatible value type.
#[derive(Debug, Error)]
pub struct IncompatibleVariantTypeError<AnnotatedT> {
    /// Type name.
    pub type_name: String,

    /// Expected type names.
    pub expected_type_names: Vec<String>,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(IncompatibleVariantTypeError);

impl<AnnotatedT> IncompatibleVariantTypeError<AnnotatedT> {
    /// Constructor.
    pub fn new(type_name: String, expected_type_names: Vec<String>) -> Self
    where
        AnnotatedT: Default,
    {
        Self { type_name, expected_type_names, annotated: Default::default() }
    }

    /// Constructor.
    pub fn new_from(variant: &Variant<AnnotatedT>, expected_type_names: &[&str]) -> Self
    where
        AnnotatedT: Annotated + Default + Clone,
    {
        Self::new(
            variant.type_name().into(),
            expected_type_names.iter().map(|type_name| String::from(*type_name)).collect(),
        )
        .with_annotations_from(variant)
    }
}

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<IncompatibleVariantTypeError<NewAnnotatedT>>
    for IncompatibleVariantTypeError<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> IncompatibleVariantTypeError<NewAnnotatedT> {
        IncompatibleVariantTypeError::new(self.type_name, self.expected_type_names)
            .with_annotations_from(&self.annotated)
    }
}

impl<AnnotatedT> Depict for IncompatibleVariantTypeError<AnnotatedT> {
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

impl<AnnotatedT> fmt::Display for IncompatibleVariantTypeError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "is {}, expected {}", self.type_name, self.expected_type_names.join_conjunction("or"))
    }
}
