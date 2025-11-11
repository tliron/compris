use super::{
    super::{annotate::*, normal::*},
    mode::*,
};

use {depiction::*, std::io};

//
// AnnotatedMapDepiction
//

/// [Depict] wrapper for a [Map] with [Annotations].
pub struct AnnotatedMapDepiction<'inner, AnnotatedT> {
    /// Inner map.
    pub inner: &'inner Map<AnnotatedT>,

    /// Depiction mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'inner, AnnotatedT> AnnotatedMapDepiction<'inner, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'inner Map<AnnotatedT>) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Inline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'inner, AnnotatedT> Depict for AnnotatedMapDepiction<'inner, AnnotatedT>
where
    AnnotatedT: 'static + Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let entries: Vec<_> = self
            .inner
            .into_iter()
            .map(|(key, value)| {
                (key.annotated_depiction().with_mode(self.mode), value.annotated_depiction().with_mode(self.mode))
            })
            .collect();
        let entries = entries.iter().map(|(k, v)| (k, v));
        depict_map(entries, None, writer, context)
    }
}
