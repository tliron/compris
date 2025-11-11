use super::{super::super::normal::*, deserializer::*, errors::*, variant_deserializer::*};

use {problemo::*, serde::de};

//
// EnumDeserializer
//

pub(crate) struct EnumDeserializer<'de, AnnotatedT> {
    key: &'de Variant<AnnotatedT>,
    value: &'de Variant<AnnotatedT>,
}

impl<'de, AnnotatedT> EnumDeserializer<'de, AnnotatedT> {
    pub(crate) fn new(map: &'de Map<AnnotatedT>) -> Result<Self, SerdeProblem>
    where
        AnnotatedT: 'static + Clone + Send + Sync,
    {
        if map.inner.len() == 1 {
            let (key, value) = map.inner.iter().next().expect("non-empty");
            Ok(Self { key, value })
        } else {
            Err(incompatible_error("map length is not 1", map.clone()))
        }
    }
}

impl<'de, AnnotatedT> de::EnumAccess<'de> for EnumDeserializer<'de, AnnotatedT>
where
    AnnotatedT: 'static + Clone + Send + Sync,
{
    type Error = SerdeProblem;
    type Variant = VariantDeserializer<'de, AnnotatedT>;

    fn variant_seed<SeedT>(self, seed: SeedT) -> Result<(SeedT::Value, Self::Variant), Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        let key = seed.deserialize(&mut Deserializer::new(self.key))?;
        Ok((key, VariantDeserializer::new(self.value)))
    }
}
