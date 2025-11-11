use super::super::{annotate::*, parse};

use {problemo::*, serde::de, std::io};

impl parse::Parser {
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_reader<ReadT, DeserializedT, AnnotatedT>(
        &mut self,
        reader: &mut ReadT,
    ) -> Result<DeserializedT, Problem>
    where
        ReadT: io::Read,
        DeserializedT: de::DeserializeOwned,
        AnnotatedT: 'static + Annotated + Clone + Default + Send + Sync,
    {
        let variant = self.parse_reader::<_, AnnotatedT>(reader)?;
        variant.deserialize()
    }

    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize_string<DeserializedT, AnnotatedT>(&mut self, string: &str) -> Result<DeserializedT, Problem>
    where
        DeserializedT: de::DeserializeOwned,
        AnnotatedT: 'static + Annotated + Clone + Default + Send + Sync,
    {
        let variant = self.parse_string::<AnnotatedT>(string)?;
        variant.deserialize()
    }
}
