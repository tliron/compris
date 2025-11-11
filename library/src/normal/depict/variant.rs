use super::super::{super::annotate::*, variant::*};

use {depiction::*, std::io};

//
// AnnotatedDepictVariant
//

/// [Depict] wrapper for a [Variant] with [Annotations].
pub struct AnnotatedDepictVariant<'this, AnnotatedT> {
    /// Inner.
    pub inner: &'this Variant<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, AnnotatedT> AnnotatedDepictVariant<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'this Variant<AnnotatedT>, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
    }
}

impl<'this, AnnotatedT> Depict for AnnotatedDepictVariant<'this, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.inner {
            Variant::List(list) => list.annotated_depict(self.mode).depict(writer, context),
            Variant::Map(map) => map.annotated_depict(self.mode).depict(writer, context),
            _ => AnnotatedDepiction::new(self.inner, self.mode).depict(writer, context),
        }
    }
}
