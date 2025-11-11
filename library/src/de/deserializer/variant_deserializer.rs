use super::{super::super::normal::*, deserializer::*, errors::*};

use {
    problemo::*,
    serde::{Deserializer as _, de},
};

//
// VariantDeserializer
//

pub(crate) struct VariantDeserializer<'de, AnnotatedT> {
    variant: &'de Variant<AnnotatedT>,
}

impl<'de, AnnotatedT> VariantDeserializer<'de, AnnotatedT> {
    pub fn new(variant: &'de Variant<AnnotatedT>) -> Self {
        Self { variant }
    }
}

impl<'de, AnnotatedT> de::VariantAccess<'de> for VariantDeserializer<'de, AnnotatedT>
where
    AnnotatedT: 'static + Clone + Send + Sync,
{
    type Error = SerdeProblem;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(incompatible_deserialization_problem("unit", self.variant.clone()))
    }

    fn newtype_variant_seed<SeedT>(self, seed: SeedT) -> Result<SeedT::Value, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut Deserializer::new(self.variant))
    }

    fn tuple_variant<VisitorT>(self, len: usize, visitor: VisitorT) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        Deserializer::new(self.variant).deserialize_tuple(len, visitor)
    }

    fn struct_variant<VisitorT>(
        self,
        fields: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Self::Error>
    where
        VisitorT: de::Visitor<'de>,
    {
        Deserializer::new(self.variant).deserialize_struct("", fields, visitor)
    }
}
