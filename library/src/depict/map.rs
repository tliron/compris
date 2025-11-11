use super::{
    super::{annotate::*, normal::*},
    mode::*,
};

use {depiction::*, std::io};

//
// AnnotatedMapDepiction
//

/// [Depict] wrapper for a [Map] with [Annotations].
pub struct AnnotatedMapDepiction<'this, AnnotatedT> {
    /// Inner.
    pub inner: &'this Map<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, AnnotatedT> AnnotatedMapDepiction<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'this Map<AnnotatedT>) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Inline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'this, AnnotatedT> Depict for AnnotatedMapDepiction<'this, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> = self
            .inner
            .into_iter()
            .map(|(key, value)| {
                (key.annotated_depict().with_mode(self.mode), value.annotated_depict().with_mode(self.mode))
            })
            .collect();
        let vector: Vec<_> = vector.iter().map(|(k, v)| (k, v)).collect();
        utils::depict_map(vector.into_iter(), None, writer, context)?;
        Ok(())
    }
}
