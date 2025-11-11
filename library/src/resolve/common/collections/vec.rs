use super::super::{
    super::{
        super::{annotate::*, normal::*},
        errors::*,
        iterator::*,
        resolve::*,
    },
    iterate::*,
};

use problemo::*;

impl<ItemT, AnnotatedT> Resolve<Vec<ItemT>> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<ItemT>,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(self, errors: &mut ProblemReceiverT) -> ResolveResult<Vec<ItemT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let mut resolved = Vec::default();

        if let Some(mut iterator) = ResolvingVariantIterator::new_from(self, errors)? {
            while let Some(item) = iterator.resolve_next(errors)? {
                resolved.push(item);
            }
        }

        Ok(Some(resolved))
    }
}
