use super::super::{annotations::*, r#struct::*, traits::*};

use problemo::*;

//
// ProblemReceiverWithFallbackAnnotations
//

/// A [ProblemReceiver] wrapper that attaches [Annotations] to problem that don't already them.
pub struct ProblemReceiverWithFallbackAnnotations<'this, InnerT> {
    /// Inner.
    pub inner: &'this mut InnerT,

    /// Fallback annotations.
    pub fallback_annotations: Option<&'this Annotations>,
}

impl<'this, InnerT> ProblemReceiverWithFallbackAnnotations<'this, InnerT> {
    /// Constructor.
    pub fn new(inner: &'this mut InnerT, fallback_annotations: Option<&'this Annotations>) -> Self {
        Self { inner, fallback_annotations }
    }
}

impl<'this, InnerT> ProblemReceiver for ProblemReceiverWithFallbackAnnotations<'this, InnerT>
where
    InnerT: ProblemReceiver,
{
    fn give(&mut self, problem: Problem) -> Result<(), Problem> {
        if !problem.has_annotations()
            && let Some(annotations) = self.fallback_annotations
        {
            self.inner.give(problem.with_annotations(annotations.clone()))
        } else {
            self.inner.give(problem)
        }
    }
}

//
// WithFallbackAnnotations
//

/// With fallback annotations.
pub trait WithFallbackAnnotations<'this, InnerT> {
    /// With fallback annotations.
    fn with_fallback_annotations(
        &'this mut self,
        annotations: Option<&'this Annotations>,
    ) -> ProblemReceiverWithFallbackAnnotations<'this, InnerT>;

    /// With fallback annotations from field.
    fn with_fallback_annotations_from_field<StructT>(
        &'this mut self,
        r#struct: &'this StructT,
        name: &str,
    ) -> ProblemReceiverWithFallbackAnnotations<'this, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_fallback_annotations(r#struct.field_or_struct_annotations(name))
    }

    /// With fallback annotations from struct.
    fn with_fallback_annotations_from_struct<StructT>(
        &'this mut self,
        r#struct: &'this StructT,
    ) -> ProblemReceiverWithFallbackAnnotations<'this, InnerT>
    where
        StructT: AnnotatedStruct,
    {
        self.with_fallback_annotations(r#struct.struct_annotations())
    }
}

impl<'this, ProblemReceiverT> WithFallbackAnnotations<'this, ProblemReceiverT> for ProblemReceiverT
where
    ProblemReceiverT: ProblemReceiver,
{
    fn with_fallback_annotations(
        &'this mut self,
        annotations: Option<&'this Annotations>,
    ) -> ProblemReceiverWithFallbackAnnotations<'this, ProblemReceiverT> {
        ProblemReceiverWithFallbackAnnotations::new(self, annotations)
    }
}

/// Wrap a problem receiver with fallback [Annotations] from field.
#[macro_export]
macro_rules! problems_with_fallback_annotations_from_field {
    ( $new_problems:ident, $problems:expr, $self:expr, $field:expr, $( $code:tt )* ) => {
        {
            let annotations = $self.field_or_struct_annotations($field).cloned();
            let $new_problems = &mut $problems.with_fallback_annotations(annotations.as_ref());
            $( $code )*
        }
    };
}

#[allow(unused_imports)]
pub use problems_with_fallback_annotations_from_field;
