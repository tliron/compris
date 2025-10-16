use super::{
    super::{annotate::*, normal::*, parse::*},
    errors::*,
    resolve::*,
};

use {kutil::std::error::*, std::io};

impl Parser {
    /// Resolve the parsed [Variant] into another type.
    pub fn resolve<ResolvedT, ReadT, AnnotatedT, ErrorReceiverT>(
        &self,
        reader: &mut ReadT,
        errors: &mut ErrorReceiverT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        let variant = self.parse_reader(reader).expect("parse");
        variant.resolve_with_errors(errors)
    }

    /// Resolve the parsed [Variant] into another type.
    pub fn resolve_string<ResolvedT, AnnotatedT, ErrorReceiverT>(
        &self,
        string: &str,
        errors: &mut ErrorReceiverT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        let variant = self.parse_string(string).expect("parse");
        variant.resolve_with_errors(errors)
    }

    /// Resolve the parsed [Variant] into another type while failing on the first encountered
    /// error.
    ///
    /// Uses [FailFastErrorReceiver].
    pub fn resolve_fail_fast<ResolvedT, ReadT, AnnotatedT>(
        &self,
        reader: &mut ReadT,
    ) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        ReadT: io::Read,
        AnnotatedT: Annotated + Clone + Default,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        self.resolve(reader, &mut FailFastErrorReceiver)
    }

    /// Resolve the parsed [Variant] into another type while failing on the first encountered
    /// error.
    ///
    /// Uses [FailFastErrorReceiver].
    pub fn resolve_string_fail_fast<ResolvedT, AnnotatedT>(&self, string: &str) -> ResolveResult<ResolvedT, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        Variant<AnnotatedT>: Resolve<ResolvedT, AnnotatedT>,
    {
        self.resolve_string(string, &mut FailFastErrorReceiver)
    }
}
