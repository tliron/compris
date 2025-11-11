use super::{super::normal::*, deserializer::*};

use {problemo::*, serde::de};

//
// Variant
//

impl<AnnotatedT> Variant<AnnotatedT>
where
    AnnotatedT: 'static + Clone + Send + Sync,
{
    /// Deserialize.
    ///
    /// Will convert number types only if information is not lost. Otherwise, will return an error.
    pub fn deserialize<'de, DeserializedT>(&'de self) -> Result<DeserializedT, Problem>
    where
        DeserializedT: de::Deserialize<'de>,
    {
        DeserializedT::deserialize(&mut Deserializer::new(self)).from_serde_problem()
    }
}
