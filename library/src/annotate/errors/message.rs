use super::super::impl_dyn_annotated_error;

use {
    kutil::cli::depict::*,
    std::{error::*, fmt, io},
};

//
// AnnotatedMessageError
//

/// [Annotated](super::super::Annotated) [Error] with [String] message.
#[derive(Debug)]
pub struct AnnotatedMessageError<AnnotatedT> {
    /// Message.
    pub message: String,

    /// Annotated.
    pub annotated: AnnotatedT,
}

impl_dyn_annotated_error!(AnnotatedMessageError);

impl<AnnotatedT> AnnotatedMessageError<AnnotatedT> {
    /// Constructor.
    pub fn new(message: String) -> Self
    where
        AnnotatedT: Default,
    {
        Self { message, annotated: Default::default() }
    }
}

impl<AnnotatedT> Depict for AnnotatedMessageError<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, _context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{}", self.message)
    }
}

impl<AnnotatedT> fmt::Display for AnnotatedMessageError<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, formatter)
    }
}

impl<AnnotatedT> Error for AnnotatedMessageError<AnnotatedT> where AnnotatedT: fmt::Debug {}

impl<AnnotatedT> From<String> for AnnotatedMessageError<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl<AnnotatedT> From<&str> for AnnotatedMessageError<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}
