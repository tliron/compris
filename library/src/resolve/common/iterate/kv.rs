use super::super::super::{
    super::{annotate::*, errors::*, kv::*, normal::*},
    errors::*,
    iterator::*,
    resolve::*,
};

use problemo::*;

//
// ResolvingKeyValuePairIterator
//

/// Resolves a [KeyValuePairIterator], one key-value pair at a time.
///
/// Both keys and values are resolved.
///
/// Note that the implementation relies on `dyn` to support different [KeyValuePairIterator]
/// implementations.
///
/// Useful for implementing [Resolve] for map-like collections, such as
/// [HashMap](std::collections::HashMap).
pub struct ResolvingKeyValuePairIterator<'this, AnnotatedT> {
    /// Inner key-value pair iterator.
    pub inner: Box<dyn IntoKeyValuePairIterator<AnnotatedT> + 'this>,
}

impl<'this, AnnotatedT> ResolvingKeyValuePairIterator<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: Box<dyn IntoKeyValuePairIterator<AnnotatedT> + 'this>) -> Self {
        Self { inner }
    }

    /// Constructor.
    pub fn new_from<ProblemReceiverT>(
        variant: Variant<AnnotatedT>,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<Self>
    where
        AnnotatedT: 'this + Annotated + Clone + Default,
        ProblemReceiverT: ProblemReceiver,
    {
        if variant.is_collection() {
            let iterator = variant.into_key_value_iterator().expect("map or list");
            Ok(Some(Self::new(iterator)))
        } else {
            problems.give(
                IncompatibleVariantTypeError::new_from(&variant, &["map", "list"]).into_problem().via(ResolveError),
            )?;
            Ok(None)
        }
    }
}

impl<'this, KeyT, ValueT, AnnotatedT> ResolvingIterator<(KeyT, ValueT)>
    for ResolvingKeyValuePairIterator<'this, AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<KeyT>,
    Variant<AnnotatedT>: Resolve<ValueT>,
    AnnotatedT: Annotated + Default,
{
    fn resolve_next<ProblemReceiverT>(&mut self, problems: &mut ProblemReceiverT) -> ResolveResult<(KeyT, ValueT)>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        // Repeat until we get a non-error
        loop {
            match self.inner.next() {
                Ok(next) => {
                    return Ok(match next {
                        Some(pair) => pair.resolve_with_problems(problems)?,
                        None => None,
                    });
                }

                Err((error, cause)) => {
                    problems.give(error.into_problem().maybe_with(cause.annotations().cloned()).via(ResolveError))?
                }
            }
        }
    }
}
