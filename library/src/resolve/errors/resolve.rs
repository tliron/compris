use super::super::super::annotate::*;

use {depiction::*, problemo::*};

//
// ResolveError
//

tag_error!(ResolveError, "resolve");

//
// IntoResolveProblem
//

///
pub trait IntoResolveProblem {
    /// Creates a [Problem] with self as the context of a child problem.
    ///
    /// If the [Annotated] has [Annotations] then attaches them.
    fn into_resolve_problem<AnnotatedT>(self, annotated: &AnnotatedT) -> Problem
    where
        AnnotatedT: Annotated;
}

impl<ErrorT> IntoResolveProblem for ErrorT
where
    ErrorT: 'static + Depict + Send + Sync,
{
    fn into_resolve_problem<AnnotatedT>(self, annotated: &AnnotatedT) -> Problem
    where
        AnnotatedT: Annotated,
    {
        self.into_depict().into_problem().via(ResolveError).maybe_with(annotated.annotations().cloned())
    }
}

//
// ResolveResult
//

/// Resolve result.
pub type ResolveResult<ResolvedT> = Result<Option<ResolvedT>, Problem>;
