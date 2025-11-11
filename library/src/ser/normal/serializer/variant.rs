use super::{
    super::super::super::{annotate::*, normal::*},
    list::*,
    map::*,
};

use {kutil::std::immutable::*, problemo::*, serde::ser::*, std::marker::*};

//
// VariantSerializer
//

/// Normal serializer.
#[derive(Debug, Default)]
pub struct VariantSerializer<AnnotatedT> {
    /// Prefix for labels.
    pub label_prefix: Option<String>,

    annotated: PhantomData<AnnotatedT>,
}

impl<AnnotatedT> VariantSerializer<AnnotatedT> {
    /// Constructor.
    pub fn new<LabelPrefixT>(label_prefix: Option<LabelPrefixT>) -> Self
    where
        LabelPrefixT: ToString,
    {
        Self { label_prefix: label_prefix.map(|label_prefix| label_prefix.to_string()), annotated: Default::default() }
    }

    /// With label.
    pub fn with_label(&self, variant: Variant<AnnotatedT>, label: &'static str) -> Variant<AnnotatedT>
    where
        AnnotatedT: Annotated,
    {
        match &self.label_prefix {
            Some(label_prefix) => {
                let label = label_prefix.clone() + label;
                variant.with_label(Some(Label::String(label.into())))
            }
            None => variant.with_label(Some(Label::String(ByteString::from_static(label)))),
        }
    }
}

impl<'ser, AnnotatedT> Serializer for &'ser VariantSerializer<AnnotatedT>
where
    AnnotatedT: Annotated + Default,
{
    type Ok = Variant<AnnotatedT>;
    type Error = SerdeProblem;
    type SerializeSeq = ListSerializer<'ser, AnnotatedT>;
    type SerializeTuple = ListSerializer<'ser, AnnotatedT>;
    type SerializeTupleStruct = ListSerializer<'ser, AnnotatedT>;
    type SerializeTupleVariant = ListSerializer<'ser, AnnotatedT>;
    type SerializeMap = MapSerializer<'ser, AnnotatedT>;
    type SerializeStruct = MapSerializer<'ser, AnnotatedT>;
    type SerializeStructVariant = MapSerializer<'ser, AnnotatedT>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(value.into())
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(String::from(value).into())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(ByteString::from(value).into())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Bytes::copy_from_slice(value).into())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Variant::Undefined)
    }

    fn serialize_some<SerializeT>(self, value: &SerializeT) -> Result<Self::Ok, Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        // Note: we lose the fact that this was a Some
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(().into())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit().map(|variant| self.with_label(variant, name))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit_struct(name)
    }

    fn serialize_newtype_struct<SerializeT>(
        self,
        name: &'static str,
        value: &SerializeT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        value.serialize(self).map(|variant| self.with_label(variant, name))
    }

    fn serialize_newtype_variant<SerializeT>(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &SerializeT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializeT: ?Sized + Serialize,
    {
        self.serialize_newtype_struct(name, value)
    }

    fn serialize_seq(self, length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(ListSerializer::new(self, None, length))
    }

    fn serialize_tuple(self, length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(length))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(ListSerializer::new(self, Some(name.into()), Some(length)))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_tuple_struct(name, length)
    }

    fn serialize_map(self, _length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self, None))
    }

    fn serialize_struct(self, name: &'static str, _length: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(MapSerializer::new(self, Some(name.into())))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_struct(name, length)
    }
}
