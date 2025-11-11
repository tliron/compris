use super::{super::annotate::*, mode::*, problem::*};

use {
    depiction::*,
    kutil::std::iter::*,
    problemo::*,
    std::{cmp::*, collections::*, io},
};

//
// AnnotatedProblemsDepiction
//

/// A [Depict] wrapper for an [Iterator] of [Problem]. Supports [DepictRef] and [Annotations].
pub struct AnnotatedProblemsDepiction<'this, InnerT>
where
    &'this InnerT: IntoIterator<Item = &'this Problem>,
{
    /// Inner.
    pub inner: &'this InnerT,

    /// Mode.
    pub mode: AnnotatedDepictionMode,

    /// Optional heading.
    pub heading: Option<String>,
}

impl<'this, InnerT> AnnotatedProblemsDepiction<'this, InnerT>
where
    &'this InnerT: IntoIterator<Item = &'this Problem>,
{
    /// Constructor.
    pub fn new(inner: &'this InnerT) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Multiline, heading: None }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }

    /// With heading.
    pub fn with_heading(mut self, heading: String) -> Self {
        self.heading = Some(heading);
        self
    }
}

impl<'this, InnerT> Depict for AnnotatedProblemsDepiction<'this, InnerT>
where
    &'this InnerT: IntoIterator<Item = &'this Problem>,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(heading) = &self.heading {
            context.theme.write_heading(writer, heading)?;
        }

        let mut table = BTreeMap::<_, Vec<_>>::default();
        for problem in self.inner {
            let source = problem.attachment_of_type::<Annotations>().and_then(|annotations| annotations.source.clone());
            match table.get_mut(&source) {
                Some(list) => list.push(problem),
                None => {
                    let mut list = Vec::default();
                    list.push(problem);
                    table.insert(source, list);
                }
            }
        }

        table.values_mut().for_each(|list| {
            list.sort_by(|a, b| {
                if let Some(a_annotations) = a.attachment_of_type::<Annotations>()
                    && let Some(a_span) = &a_annotations.span
                    && let Some(b_annotations) = b.attachment_of_type::<Annotations>()
                    && let Some(b_span) = &b_annotations.span
                {
                    a_span.start.cmp(&b_span.start)
                } else {
                    Ordering::Equal
                }
            })
        });

        let child_context = &context.child().increase_indentation();

        for ((source, list), first) in IterateWithFirst::new(table) {
            context.separate_or_indent(writer, first && self.heading.is_none())?;

            match source {
                Some(source) => context.theme.write_meta(writer, source)?,
                None => context.theme.write_meta(writer, "general")?,
            }

            for problem in list {
                context.indent_into(writer, utils::DEPICT_INTO_LIST_ITEM)?;
                write!(writer, " ")?;
                problem.annotated_depiction().with_mode(self.mode).depict(writer, child_context)?;
            }
        }

        Ok(())
    }
}

//
// ToAnnotatedProblemsDepiction
//

/// To [AnnotatedProblemsDepiction].
pub trait ToAnnotatedProblemsDepiction<'this>
where
    Self: 'this + Sized,
    &'this Self: IntoIterator<Item = &'this Problem>,
{
    /// To [AnnotatedProblemsDepiction].
    fn annotated_depiction(&'this self) -> AnnotatedProblemsDepiction<'this, Self>;
}

impl<'this, ProblemsT> ToAnnotatedProblemsDepiction<'this> for ProblemsT
where
    ProblemsT: 'this,
    &'this ProblemsT: IntoIterator<Item = &'this Problem>,
{
    fn annotated_depiction(&'this self) -> AnnotatedProblemsDepiction<'this, Self> {
        AnnotatedProblemsDepiction::new(self)
    }
}
