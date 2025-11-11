use super::super::{super::normal::*, errors::*, resolve::*};

use problemo::*;

impl<BoxedT, AnnotatedT> Resolve<Box<BoxedT>> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<BoxedT>,
{
    fn resolve_with_problems<ProblemReceiverT>(self, problems: &mut ProblemReceiverT) -> ResolveResult<Box<BoxedT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(self.resolve_with_problems(problems)?.map(|boxed| boxed.into()))
    }
}
