use super::super::{super::normal::*, *};

use {problemo::*, std::io};

impl Parser {
    /// Parses XML into a [Variant].
    pub fn read_xml<ReadT, AnnotatedT>(&self, _reader: &mut ReadT) -> Result<Variant<AnnotatedT>, Problem>
    where
        ReadT: io::Read,
    {
        todo!()
    }
}
