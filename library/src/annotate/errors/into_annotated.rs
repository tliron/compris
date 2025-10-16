use super::super::traits::*;

use {kutil::std::error::*, std::marker::*};

/// An [ErrorRecipient] wrapper that calls [IntoAnnotated::into_annotated] on all errors.
#[derive(Debug)]
pub struct IntoAnnotatedErrorRecipient<'own, InnerT, ErrorT>
where
    InnerT: ErrorRecipient<ErrorT>,
{
    /// Inner.
    pub inner: &'own mut InnerT,

    error: PhantomData<ErrorT>,
}

impl<'own, InnerT, ErrorT, NewErrorT> ErrorRecipient<NewErrorT> for IntoAnnotatedErrorRecipient<'own, InnerT, ErrorT>
where
    InnerT: ErrorRecipient<ErrorT>,
    ErrorT: IntoAnnotated<NewErrorT>,
    NewErrorT: IntoAnnotated<ErrorT>,
{
    fn give_error(&mut self, error: NewErrorT) -> Result<(), NewErrorT> {
        self.inner.give_error(error.into_annotated()).map_err(|error| error.into_annotated())
    }
}

impl<'own, InnerT, ErrorT> IntoAnnotated<IntoAnnotatedErrorRecipient<'own, InnerT, ErrorT>> for &'own mut InnerT
where
    InnerT: ErrorRecipient<ErrorT>,
    // ErrorT: IntoAnnotated<NewErrorT>,
    // NewT: ErrorRecipient<NewErrorT>,
    // NewErrorT: ErrorRecipient<ErrorT>,
{
    fn into_annotated(self) -> IntoAnnotatedErrorRecipient<'own, InnerT, ErrorT> {
        IntoAnnotatedErrorRecipient { inner: self, error: PhantomData }
    }
}
