use super::super::{super::normal::*, errors::*, resolve::*};

use kutil::std::error::*;

// We only have to care about Some, because None will never get resolved
// (A Null is definitely not a None and requires entirely different consideration)

impl<OptionalT, AnnotatedT> Resolve<Option<OptionalT>, AnnotatedT> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<OptionalT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorReceiverT>(
        self,
        errors: &mut ErrorReceiverT,
    ) -> ResolveResult<Option<OptionalT>, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
    {
        Ok(Some(self.resolve_with_errors(errors)?))
    }
}
