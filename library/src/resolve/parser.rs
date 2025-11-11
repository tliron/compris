use super::{
    super::{annotate::*, normal::*, parse::*},
    errors::*,
    resolve::*,
};

use {problemo::*, std::io};

impl Parser {
    /// Resolve the parsed [Variant] into another type.
    pub fn resolve<ResolvedT, ReadT, AnnotatedT, ProblemReceiverT>(
        &self,
        reader: &mut ReadT,
        errors: &mut ProblemReceiverT,
    ) -> ResolveResult<ResolvedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        ProblemReceiverT: ProblemReceiver,
        Variant<AnnotatedT>: Resolve<ResolvedT>,
    {
        let variant = self.parse_reader(reader).expect("parse");
        variant.resolve_with_problems(errors)
    }

    /// Resolve the parsed [Variant] into another type.
    pub fn resolve_string<ResolvedT, AnnotatedT, ProblemReceiverT>(
        &self,
        string: &str,
        errors: &mut ProblemReceiverT,
    ) -> ResolveResult<ResolvedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ProblemReceiverT: ProblemReceiver,
        Variant<AnnotatedT>: Resolve<ResolvedT>,
    {
        let variant = self.parse_string(string).expect("parse");
        variant.resolve_with_problems(errors)
    }

    /// Resolve the parsed [Variant] into another type while failing on the first encountered
    /// error.
    ///
    /// Uses [FailFastErrorReceiver].
    pub fn resolve_fail_fast<ResolvedT, ReadT, AnnotatedT>(&self, reader: &mut ReadT) -> ResolveResult<ResolvedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        Variant<AnnotatedT>: Resolve<ResolvedT>,
    {
        self.resolve(reader, &mut FailFast)
    }

    /// Resolve the parsed [Variant] into another type while failing on the first encountered
    /// error.
    ///
    /// Uses [FailFastErrorReceiver].
    pub fn resolve_string_fail_fast<ResolvedT, AnnotatedT>(&self, string: &str) -> ResolveResult<ResolvedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        Variant<AnnotatedT>: Resolve<ResolvedT>,
    {
        self.resolve_string(string, &mut FailFast)
    }
}
