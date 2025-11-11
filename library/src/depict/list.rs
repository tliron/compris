use super::{
    super::{annotate::*, normal::*},
    mode::*,
};

use {depiction::*, std::io};

//
// AnnotatedListDepiction
//

/// [Depict] wrapper for a [List] with [Annotations].
pub struct AnnotatedListDepiction<'inner, AnnotatedT> {
    /// Inner list.
    pub inner: &'inner List<AnnotatedT>,

    /// Depiction mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'inner, AnnotatedT> AnnotatedListDepiction<'inner, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'inner List<AnnotatedT>) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Inline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'inner, AnnotatedT> Depict for AnnotatedListDepiction<'inner, AnnotatedT>
where
    AnnotatedT: 'static + Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let list: Vec<_> =
            self.inner.into_iter().map(|value| value.annotated_depiction().with_mode(self.mode)).collect();
        depict_list(&list, None, writer, context)
    }
}
