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
        // TODO: serde_yml is broken for complex keys
        let mut serializer = serde_norway::Serializer::new(writer);
        Ok(value.serialize(&mut serializer)?)

        // Ok(serde_saphyr::to_writer(&mut writer.as_fmt_write(), value)
        //     .map_err(|error| SerializeError::YAML(error.to_string()))?)
    }
}
