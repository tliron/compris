use super::{super::annotate::*, mode::*};

use {depiction::*, std::io};

//
// AnnotatedDepiction
//

/// A [Depict] wrapper for an [Annotated] [Depict].
pub struct AnnotatedDepiction<'this, InnerT> {
    /// Inner.
    pub inner: &'this InnerT,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, InnerT> AnnotatedDepiction<'this, InnerT> {
    /// Constructor.
    pub fn new(inner: &'this InnerT) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Multiline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'this, InnerT> Depict for AnnotatedDepiction<'this, InnerT>
where
    InnerT: Annotated + Depict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(annotations) = self.inner.annotations() {
            match self.mode {
                AnnotatedDepictionMode::Inline => {
                    self.inner.depict(writer, context)?;
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
                    self.inner.depict(writer, context)?;
                }
            }
        } else {
            context.separate(writer)?;
            self.inner.depict(writer, context)?;
        }

        Ok(())
    }
}
