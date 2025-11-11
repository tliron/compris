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
pub struct AnnotatedVariantDepiction<'this, AnnotatedT> {
    /// Inner.
    pub inner: &'this Variant<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, AnnotatedT> AnnotatedVariantDepiction<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'this Variant<AnnotatedT>) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Inline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'this, AnnotatedT> Depict for AnnotatedVariantDepiction<'this, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.inner {
            Variant::List(list) => list.annotated_depict().with_mode(self.mode).depict(writer, context),
            Variant::Map(map) => map.annotated_depict().with_mode(self.mode).depict(writer, context),
            _ => AnnotatedDepiction::new(self.inner).with_mode(self.mode).depict(writer, context),
        }
    }
}
