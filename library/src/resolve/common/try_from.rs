use super::super::{
    super::{annotate::*, errors::*, normal::*},
    errors::*,
    resolve::*,
};

use {
    problemo::*,
    std::{fmt, marker::*},
    tynm::*,
};

/// Resolve a [Variant] into a [TryFrom] via an intermediate.
pub fn resolve_try_from<TryFromT, IntermediateT, AnnotatedT, ProblemReceiverT>(
    variant: Variant<AnnotatedT>,
    problems: &mut ProblemReceiverT,
) -> ResolveResult<TryFromT>
where
    Variant<AnnotatedT>: TryInto<IntermediateT>,
    <Variant<AnnotatedT> as TryInto<IntermediateT>>::Error: fmt::Display,
    TryFromT: TryFrom<IntermediateT>,
    TryFromT::Error: fmt::Display,
    AnnotatedT: Annotated + Clone + Default,
    ProblemReceiverT: ProblemReceiver,
{
    let maybe_annotations = variant.maybe_annotations();

    let intermediate: IntermediateT = match variant.try_into() {
        Ok(intermediate) => intermediate,

        Err(error) => {
            problems.give(
                MalformedError::as_problem(type_name::<IntermediateT>(), error.to_string())
                    .maybe_with(maybe_annotations.annotations().cloned())
                    .via(ResolveError),
            )?;
            return Ok(None);
        }
    };

    Ok(match intermediate.try_into() {
        Ok(resolved) => Some(resolved),

        Err(error) => {
            problems.give(
                MalformedError::as_problem(type_name::<TryFromT>(), error.to_string())
                    .maybe_with(maybe_annotations.annotations().cloned())
                    .via(ResolveError),
            )?;
            None
        }
    })
}

//
// ResolveTryFrom
//

/// A wrapper for a [TryFrom] that implements [Resolve].
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveTryFrom<InnerT, IntermediateT> {
    /// Inner.
    pub inner: InnerT,

    intermediate: PhantomData<IntermediateT>,
}

impl<InnerT, IntermediateT> ResolveTryFrom<InnerT, IntermediateT> {
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner, intermediate: PhantomData }
    }
}

impl<InnerT, IntermediateT, AnnotatedT> Resolve<ResolveTryFrom<InnerT, IntermediateT>> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: TryInto<IntermediateT>,
    <Variant<AnnotatedT> as TryInto<IntermediateT>>::Error: fmt::Display,
    InnerT: TryFrom<IntermediateT>,
    InnerT::Error: fmt::Display,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        errors: &mut ProblemReceiverT,
    ) -> ResolveResult<ResolveTryFrom<InnerT, IntermediateT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        resolve_try_from(self, errors).map(|resolved| resolved.map(ResolveTryFrom::new))
    }
}

impl<InnerT, IntermediateT> AsRef<InnerT> for ResolveTryFrom<InnerT, IntermediateT> {
    fn as_ref(&self) -> &InnerT {
        &self.inner
    }
}

impl<InnerT, IntermediateT> From<InnerT> for ResolveTryFrom<InnerT, IntermediateT> {
    fn from(inner: InnerT) -> Self {
        Self::new(inner)
    }
}

impl<InnerT, IntermediateT> fmt::Display for ResolveTryFrom<InnerT, IntermediateT>
where
    InnerT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}
