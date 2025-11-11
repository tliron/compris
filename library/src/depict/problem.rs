use super::{super::annotate::*, mode::*};

use {depiction::*, problemo::*, std::io};

//
// AnnotatedProblemDepiction
//

/// A [Depict] wrapper for a [Problem]. Supports [DepictRef] and [Annotations].
pub struct AnnotatedProblemDepiction<'inner> {
    /// Inner problem.
    pub inner: &'inner Problem,

    /// Depiction mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'inner> AnnotatedProblemDepiction<'inner> {
    /// Constructor.
    pub fn new(inner: &'inner Problem) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Multiline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'inner> Depict for AnnotatedProblemDepiction<'inner> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(annotations) = self.inner.attachment_of_type::<Annotations>() {
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

//
// ToAnnotatedProblemDepiction
//

/// To [AnnotatedProblemDepiction].
pub trait ToAnnotatedProblemDepiction<'this> {
    /// To [AnnotatedProblemDepiction].
    fn annotated_depiction(&'this self) -> AnnotatedProblemDepiction<'this>;
}

impl<'this> ToAnnotatedProblemDepiction<'this> for Problem {
    fn annotated_depiction(&'this self) -> AnnotatedProblemDepiction<'this> {
        AnnotatedProblemDepiction::new(self)
    }
}
