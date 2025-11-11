use super::super::super::{
    super::{annotate::*, errors::*, normal::*},
    errors::*,
    iterator::*,
    resolve::*,
};

use {problemo::*, std::vec};

//
// ResolvingVariantIterator
//

/// Resolves an [Iterator] of [Variant], one item at a time.
///
/// Can be used directly on a [List].
///
/// Useful for implementing [Resolve] for list-like collections, such as [Vec].
pub struct ResolvingVariantIterator<InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = Variant<AnnotatedT>>,
{
    /// Inner.
    pub inner: InnerT,
}

impl<'this, InnerT, AnnotatedT> ResolvingVariantIterator<InnerT, AnnotatedT>
where
    InnerT: Iterator<Item = Variant<AnnotatedT>>,
{
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_for<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<IntoIter = InnerT>,
    {
        Self::new(iterable.into_iter())
    }
}

impl<'this, AnnotatedT> ResolvingVariantIterator<vec::IntoIter<Variant<AnnotatedT>>, AnnotatedT> {
    /// Constructor.
    pub fn new_from<ProblemReceiverT>(
        variant: Variant<AnnotatedT>,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<Self>
    where
        AnnotatedT: Annotated + Clone + Default,
        ProblemReceiverT: ProblemReceiver,
    {
        match variant {
            Variant::List(list) => return Ok(Some(Self::new_for(list))),

            _ => problems.give(IncompatibleVariantTypeError::as_problem_from(&variant, &["list"]).via(ResolveError))?,
        }

        Ok(None)
    }
}

impl<'this, ResolvedT, InnerT, AnnotatedT> ResolvingIterator<ResolvedT> for ResolvingVariantIterator<InnerT, AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<ResolvedT>,
    InnerT: Iterator<Item = Variant<AnnotatedT>>,
{
    fn resolve_next<ProblemReceiverT>(&mut self, problems: &mut ProblemReceiverT) -> ResolveResult<ResolvedT>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match self.inner.next() {
            Some(next) => next.resolve_with_problems(problems)?,
            None => None,
        })
    }
}
