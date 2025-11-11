use super::{super::annotate::*, mode::*, problem::*};

use {
    depiction::*,
    kutil::std::{immutable::*, iter::*},
    problemo::*,
    std::{cmp::*, collections::*, io},
};

//
// AnnotatedProblemsDepiction
//

/// A [Depict] wrapper for an [Iterator] of [Problem]. Supports [DepictRef] and [Annotations].
pub struct AnnotatedProblemsDepiction<'inner, InnerT>
where
    &'inner InnerT: IntoIterator<Item = &'inner Problem>,
{
    /// Inner iterator.
    pub inner: &'inner InnerT,

    /// Depiction mode.
    pub mode: AnnotatedDepictionMode,

    /// Optional heading.
    pub heading: Option<String>,
}

impl<'inner, InnerT> AnnotatedProblemsDepiction<'inner, InnerT>
where
    &'inner InnerT: IntoIterator<Item = &'inner Problem>,
{
    /// Constructor.
    pub fn new(inner: &'inner InnerT) -> Self {
        Self { inner, mode: AnnotatedDepictionMode::Multiline, heading: None }
    }

    /// With mode.
    pub fn with_mode(mut self, mode: AnnotatedDepictionMode) -> Self {
        self.mode = mode;
        self
    }

    /// With heading.
    pub fn with_heading<HeadingT>(mut self, heading: HeadingT) -> Self
    where
        HeadingT: ToString,
    {
        self.heading = Some(heading.to_string());
        self
    }
}

impl<'inner, InnerT> Depict for AnnotatedProblemsDepiction<'inner, InnerT>
where
    &'inner InnerT: IntoIterator<Item = &'inner Problem>,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(heading) = &self.heading {
            context.theme.write_heading(writer, heading)?;
        }

        let child_context = context.child().increase_indentation();

        for ((source, list), first) in IterateWithFirst::new(problems_table(self.inner)) {
            context.separate_or_indent(writer, first && self.heading.is_none())?;

            match source {
                Some(source) => context.theme.write_meta(writer, source)?,
                None => context.theme.write_meta(writer, "general")?,
            }

            for problem in list {
                context.indent_into(writer, DEPICT_INTO_LIST_ITEM)?;
                write!(writer, " ")?;
                problem.annotated_depiction().with_mode(self.mode).depict(writer, &child_context)?;
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

// Utils

fn problems_table<'problems, ProblemsT>(problems: ProblemsT) -> BTreeMap<Option<ByteString>, Vec<&'problems Problem>>
where
    ProblemsT: IntoIterator<Item = &'problems Problem>,
{
    let mut table = BTreeMap::<_, Vec<_>>::default();
    for problem in iter_unique_problem_refs(problems) {
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

    table
}
