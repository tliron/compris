use super::super::{super::format::*, errors::*, serializer::*};

use {
    problemo::{common::*, *},
    serde::Serialize,
    std::io,
};

impl Serializer {
    /// Serializes the provided value to the writer as MessagePack.
    ///
    /// Is affected by [Serializer::base64](super::super::Serializer::base64).
    pub fn write_message_pack<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), Problem>
    where
        WriteT: io::Write,
        SerializableT: Serialize + ?Sized,
    {
        fn write<WriteT, SerializableT>(value: &SerializableT, writer: &mut WriteT) -> Result<(), Problem>
        where
            WriteT: io::Write,
            SerializableT: Serialize + ?Sized,
        {
            rmp_serde::encode::write(writer, value).via(SerializationError::new("serde")).with(Format::MessagePack)
        }

        if self.base64 {
            write(value, &mut Self::base64_writer(writer)).into_low_level_serialization_problem(Format::MessagePack)?;
        } else {
            write(value, writer).into_low_level_serialization_problem(Format::MessagePack)?;
        }

        if self.pretty {
            Self::write_newline(writer).into_low_level_serialization_problem(Format::MessagePack)
        } else {
            Ok(())
        }
    }
}
