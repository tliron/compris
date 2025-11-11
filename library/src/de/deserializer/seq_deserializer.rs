use super::{super::super::normal::*, deserializer::*};

use {problemo::*, serde::de, std::slice::*};

//
// SeqDeserializer
//

pub(crate) struct SeqDeserializer<'de, AnnotatedT> {
    iterator: Iter<'de, Variant<AnnotatedT>>,
    current_item: Option<&'de Variant<AnnotatedT>>,
}

impl<'de, AnnotatedT> SeqDeserializer<'de, AnnotatedT> {
    pub(crate) fn new(list: &'de List<AnnotatedT>) -> Self {
        Self { iterator: list.inner.iter(), current_item: None }
    }

    fn next(&mut self) {
        self.current_item = self.iterator.next();
    }
}

impl<'de, AnnotatedT> de::SeqAccess<'de> for SeqDeserializer<'de, AnnotatedT>
where
    AnnotatedT: 'static + Clone + Send + Sync,
{
    type Error = SerdeProblem;

    fn next_element_seed<SeedT>(&mut self, seed: SeedT) -> Result<Option<SeedT::Value>, Self::Error>
    where
        SeedT: de::DeserializeSeed<'de>,
    {
        self.next();
        match self.current_item {
            Some(item) => Ok(Some(seed.deserialize(&mut Deserializer::new(item))?)),
            None => Ok(None),
        }
    }
}
