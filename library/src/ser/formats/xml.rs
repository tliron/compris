use super::super::{super::format::*, errors::*, serializer::*};

use {
    problemo::{common::*, *},
    serde::Serialize,
    std::io,
};

impl Serializer {
    // Broken :(
    // Write out own using https://docs.rs/quick-xml/latest/quick_xml/
    /// Serializes the provided value to the writer as XML.
    pub fn write_xml<WriteT, SerializableT>(&self, value: &SerializableT, writer: &mut WriteT) -> Result<(), Problem>
    where
        WriteT: io::Write,
        SerializableT: Serialize + Sized,
    {
        // Note: serde_xml_rs requires value to be Sized
        serde_xml_rs::to_writer(writer.by_ref(), value).via(SerializationError::new("serde")).with(Format::XML)?;

        if self.pretty { Self::write_newline(writer).into_low_level_serialization_problem(Format::XML) } else { Ok(()) }
    }
}
