use super::super::{
    super::{annotate::*, normal::*},
    errors::*,
    resolve::*,
};

use problemo::*;

impl<InnerT, AnnotatedT> Resolve<Annotate<InnerT, AnnotatedT>> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Default,
    Variant<AnnotatedT>: Resolve<InnerT>,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<Annotate<InnerT, AnnotatedT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let annotations = self.annotations().cloned();
        Ok(self.resolve_with_problems(problems)?.map(|inner| {
            let annotate = Annotate::new(inner);
            match annotations {
                Some(annotations) => annotate.with_annotations(annotations),
                None => annotate,
            }
        }))
    }
}
