use super::super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use {
    problemo::*,
    std::{collections::*, hash::*},
};

impl<KeyT, ValueT, AnnotatedT> Resolve<BTreeMap<KeyT, ValueT>> for Variant<AnnotatedT>
where
    KeyT: Hash + Eq + Ord,
    Variant<AnnotatedT>: Resolve<KeyT> + Resolve<ValueT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        errors: &mut ProblemReceiverT,
    ) -> ResolveResult<BTreeMap<KeyT, ValueT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut resolved = BTreeMap::default();

        if let Some(mut iterator) = ResolvingKeyValuePairIterator::new_from(self, errors)? {
            while let Some((key, value)) = iterator.resolve_next(errors)? {
                resolved.insert(key, value);
            }
        }

        Ok(Some(resolved))
    }
}
