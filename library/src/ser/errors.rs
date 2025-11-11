use super::super::format::*;

use {
    problemo::{common::*, *},
    std::error::Error,
};

//
// SerializationProblem
//

/// Serialization problem.
pub trait SerializationProblem {
    /// Into a [Problem] via [SerializationError] with [Format] attachment.
    fn into_serialization_problem(self, format: Format) -> Problem;
}

impl<ErrorT> SerializationProblem for ErrorT
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_serialization_problem(self, format: Format) -> Problem {
        self.into_problem().via(SerializationError::new("serde")).with(format)
    }
}

//
// SerializationProblemResult
//

/// Serialization problem result.
pub trait SerializationProblemResult<OkT> {
    /// Into a [Problem] via [LowLevelError] and [SerializationError] with [Format] attachment.
    fn into_low_level_serialization_problem(self, format: Format) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> SerializationProblemResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn into_low_level_serialization_problem(self, format: Format) -> Result<OkT, Problem> {
        self.into_problem().via(LowLevelError).via(SerializationError::new("serde")).with(format)
    }
}
