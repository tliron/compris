use super::errors::*;

use problemo::{common::*, *};

//
// Resolve
//

/// Resolve one type into another.
pub trait Resolve<ResolvedT>: Sized {
    /// Resolve one type into another.
    ///
    /// A lot like [TryFrom], except that we can accumulate potentially annotated errors.
    ///
    /// Errors can be reported as usual by [Err] *but also* by the [ProblemReceiver]. Callers should
    /// thus check that `problems` is empty even when the function returns [Ok].
    ///
    /// The function may return [Some] partially resolved result even if there are errors.
    fn resolve_with_problems<ProblemReceiverT>(self, problems: &mut ProblemReceiverT) -> ResolveResult<ResolvedT>
    where
        ProblemReceiverT: ProblemReceiver;

    /// Resolve one type into another.
    ///
    /// A lot like [TryFrom], except that we can accumulate potentially annotated errors.
    ///
    /// Unlike [resolve](Resolve::resolve) will fail on the first encountered error and will return
    /// an [UnavailableError] problem instead of [None].
    ///
    /// If you want all the errors use [resolve_with_problems](Resolve::resolve_with_problems)
    /// instead.
    fn resolve(self) -> Result<ResolvedT, Problem> {
        self.resolve_with_problems(&mut FailFast)?.ok_or(UnavailableError::new("resolved").into())
    }
}
