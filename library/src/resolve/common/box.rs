use super::super::{super::normal::*, errors::*, resolve::*};

use kutil::std::error::*;

impl<BoxedT, AnnotatedT> Resolve<Box<BoxedT>, AnnotatedT> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<BoxedT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Box<BoxedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        Ok(self.resolve_with_errors(errors)?.map(|boxed| boxed.into()))
    }
}
