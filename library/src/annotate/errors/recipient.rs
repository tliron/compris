use super::super::{annotated::*, annotations::*, r#struct::*};

use kutil::std::error::*;

//
// ErrorRecipientWithFallbackAnnotations
//

/// An [ErrorRecipient] wrapper that adds an [Annotations] to errors that don't already have
/// [Annotations].
pub struct ErrorRecipientWithFallbackAnnotations<'own, InnerT> {
    /// Inner.
    pub inner: &'own mut InnerT,

    /// Fallback annotations.
    pub fallback_annotations: Option<&'own Annotations>,
}

impl<'own, InnerT> ErrorRecipientWithFallbackAnnotations<'own, InnerT> {
    /// Constructor.
    pub fn new(inner: &'own mut InnerT, fallback_annotations: Option<&'own Annotations>) -> Self {
        Self { inner, fallback_annotations }
    }
}

impl<'own, ErrorT, InnerT> ErrorRecipient<ErrorT> for ErrorRecipientWithFallbackAnnotations<'own, InnerT>
where
    ErrorT: Annotated,
    InnerT: ErrorRecipient<ErrorT>,
{
    fn give_error(&mut self, error: ErrorT) -> Result<(), ErrorT> {
        if !error.has_annotations()
            && let Some(annotations) = self.fallback_annotations
        {
            self.inner.give_error(error.with_annotations(annotations.clone()))
        } else {
            self.inner.give_error(error)
        }
    }
}

//
// WithFallbackAnnotations
//

/// With fallback annotations.
pub trait WithFallbackAnnotations<'own, ErrorT, InnerT> {
    /// With fallback annotations.
    fn with_fallback_annotations(
        &'own mut self,
        annotations: Option<&'own Annotations>,
    ) -> ErrorRecipientWithFallbackAnnotations<'own, InnerT>;

    /// With fallback annotations from field.
    fn with_fallback_annotations_from_field<StructT>(
        &'own mut self,
        r#struct: &'own StructT,
        name: &str,
    ) -> ErrorRecipientWithFallbackAnnotations<'own, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_fallback_annotations(r#struct.field_annotations(name))
    }

    /// With fallback annotations from struct.
    fn with_fallback_annotations_from_struct<StructT>(
        &'own mut self,
        r#struct: &'own StructT,
    ) -> ErrorRecipientWithFallbackAnnotations<'own, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_fallback_annotations(r#struct.struct_annotations())
    }
}

impl<'own, ErrorT, ErrorRecipientT> WithFallbackAnnotations<'own, ErrorT, ErrorRecipientT> for ErrorRecipientT
where
    ErrorT: Annotated,
    ErrorRecipientT: ErrorRecipient<ErrorT>,
{
    fn with_fallback_annotations(
        &'own mut self,
        annotations: Option<&'own Annotations>,
    ) -> ErrorRecipientWithFallbackAnnotations<'own, ErrorRecipientT> {
        ErrorRecipientWithFallbackAnnotations::new(self, annotations)
    }
}
