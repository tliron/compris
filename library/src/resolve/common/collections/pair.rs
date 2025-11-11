use super::super::super::{super::normal::*, errors::*, resolve::*};

use problemo::*;

// Resolve two values at once
// Useful for key-value pairs of maps

impl<FirstT, SecondT, AnnotatedT> Resolve<(FirstT, SecondT)> for (Variant<AnnotatedT>, Variant<AnnotatedT>)
where
    Variant<AnnotatedT>: Resolve<FirstT> + Resolve<SecondT>,
{
    fn resolve_with_problems<ProblemReceiverT>(self, errors: &mut ProblemReceiverT) -> ResolveResult<(FirstT, SecondT)>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        let first = self.0.resolve_with_problems(errors)?;
        let second = self.1.resolve_with_problems(errors)?;

        Ok(
            if let Some(first) = first
                && let Some(second) = second
            {
                Some((first, second))
            } else {
                None
            },
        )
    }
}
