use super::{super::normal::*, node::*, representation::*, segment::*};

use {
    depiction::*,
    kutil::std::iter::*,
    std::{
        fmt::{self, Write},
        io, ptr,
    },
};

//
// Path
//

/// Path between two [Variant] nodes.
///
/// Because this type contains references to the variants, it shares their lifetime. For a version of
/// [Path] that does not keep the references see [PathRepresentation].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path<'context, AnnotatedT> {
    /// Path nodes.
    pub nodes: Vec<PathNode<'context, AnnotatedT>>,
}

impl<'context, AnnotatedT> Path<'context, AnnotatedT> {
    /// Find the path from an ancestor to a descendent, if it exists.
    ///
    /// Paths will include the endpoints. In the case of the route from oneself to oneself, it will
    /// contain just oneself (single endpoint).
    ///
    /// Important: For our purposes here, the identities of the provided variants are the
    /// *pointers* represented by the references. Thus a clone of a variant or an otherwise equal
    /// variant will *not* be considered identical.
    pub fn find(ancestor: &'context Variant<AnnotatedT>, descendent: &'context Variant<AnnotatedT>) -> Option<Self>
    where
        AnnotatedT: Default,
    {
        if ptr::eq(descendent, ancestor) {
            let mut path = Path::default();
            path.push(ancestor);
            return Some(path);
        }

        match ancestor {
            Variant::List(list) => {
                for (index, child) in list.inner.iter().enumerate() {
                    if let Some(child_route) = Self::find(child, descendent) {
                        let mut path = Path::default();
                        path.push_list_index(ancestor, index);
                        path.extend(child_route);
                        return Some(path);
                    }
                }
            }

            Variant::Map(map) => {
                for (key, child) in &map.inner {
                    // The descendent we are looking for might be this key
                    if ptr::eq(descendent, key) {
                        let mut path = Path::default();
                        path.push_map_key(ancestor, key);
                        return Some(path);
                    }

                    if let Some(child_path) = Self::find(child, descendent) {
                        let mut path = Path::default();
                        path.push_map_key(ancestor, key);
                        path.extend(child_path);
                        return Some(path);
                    }
                }
            }

            _ => {}
        }

        None
    }

    /// Push a new path node.
    pub fn push(&mut self, variant: &'context Variant<AnnotatedT>) {
        self.nodes.push(PathNode::new(variant, None))
    }

    /// Push a new list index path node.
    pub fn push_list_index(&mut self, variant: &'context Variant<AnnotatedT>, index: usize) {
        self.nodes.push(PathNode::new(variant, Some(PathSegment::ListIndex(index))))
    }

    /// Push a new map key path node.
    pub fn push_map_key(&mut self, variant: &'context Variant<AnnotatedT>, key: &'context Variant<AnnotatedT>) {
        self.nodes.push(PathNode::new(variant, Some(PathSegment::MapKey(key))))
    }

    /// Extend this path with another path.
    pub fn extend(&mut self, other: Path<'context, AnnotatedT>) {
        self.nodes.extend(other.nodes);
    }

    /// Into [PathRepresentation].
    pub fn into_representation(self) -> PathRepresentation {
        PathRepresentation {
            segments: self
                .nodes
                .into_iter()
                .filter_map(|node| node.segment.map(|segment| segment.to_string_keys()))
                .collect(),
        }
    }
}

impl<'context, AnnotatedT> Depict for Path<'context, AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for (node, first) in IterateWithFirst::new(&self.nodes) {
            if let Some(segment) = &node.segment {
                if !first && matches!(segment, PathSegment::MapKey(_)) {
                    context.theme.write_delimiter(writer, '.')?;
                }

                segment.depict(writer, context)?;
            }
        }

        Ok(())
    }
}

impl<'context, AnnotatedT> fmt::Display for Path<'context, AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for (node, first) in IterateWithFirst::new(&self.nodes) {
            if let Some(segment) = &node.segment {
                if !first && matches!(segment, PathSegment::MapKey(_)) {
                    formatter.write_char('.')?;
                }

                fmt::Display::fmt(segment, formatter)?;
            }
        }

        Ok(())
    }
}
