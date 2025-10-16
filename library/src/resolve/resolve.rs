use super::errors::*;

use kutil::std::error::*;

//
// Resolve
//

/// Resolve one type into another.
pub trait Resolve<ResolvedT, AnnotatedT>: Sized {
    /// Resolve one type into another.
    ///
    /// A lot like [TryFrom], except that we can accumulate potentially annotated errors.
    ///
    /// Errors can be reported as usual by [Err] *but also* by the [ErrorReceiver]. Callers should
    /// thus check that `errors` is empty even when the function returns [Ok].
    ///
    /// The function may return [Some] partially resolved result even if there are errors.
    fn resolve_with_errors<ErrorReceiverT>(self, errors: &mut ErrorReceiverT) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>;

    /// Resolve one type into another.
    ///
    /// A lot like [TryFrom], except that we can accumulate potentially annotated errors.
    ///
    /// Unlike [resolve](Resolve::resolve) will fail on the first encountered error and will return
    /// [ResolveError::Missing] instead of [None].
    ///
    /// If you want all the errors use [resolve](Resolve::resolve) instead.
    fn resolve(self) -> Result<ResolvedT, ResolveError<AnnotatedT>> {
        self.resolve_with_errors(&mut FailFastErrorReceiver)?.ok_or(ResolveError::Missing)
    }
}
