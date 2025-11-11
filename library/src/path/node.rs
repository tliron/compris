use super::{super::normal::*, segment::*};

//
// PathNode
//

/// [Path](super::Path) node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathNode<'this, AnnotatedT> {
    /// Variant.
    pub variant: &'this Variant<AnnotatedT>,

    /// Segment.
    pub segment: Option<PathSegment<&'this Variant<AnnotatedT>>>,
}

impl<'this, AnnotatedT> PathNode<'this, AnnotatedT> {
    /// Constructor.
    pub fn new(variant: &'this Variant<AnnotatedT>, segment: Option<PathSegment<&'this Variant<AnnotatedT>>>) -> Self {
        Self { variant, segment }
    }
}
