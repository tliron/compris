use super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    resolve::*,
};

use problemo::*;

impl<ResolvedAnnotationsT, AnnotatedT> Resolve<Variant<ResolvedAnnotationsT>> for Variant<AnnotatedT>
where
    ResolvedAnnotationsT: Annotated + Default,
    AnnotatedT: Annotated + Clone,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        _problems: &mut ProblemReceiverT,
    ) -> ResolveResult<Variant<ResolvedAnnotationsT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(Some(self.into_annotated()))
    }
}
