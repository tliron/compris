use super::{
    super::super::super::{annotate::*, normal::*},
    variant::*,
};

use {
    problemo::{common::*, *},
    serde::ser::*,
    std::{collections::*, marker::*, mem::*},
};

//
// MapSerializer
//

/// Map serializer.
#[derive(Debug)]
pub struct MapSerializer<'ser, AnnotatedT> {
    /// Serializer.
    pub serializer: &'ser VariantSerializer<AnnotatedT>,

    /// Label.
    pub label: Option<&'static str>,

    /// Map.
    pub map: BTreeMap<Variant<AnnotatedT>, Variant<AnnotatedT>>,

    /// Current key.
    pub key: Option<Variant<AnnotatedT>>,
}

impl<'ser, AnnotatedT> MapSerializer<'ser, AnnotatedT> {
    /// Constructor.
    pub fn new(serializer: &'ser VariantSerializer<AnnotatedT>, label: Option<&'static str>) -> Self {
        Self { serializer, label, map: Default::default(), key: None }
    }
}

impl<'ser, AnnotatedT> Into<Variant<AnnotatedT>> for MapSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        match self.label {
            Some(label) => self.serializer.with_label(self.map.into(), &label),
            None => self.map.into(),
        }
    }
}

impl<'ser, AnnotatedT> SerializeMap for MapSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerdeProblem;

    fn serialize_key<SerializeT>(&mut self, key: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        if self.key.is_some() {
            return Err(InvalidError::as_problem("key already serialized")
                .via(SerializationError::new("serde"))
                .into());
        }
        let key = key.serialize(self.serializer)?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<SerializeT>(&mut self, value: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        match take(&mut self.key) {
            Some(key) => {
                let value = value.serialize(self.serializer)?;
                self.map.insert(key, value);
                Ok(())
            }

            None => Err(MissingError::as_problem("map key").via(SerializationError::new("serde")).into()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser, AnnotatedT> SerializeStruct for MapSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerdeProblem;

    fn serialize_field<SerializeT>(&mut self, key: &'static str, value: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let value = value.serialize(self.serializer)?;
        self.map.insert(key.into(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser, AnnotatedT> SerializeStructVariant for MapSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerdeProblem;

    fn serialize_field<SerializeT>(&mut self, key: &'static str, value: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let value = value.serialize(self.serializer)?;
        self.map.insert(key.into(), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}
