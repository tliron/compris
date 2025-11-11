use super::super::{super::normal::*, errors::*, resolve::*};

use problemo::*;

impl<InnerT, AnnotatedT> Resolve<Box<InnerT>> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<InnerT>,
{
    fn resolve_with_problems<ProblemReceiverT>(self, problems: &mut ProblemReceiverT) -> ResolveResult<Box<InnerT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(self.resolve_with_problems(problems)?.map(|inner| inner.into()))
    }
}
