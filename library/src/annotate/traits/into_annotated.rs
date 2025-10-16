/// Into different [Annotated] implementation.
pub trait IntoAnnotated<NewT> {
    /// Into different [Annotated] implementation.
    fn into_annotated(self) -> NewT;
}
