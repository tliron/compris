use problemo::{common::*, *};

/// [Problem] for [IncompatibleError] with an attachment via [DeserializeError].
pub fn incompatible_error<AttachmentT>(message: &str, attachment: AttachmentT) -> SerdeProblem
where
    AttachmentT: 'static + Send + Sync,
{
    IncompatibleError::new(message).into_problem().with(attachment).via(DeserializeError::new("serde")).into()
}
