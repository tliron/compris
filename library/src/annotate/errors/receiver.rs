use super::super::{annotations::*, r#struct::*, traits::*};

use kutil::std::error::*;

//
// ErrorReceiverWithFallbackAnnotations
//

/// An [ErrorReceiver] wrapper that adds an [Annotations] to errors that don't already have
/// [Annotations].
pub struct ErrorReceiverWithFallbackAnnotations<'own, InnerT> {
    /// Inner.
    pub inner: &'own mut InnerT,

    /// Fallback annotations.
    pub fallback_annotations: Option<&'own Annotations>,
}

impl<'own, InnerT> ErrorReceiverWithFallbackAnnotations<'own, InnerT> {
    /// Constructor.
    pub fn new(inner: &'own mut InnerT, fallback_annotations: Option<&'own Annotations>) -> Self {
        Self { inner, fallback_annotations }
    }
}

impl<'own, ErrorT, InnerT> ErrorReceiver<ErrorT> for ErrorReceiverWithFallbackAnnotations<'own, InnerT>
where
    ErrorT: Annotated,
    InnerT: ErrorReceiver<ErrorT>,
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
    ) -> ErrorReceiverWithFallbackAnnotations<'own, InnerT>;

    /// With fallback annotations from field.
    fn with_fallback_annotations_from_field<StructT>(
        &'own mut self,
        r#struct: &'own StructT,
        name: &str,
    ) -> ErrorReceiverWithFallbackAnnotations<'own, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_fallback_annotations(r#struct.field_or_struct_annotations(name))
    }

    /// With fallback annotations from struct.
    fn with_fallback_annotations_from_struct<StructT>(
        &'own mut self,
        r#struct: &'own StructT,
    ) -> ErrorReceiverWithFallbackAnnotations<'own, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_fallback_annotations(r#struct.struct_annotations())
    }
}

impl<'own, ErrorT, ErrorReceiverT> WithFallbackAnnotations<'own, ErrorT, ErrorReceiverT> for ErrorReceiverT
where
    ErrorT: Annotated,
    ErrorReceiverT: ErrorReceiver<ErrorT>,
{
    fn with_fallback_annotations(
        &'own mut self,
        annotations: Option<&'own Annotations>,
    ) -> ErrorReceiverWithFallbackAnnotations<'own, ErrorReceiverT> {
        ErrorReceiverWithFallbackAnnotations::new(self, annotations)
    }
}

/// Wrap errors with fallback [Annotations] from field.
#[macro_export]
macro_rules! errors_with_fallback_annotations_from_field {
    ( $new_errors:ident, $errors:expr, $self:expr, $field:expr, $( $code:tt )* ) => {
        {
            let annotations = $self.field_or_struct_annotations($field).cloned();
            let $new_errors = &mut $errors.with_fallback_annotations(annotations.as_ref());
            $( $code )*
        }
    };
}

#[allow(unused_imports)]
pub use errors_with_fallback_annotations_from_field;
