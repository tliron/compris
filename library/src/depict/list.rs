use super::{
    super::{annotate::*, normal::*},
    mode::*,
};

use {depiction::*, std::io};

//
// AnnotatedListDepiction
//

/// [Depict] wrapper for a [List] with [Annotations].
pub struct AnnotatedListDepiction<'this, AnnotatedT> {
    /// Inner.
    pub inner: &'this List<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, AnnotatedT> AnnotatedListDepiction<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'this List<AnnotatedT>) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Inline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'this, AnnotatedT> Depict for AnnotatedListDepiction<'this, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> =
            self.inner.into_iter().map(|value| value.annotated_depict().with_mode(self.mode)).collect();
        utils::depict_list(vector.iter(), None, writer, context)
    }
}
