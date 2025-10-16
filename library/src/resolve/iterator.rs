use super::errors::*;

use kutil::std::error::*;

/// Iterator that resolves one item at a time.
pub trait ResolvingIterator<ResolvedT, AnnotatedT> {
    /// Resolve next.
    ///
    /// Important: An error returned here does *not* mean that there are no more entries, just that
    /// the current iteration caused an error. Future ones might not. To exhaust the iterator, keep
    /// calling this function until it returns [None].
    fn resolve_next<ErrorReceiverT>(&mut self, errors: &mut ErrorReceiverT) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>;
}
