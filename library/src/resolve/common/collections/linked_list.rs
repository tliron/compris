use super::super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use {problemo::*, std::collections::*};

// Uses push_back

impl<ItemT, AnnotatedT> Resolve<LinkedList<ItemT>> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<ItemT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(self, errors: &mut ProblemReceiverT) -> ResolveResult<LinkedList<ItemT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut resolved = LinkedList::default();

        if let Some(mut iterator) = ResolvingVariantIterator::new_from(self, errors)? {
            while let Some(item) = iterator.resolve_next(errors)? {
                resolved.push_back(item);
            }
        }

        Ok(Some(resolved))
    }
}
