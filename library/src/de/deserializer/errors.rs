use problemo::{common::*, *};

/// [Problem] for [IncompatibleError] with an attachment via [DeserializationError].
#[track_caller]
pub fn incompatible_deserialization_problem<AttachmentT>(message: &str, attachment: AttachmentT) -> SerdeProblem
where
    AttachmentT: 'static + Send + Sync,
{
    IncompatibleError::as_problem(message).with(attachment).via(DeserializationError::new("serde")).into()
}

/// [Problem] for [MissingError] via [DeserializationError].
#[track_caller]
pub fn missing_deserialization_problem(message: &str) -> SerdeProblem {
    MissingError::as_problem(message).via(DeserializationError::new("serde")).into()
}

/// [Problem] for [UnsupportedError] via [DeserializationError].
#[track_caller]
pub fn unsupported_deserialization_problem(message: &str) -> SerdeProblem {
    UnsupportedError::as_problem(message).via(DeserializationError::new("serde")).into()
}
