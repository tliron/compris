use super::super::format::*;

use {
    problemo::{common::*, *},
    std::error::Error,
};

//
// SerializeProblem
//

/// Parse problem.
pub trait SerializeProblem {
    /// Into a [Problem] via [ParseError] with [Format] attachment.
    fn into_serialize_problem(self, format: Format) -> Problem;
}

impl<ErrorT> SerializeProblem for ErrorT
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_serialize_problem(self, format: Format) -> Problem {
        self.into_problem().via(SerializeError::new("serde")).with(format)
    }
}

//
// SerializeProblemResult
//

/// Serialize problem result.
pub trait SerializeProblemResult<OkT> {
    /// Into a [Problem] via [LowLevelError] and [SerializeError] with [Format] attachment.
    fn into_low_level_serialize_problem(self, format: Format) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> SerializeProblemResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn into_low_level_serialize_problem(self, format: Format) -> Result<OkT, Problem> {
        self.into_problem().via(LowLevelError).via(SerializeError::new("serde")).with(format)
    }
}
