use super::super::format::*;

use {
    problemo::{common::*, *},
    std::error::Error,
};

string_attachment!(ReferenceAttachment);

//
// ParsingProblem
//

/// Parsing problem.
pub trait ParsingProblem {
    /// Into a [Problem] via [ParsingError] with a [Format] attachment.
    fn into_parsing_problem(self, format: Format) -> Problem;
}

impl<ErrorT> ParsingProblem for ErrorT
where
    ErrorT: 'static + Error + Send + Sync,
{
    fn into_parsing_problem(self, format: Format) -> Problem {
        self.into_problem().via(ParsingError::new("Compris")).with(format)
    }
}

//
// ParsingProblemResult
//

/// Parsing problem result.
pub trait ParsingProblemResult<OkT> {
    /// Into a [Problem] via [ParsingError] with a [Format] attachment.
    fn into_parsing_problem(self, format: Format) -> Result<OkT, Problem>;
}

impl<ResultT, OkT> ParsingProblemResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    #[track_caller]
    fn into_parsing_problem(self, format: Format) -> Result<OkT, Problem> {
        self.into_problem().via(ParsingError::new("Compris")).with(format)
    }
}
