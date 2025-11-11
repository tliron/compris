use super::{super::normal::*, segment::*};

//
// PathNode
//

/// [Path](super::Path) node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathNode<'context, AnnotatedT> {
    /// Variant.
    pub variant: &'context Variant<AnnotatedT>,

    /// Segment.
    pub segment: Option<PathSegment<&'context Variant<AnnotatedT>>>,
}

impl<'context, AnnotatedT> PathNode<'context, AnnotatedT> {
    /// Constructor.
    pub fn new(
        variant: &'context Variant<AnnotatedT>,
        segment: Option<PathSegment<&'context Variant<AnnotatedT>>>,
    ) -> Self {
        Self { variant, segment }
    }
}
