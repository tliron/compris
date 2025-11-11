use super::super::annotate::*;

use {
    depiction::*,
    std::{cmp::*, fmt, hash::*, io},
};

//
// Null
//

/// Normal null variant.
///
/// [Annotations], if present, are *ignored* for the purposes of comparison and hashing.
#[derive(Clone, Debug)]
pub struct Null<AnnotatedT> {
    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_annotated!(Null);

impl<AnnotatedT, NewAnnotatedT> IntoAnnotated<Null<NewAnnotatedT>> for Null<AnnotatedT>
where
    AnnotatedT: Annotated,
    NewAnnotatedT: Annotated + Default,
{
    fn into_annotated(self) -> Null<NewAnnotatedT> {
        if AnnotatedT::can_have_annotations()
            && NewAnnotatedT::can_have_annotations()
            && let Some(annotations) = self.annotated.annotations()
        {
            Null::default().with_annotations(annotations.clone())
        } else {
            Default::default()
        }
    }
}

impl<AnnotatedT> RemoveAnnotations<Null<WithoutAnnotations>> for Null<AnnotatedT> {
    fn remove_annotations(self) -> Null<WithoutAnnotations> {
        Default::default()
    }
}

impl<AnnotatedT> Depict for Null<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_symbol(writer, "Null")
    }
}

impl<AnnotatedT> Default for Null<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn default() -> Self {
        Self { annotated: Default::default() }
    }
}

impl<AnnotatedT> fmt::Display for Null<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt("Null", formatter)
    }
}

// Basics

impl<AnnotatedT> PartialEq for Null<AnnotatedT> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<AnnotatedT> Eq for Null<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Null<AnnotatedT> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<AnnotatedT> Ord for Null<AnnotatedT> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<AnnotatedT> Hash for Null<AnnotatedT> {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        ().hash(state)
    }
}

// Conversions

impl<AnnotatedT> From<()> for Null<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl<AnnotatedT> From<Null<AnnotatedT>> for () {
    fn from(_: Null<AnnotatedT>) -> Self {
        ()
    }
}
