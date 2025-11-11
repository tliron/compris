use super::{
    super::{annotate::*, normal::*},
    annotated::*,
    mode::*,
};

use {depiction::*, std::io};

//
// AnnotatedVariantDepiction
//

/// [Depict] wrapper for a [Variant] with [Annotations].
pub struct AnnotatedVariantDepiction<'inner, AnnotatedT> {
    /// Inner variant.
    pub inner: &'inner Variant<AnnotatedT>,

    /// Depiction mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'inner, AnnotatedT> AnnotatedVariantDepiction<'inner, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'inner Variant<AnnotatedT>) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Inline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'inner, AnnotatedT> Depict for AnnotatedVariantDepiction<'inner, AnnotatedT>
where
    AnnotatedT: 'static + Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.inner {
            Variant::List(list) => list.annotated_depiction().with_mode(self.mode).depict(writer, context),
            Variant::Map(map) => map.annotated_depiction().with_mode(self.mode).depict(writer, context),
            _ => AnnotatedDepiction::new(self.inner).with_mode(self.mode).depict(writer, context),
        }
    }
}
