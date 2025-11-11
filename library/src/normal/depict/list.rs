use super::{
    super::{super::annotate::*, list::*},
    variant::*,
};

use {depiction::*, std::io};

//
// AnnotatedDepictList
//

/// [Depict] wrapper for a [List] with [Annotations].
pub struct AnnotatedDepictList<'this, AnnotatedT> {
    /// Inner.
    pub inner: &'this List<AnnotatedT>,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, AnnotatedT> AnnotatedDepictList<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(inner: &'this List<AnnotatedT>, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
    }
}

impl<'this, AnnotatedT> Depict for AnnotatedDepictList<'this, AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let vector: Vec<_> =
            self.inner.into_iter().map(|value| AnnotatedDepictVariant::new(value, self.mode)).collect();
        utils::depict_list(vector.iter(), None, writer, context)
    }
}
