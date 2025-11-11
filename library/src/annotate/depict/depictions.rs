use super::{super::traits::*, depiction::*, mode::*};

use {
    depiction::*,
    kutil::std::iter::*,
    std::{cmp::*, collections::*, error::*, io},
};

//
// AnnotatedDepictions
//

/// A [Depict] wrapper for an [Iterator] of [Annotated] [Depict].
pub struct AnnotatedDepictions<'this, InnerT, ItemT>
where
    &'this InnerT: IntoIterator<Item = &'this ItemT>,
    ItemT: 'this,
{
    /// Inner.
    pub inner: &'this InnerT,

    /// Mode.
    pub mode: AnnotatedDepictionMode,

    /// Optional heading.
    pub heading: Option<String>,
}

impl<'this, InnerT, ItemT> AnnotatedDepictions<'this, InnerT, ItemT>
where
    &'this InnerT: IntoIterator<Item = &'this ItemT>,
    ItemT: 'this,
{
    /// Constructor.
    pub fn new(inner: &'this InnerT, mode: AnnotatedDepictionMode, heading: Option<String>) -> Self {
        Self { inner, mode, heading }
    }
}

impl<'this, InnerT, ItemT> Depict for AnnotatedDepictions<'this, InnerT, ItemT>
where
    &'this InnerT: IntoIterator<Item = &'this ItemT>,
    ItemT: 'this + Annotated + Depict,
{
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if let Some(heading) = &self.heading {
            context.theme.write_heading(writer, heading)?;
        }

        let mut table = BTreeMap::<_, Vec<_>>::default();
        for item in self.inner {
            let source = item.annotations().and_then(|annotations| annotations.source.clone());
            match table.get_mut(&source) {
                Some(list) => list.push(item),
                None => {
                    let mut list = Vec::default();
                    list.push(item);
                    table.insert(source, list);
                }
            }
        }

        table.values_mut().for_each(|list| {
            list.sort_by(|a, b| {
                if let Some(a_annotations) = a.annotations()
                    && let Some(a_span) = &a_annotations.span
                    && let Some(b_annotations) = b.annotations()
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

            for item in list {
                context.indent_into(writer, utils::DEPICT_INTO_LIST_ITEM)?;
                write!(writer, " ")?;
                AnnotatedDepiction::new(item, self.mode).depict(writer, child_context)?;
            }
        }

        Ok(())
    }
}

//
// ToAnnotatedDepictions
//

/// To [AnnotatedDepictions].
pub trait ToAnnotatedDepictions<'this, ItemT>
where
    Self: 'this + Sized,
    &'this Self: IntoIterator<Item = &'this ItemT>,
    ItemT: 'this,
{
    /// To [AnnotatedDepictions].
    fn annotated_depictions(&'this self, heading: Option<String>) -> AnnotatedDepictions<'this, Self, ItemT>;
}

impl<'this, ErrorIterableT, ErrorT> ToAnnotatedDepictions<'this, ErrorT> for ErrorIterableT
where
    ErrorIterableT: 'this,
    &'this ErrorIterableT: IntoIterator<Item = &'this ErrorT>,
    ErrorT: 'this + Error,
{
    fn annotated_depictions(&'this self, heading: Option<String>) -> AnnotatedDepictions<'this, Self, ErrorT> {
        AnnotatedDepictions::new(self, AnnotatedDepictionMode::Multiline, heading)
    }
}
