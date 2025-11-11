use super::{super::annotate::*, mode::*};

use {depiction::*, problemo::*, std::io};

//
// AnnotatedProblemDepiction
//

/// A [Depict] wrapper for a [Problem]. Supports [DepictRef] and [Annotations].
pub struct AnnotatedProblemDepiction<'this> {
    /// Problem.
    pub problem: &'this Problem,

    /// Mode.
    pub mode: AnnotatedDepictionMode,
}

impl<'this> AnnotatedProblemDepiction<'this> {
    /// Constructor.
    pub fn new(problem: &'this Problem) -> Self {
        Self { problem, mode: AnnotatedDepictionMode::Multiline }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }
}

impl<'this> Depict for AnnotatedProblemDepiction<'this> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(annotations) = self.problem.attachment_of_type::<Annotations>() {
            match self.mode {
                AnnotatedDepictionMode::Inline => {
                    self.problem.depict(writer, context)?;
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
                    self.problem.depict(writer, context)?;
                }
            }
        } else {
            context.separate(writer)?;
            self.problem.depict(writer, context)?;
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
