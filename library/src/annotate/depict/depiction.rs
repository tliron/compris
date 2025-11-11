use super::{super::traits::*, mode::*};

use {
    depiction::*,
    std::{error::*, io},
};

//
// AnnotatedDepiction
//

/// A [Depict] wrapper for an [Annotated] [Depict].
///
/// The inner [Depict] is called first and the
/// [Annotations](super::super::annotations::Annotations) next.
pub struct AnnotatedDepiction<'this, InnerT> {
    /// Inner.
    pub inner: &'this InnerT,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this, InnerT> AnnotatedDepiction<'this, InnerT> {
    /// Constructor.
    pub fn new(inner: &'this InnerT, mode: AnnotatedDepictionMode) -> Self {
        Self { inner, mode }
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

//
// ToAnnotatedDepiction
//

/// To [AnnotatedDepiction].
pub trait ToAnnotatedDepiction<'this>
where
    Self: Sized,
{
    /// To [AnnotatedDepiction].
    fn annotated_depiction(&'this self) -> AnnotatedDepiction<'this, Self>;
}

impl<'this, ErrorT> ToAnnotatedDepiction<'this> for ErrorT
where
    ErrorT: Error,
{
    fn annotated_depiction(&'this self) -> AnnotatedDepiction<'this, Self> {
        AnnotatedDepiction::new(self, AnnotatedDepictionMode::Multiline)
    }
}
