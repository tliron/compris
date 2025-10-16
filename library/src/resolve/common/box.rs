use super::super::{super::normal::*, errors::*, resolve::*};

use kutil::std::error::*;

impl<BoxedT, AnnotatedT> Resolve<Box<BoxedT>, AnnotatedT> for Variant<AnnotatedT>
where
    Variant<AnnotatedT>: Resolve<BoxedT, AnnotatedT>,
{
    fn resolve_with_errors<ErrorReceiverT>(self, errors: &mut ErrorReceiverT) -> ResolveResult<Box<BoxedT>, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
    {
        Ok(self.resolve_with_errors(errors)?.map(|boxed| boxed.into()))
    }
}
