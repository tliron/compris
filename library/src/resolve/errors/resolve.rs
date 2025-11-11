use super::super::super::annotate::*;

use problemo::*;

//
// ResolveError
//

tag_error!(ResolveError, "resolve");

//
// ResolveResult
//

/// Resolve result.
pub type ResolveResult<ResolvedT> = Result<Option<ResolvedT>, Problem>;

//
// IntoResolveProblem
//

/// Via [ResolveError] maybe with [Annotations].
pub trait IntoResolveProblem {
    /// Via [ResolveError] maybe with [Annotations].
    fn into_resolve_problem<AnnotatedT>(self, annotated: &AnnotatedT) -> Problem
    where
        AnnotatedT: Annotated;
}

impl IntoResolveProblem for Problem {
    fn into_resolve_problem<AnnotatedT>(self, annotated: &AnnotatedT) -> Problem
    where
        AnnotatedT: Annotated,
    {
        self.via(ResolveError).maybe_with(annotated.annotations().cloned())
    }
}
