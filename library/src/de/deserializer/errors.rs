use problemo::{common::*, *};

/// [Problem] for [IncompatibleError] with an attachment via [DeserializationError].
pub fn incompatible_deserialization_problem<AttachmentT>(message: &str, attachment: AttachmentT) -> SerdeProblem
where
    AttachmentT: 'static + Send + Sync,
{
    IncompatibleError::as_problem(message).with(attachment).via(DeserializationError::new("serde")).into()
}

/// [Problem] for [MissingError] via [DeserializationError].
pub fn missing_deserialization_problem(message: &str) -> SerdeProblem {
    MissingError::as_problem(message).via(DeserializationError::new("serde")).into()
}

/// [Problem] for [UnsupportedError] via [DeserializationError].
pub fn unsupported_deserialization_problem(message: &str) -> SerdeProblem {
    UnsupportedError::as_problem(message).via(DeserializationError::new("serde")).into()
}
