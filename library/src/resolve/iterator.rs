use super::errors::*;

use problemo::*;

/// Iterator that resolves one item at a time.
pub trait ResolvingIterator<ResolvedT> {
    /// Resolve next.
    ///
    /// Important: An error returned here does *not* mean that there are no more entries, just that
    /// the current iteration caused an error. Future ones might not. To exhaust the iterator, keep
    /// calling this function until it returns [None].
    fn resolve_next<ProblemReceiverT>(&mut self, errors: &mut ProblemReceiverT) -> ResolveResult<ResolvedT>
    where
        ProblemReceiverT: ProblemReceiver;
}
