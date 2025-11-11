use super::super::{super::normal::*, errors::*, resolve::*};

use problemo::*;

// We only have to care about Some, because None will never get resolved
// (A Null is definitely not a None and requires entirely different consideration)

impl<OptionalT, AnnotatedT> Resolve<Option<OptionalT>> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<OptionalT>,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<Option<OptionalT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(Some(self.resolve_with_problems(problems)?))
    }
}
