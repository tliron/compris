use super::super::format::*;

use {
    problemo::{common::*, *},
    std::error::Error,
};

string_attachment!(ReferenceAttachment);

//
// ParseProblem
//

/// Parse problem.
pub trait ParseProblem {
    /// Into a [Problem] via [ParsingError] with a [Format] attachment.
    fn into_parse_problem(self, format: Format) -> Problem;
}

impl<ErrorT> ParseProblem for ErrorT
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_parse_problem(self, format: Format) -> Problem {
        self.into_problem().via(ParsingError::new("Compris")).with(format)
    }
}

//
// ParseProblemResult
//

/// Parse problem result.
pub trait ParseProblemResult<OkT> {
    /// Into a [Problem] via [ParsingError] with a [Format] attachment.
    fn into_parse_problem(self, format: Format) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> ParseProblemResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    fn into_parse_problem(self, format: Format) -> Result<OkT, Problem> {
        self.into_problem().via(ParsingError::new("Compris")).with(format)
    }
}
