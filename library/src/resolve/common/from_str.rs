use super::super::{
    super::{annotate::*, errors::*, normal::*},
    errors::*,
    resolve::*,
};

use {
    problemo::*,
    std::{fmt, str::*},
};

/// Resolve a [Variant] into a [FromStr].
pub fn resolve_from_str<FromStrT, AnnotatedT, ProblemReceiverT>(
    variant: Variant<AnnotatedT>,
    problems: &mut ProblemReceiverT,
) -> ResolveResult<FromStrT>
where
    FromStrT: FromStr,
    FromStrT::Err: ToString,
    AnnotatedT: Annotated + Clone + Default,
    ProblemReceiverT: ProblemReceiver,
{
    Ok(match variant {
        Variant::Text(text) => match text.inner.parse() {
            Ok(parsed) => Some(parsed),

            Err(error) => {
                problems.give(
                    MalformedError::as_problem(tynm::type_name::<FromStrT>(), error.to_string())
                        .maybe_with(text.annotations().cloned())
                        .via(ResolveError),
                )?;
                None
            }
        },

        _ => {
            problems.give(IncompatibleVariantTypeError::as_problem_from(&variant, &["text"]).via(ResolveError))?;
            None
        }
    })
}

/// Implement [Resolve] for a [FromStr].
#[macro_export]
macro_rules! impl_resolve_from_str {
    ( $type:ident $(,)? ) => {
        impl<AnnotatedT> $crate::resolve::Resolve<$type> for $crate::normal::Variant<AnnotatedT>
        where
            AnnotatedT: $crate::annotate::Annotated + ::std::clone::Clone + ::std::default::Default,
        {
            fn resolve_with_problems<ProblemReceiverT>(
                self,
                problems: &mut ProblemReceiverT,
            ) -> $crate::resolve::ResolveResult<$type>
            where
                ProblemReceiverT: ::problemo::ProblemReceiver,
            {
                $crate::resolve::resolve_from_str(self, problems)
            }
        }
    };
}

#[allow(unused_imports)]
pub use impl_resolve_from_str;

//
// ResolveFromStr
//

/// A wrapper for a [FromStr] that implements [Resolve].
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ResolveFromStr<InnerT> {
    /// Inner.
    pub inner: InnerT,
}

impl<InnerT> ResolveFromStr<InnerT> {
    /// Constructor.
    pub fn new(inner: InnerT) -> Self {
        Self { inner }
    }
}

impl<InnerT, AnnotatedT> Resolve<ResolveFromStr<InnerT>> for Variant<AnnotatedT>
where
    InnerT: FromStr,
    InnerT::Err: ToString,
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_problems<ProblemReceiverT>(
        self,
        problems: &mut ProblemReceiverT,
    ) -> ResolveResult<ResolveFromStr<InnerT>>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        resolve_from_str(self, problems).map(|resolved| resolved.map(ResolveFromStr::new))
    }
}

impl<InnerT> AsRef<InnerT> for ResolveFromStr<InnerT> {
    fn as_ref(&self) -> &InnerT {
        &self.inner
    }
}

impl<InnerT> From<InnerT> for ResolveFromStr<InnerT> {
    fn from(inner: InnerT) -> Self {
        Self::new(inner)
    }
}

impl<InnerT> fmt::Display for ResolveFromStr<InnerT>
where
    InnerT: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}
