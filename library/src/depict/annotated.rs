use super::{super::annotate::*, mode::*};

use {depiction::*, std::io};

//
// AnnotatedDepiction
//

/// A [Depict] wrapper for an [Annotated] [Depict].
pub struct AnnotatedDepiction<'inner, InnerT> {
    /// Inner.
    pub inner: &'inner InnerT,

    /// Depiction mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'inner, InnerT> AnnotatedDepiction<'inner, InnerT> {
    /// Constructor.
    pub fn new(inner: &'inner InnerT) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Multiline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'inner, InnerT> Depict for AnnotatedDepiction<'inner, InnerT>
where
    InnerT: DynAnnotated + DynDepict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(annotations) = self.inner.dyn_annotations() {
            match self.mode {
                AnnotatedDepictionMode::Inline => {
                    self.inner.dyn_depict(Box::new(writer), context)?;
                    if annotations.has_depiction(DepictionFormat::Compact) {
                        annotations.depict(writer, &context.clone().with_format(DepictionFormat::Compact))?;
                    }
                }

                AnnotatedDepictionMode::Multiline => {
                    if annotations.has_depiction(DepictionFormat::Optimized) {
                        annotations.depict(writer, &context.clone().with_format(DepictionFormat::Optimized))?;
                        context.indent(writer)?;
                    } else {
                        context.separate(writer)?;
                    }
                    self.inner.dyn_depict(Box::new(writer), context)?;
                }
            }
        } else {
            context.separate(writer)?;
            self.inner.dyn_depict(Box::new(writer), context)?;
        }

        Ok(())
    }
}
