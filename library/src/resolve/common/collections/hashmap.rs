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

impl<KeyT, ValueT, BuildHasherT, AnnotatedT> Resolve<HashMap<KeyT, ValueT, BuildHasherT>> for Variant<AnnotatedT>
where
    KeyT: Hash + Eq,
    Variant<AnnotatedT>: Resolve<KeyT> + Resolve<ValueT>,
    AnnotatedT: Annotated + Clone + Default,
    BuildHasherT: BuildHasher + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        errors: &mut ProblemReceiverT,
    ) -> ResolveResult<HashMap<KeyT, ValueT, BuildHasherT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut resolved = HashMap::default();

        if let Some(mut iterator) = ResolvingKeyValuePairIterator::new_from(self, errors)? {
            while let Some((key, value)) = iterator.resolve_next(errors)? {
                resolved.insert(key, value);
            }
        }

        Ok(Some(resolved))
    }
}
