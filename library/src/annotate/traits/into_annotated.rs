/// Into different [Annotated](super::super::Annotations) implementation.
pub trait IntoAnnotated<NewT> {
    /// Into different [Annotated](super::super::Annotations) implementation.
    fn into_annotated(self) -> NewT;
}
