use super::{
    super::super::super::{annotate::*, normal::*},
    error::*,
    variant::*,
};

use {serde::ser::*, std::marker::*};

//
// ListSerializer
//

/// List serializer.
#[derive(Debug)]
pub struct ListSerializer<'ser, AnnotatedT> {
    /// Serializer.
    pub serializer: &'ser VariantSerializer<AnnotatedT>,

    /// Label.
    pub label: Option<&'static str>,

    /// List.
    pub list: Vec<Variant<AnnotatedT>>,
}

impl<'ser, AnnotatedT> ListSerializer<'ser, AnnotatedT> {
    /// Constructor.
    pub fn new(
        serializer: &'ser VariantSerializer<AnnotatedT>,
        label: Option<&'static str>,
        length: Option<usize>,
    ) -> Self {
        Self {
            serializer,
            label,
            list: match length {
                Some(length) => Vec::with_capacity(length),
                None => Vec::default(),
            },
        }
    }
}

impl<'ser, AnnotatedT> Into<Variant<AnnotatedT>> for ListSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        match self.label {
            Some(label) => self.serializer.with_label(self.list.into(), &label),
            None => self.list.into(),
        }
    }
}

impl<'ser, AnnotatedT> SerializeSeq for ListSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerializeVariantError;

    fn serialize_element<SerializeT>(&mut self, element: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let element = element.serialize(self.serializer)?;
        self.list.push(element);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser, AnnotatedT> SerializeTuple for ListSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerializeVariantError;

    fn serialize_element<SerializeT>(&mut self, element: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let element = element.serialize(self.serializer)?;
        self.list.push(element);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser, AnnotatedT> SerializeTupleStruct for ListSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerializeVariantError;

    fn serialize_field<SerializeT>(&mut self, field: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let field = field.serialize(self.serializer)?;
        self.list.push(field);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}

impl<'ser, AnnotatedT> SerializeTupleVariant for ListSerializer<'ser, AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerializeVariantError;

    fn serialize_field<SerializeT>(&mut self, field: &SerializeT) -> Result<(), Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        let field = field.serialize(self.serializer)?;
        self.list.push(field);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.into())
    }
}
