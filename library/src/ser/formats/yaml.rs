use super::super::{errors::*, serializer::*};

use {serde::Serialize, std::io};

impl Serializer {
    /// Serializes the provided value to the writer as YAML.
    pub fn write_yaml<WriteT, SerializableT>(
        &self,
        value: &SerializableT,
        writer: &mut WriteT,
    ) -> Result<(), SerializeError>
    where
        WriteT: io::Write,
        SerializableT: Serialize,
    {
        // let mut serializer = serde_norway::Serializer::new(writer);
        // Ok(value.serialize(&mut serializer)?)

        Ok(serde_saphyr::to_io_writer(writer, value)?)
    }
}
