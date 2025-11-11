//
// RemoveAnnotations
//

/// Removes all [Annotations](super::super::Annotations) recursively.
pub trait RemoveAnnotations<NewT> {
    /// Removes all [Annotations](super::super::Annotations) recursively.
    fn remove_annotations(self) -> NewT;
}
