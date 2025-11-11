use super::super::{super::format::*, errors::*, serializer::*};

use {
    borc::{basic::streaming::*, errors::*},
    problemo::*,
    serde::{Serialize, ser},
    std::io,
};

impl Serializer {
    /// Serializes the provided value to the writer as CBOR.
    ///
    /// Is affected by [Serializer::base64](super::super::Serializer::base64).
    pub fn write_cbor<WriteT, SerializableT>(&self, value: &SerializableT, writer: &mut WriteT) -> Result<(), Problem>
    where
        WriteT: io::Write,
        SerializableT: Serialize + ?Sized,
    {
        fn write<WriteT, SerializeT>(value: &SerializeT, writer: &mut WriteT) -> Result<(), Problem>
        where
            WriteT: io::Write,
            SerializeT: Serialize + ?Sized,
        {
            value.serialize(&mut CborSerializer::new(writer)).from_serde_problem().with(Format::CBOR)
        }

        if self.base64 {
            write(value, &mut Self::base64_writer(writer)).into_low_level_serialization_problem(Format::CBOR)?;
        } else {
            write(value, writer).into_low_level_serialization_problem(Format::CBOR)?;
        }

        if self.pretty {
            Self::write_newline(writer).into_low_level_serialization_problem(Format::CBOR)
        } else {
            Ok(())
        }
    }
}

//
// CborSerializer
//

struct CborSerializer<WriteT>
where
    WriteT: io::Write,
{
    encoder: Encoder<WriteT>,
}

impl<WriteT> CborSerializer<WriteT>
where
    WriteT: io::Write,
{
    fn new(writer: WriteT) -> Self {
        Self { encoder: Encoder::new(writer) }
    }

    fn event(self: &mut Self, event: Event) -> Result<(), EncodeError> {
        tracing::trace!("{:?}", event);
        self.encoder.feed_event(event)
    }
}

impl<'this, WriteT> ser::Serializer for &'this mut CborSerializer<WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;
    type SerializeSeq = CborSeqSerializer<'this, WriteT>;
    type SerializeTuple = CborTupleSerializer<'this, WriteT>;
    type SerializeTupleStruct = CborTupleStructSerializer<'this, WriteT>;
    type SerializeTupleVariant = CborTupleVariantSerializer<'this, WriteT>;
    type SerializeMap = CborMapSerializer<'this, WriteT>;
    type SerializeStruct = CborStructSerializer<'this, WriteT>;
    type SerializeStructVariant = CborStructVariantSerializer<'this, WriteT>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Bool(v)).into_serde_serialize_problem()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        self.event(Event::create_signed(v as i64)).into_serde_serialize_problem()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        self.event(Event::create_signed(v as i64)).into_serde_serialize_problem()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        self.event(Event::create_signed(v as i64)).into_serde_serialize_problem()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        // Note: Borc will encode positive integers as unsigned
        self.event(Event::create_signed(v)).into_serde_serialize_problem()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Unsigned(v as u64)).into_serde_serialize_problem()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(self.event(Event::Unsigned(v as u64)).into_serde_serialize_problem()?)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Unsigned(v as u64)).into_serde_serialize_problem()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Unsigned(v)).into_serde_serialize_problem()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Float(v as f64)).into_serde_serialize_problem()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Float(v)).into_serde_serialize_problem()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Unsigned(v as u64)).into_serde_serialize_problem()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.event(Event::TextString(v.into())).into_serde_serialize_problem()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.event(Event::ByteString(v.into())).into_serde_serialize_problem()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Null).into_serde_serialize_problem()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.event(Event::Tag(variant_index as u64)).into_serde_serialize_problem()?;
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<SerializableT>(
        self,
        _name: &'static str,
        value: &SerializableT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<SerializableT>(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &SerializableT,
    ) -> Result<Self::Ok, Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        self.event(Event::Tag(variant_index as u64)).into_serde_serialize_problem()?;
        self.event(Event::Map(1)).into_serde_serialize_problem()?;
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => {
                self.event(Event::Array(len as u64)).into_serde_serialize_problem()?;
                Ok(CborSeqSerializer { serializer: self, known: true })
            }
            None => {
                self.event(Event::UnknownLengthArray).into_serde_serialize_problem()?;
                Ok(CborSeqSerializer { serializer: self, known: false })
            }
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.event(Event::Array(len as u64)).into_serde_serialize_problem()?;
        Ok(CborTupleSerializer { serializer: self })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.event(Event::Array(len as u64)).into_serde_serialize_problem()?;
        Ok(CborTupleStructSerializer { serializer: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.event(Event::Tag(variant_index as u64)).into_serde_serialize_problem()?;
        self.event(Event::Map(1)).into_serde_serialize_problem()?;
        variant.serialize(&mut *self)?;
        self.event(Event::Array(len as u64)).into_serde_serialize_problem()?;
        Ok(CborTupleVariantSerializer { serializer: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(len) => {
                self.event(Event::Map(len as u64)).into_serde_serialize_problem()?;
                Ok(CborMapSerializer { serializer: self, known: true })
            }
            None => {
                self.event(Event::UnknownLengthMap).into_serde_serialize_problem()?;
                Ok(CborMapSerializer { serializer: self, known: false })
            }
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.event(Event::Map(len as u64)).into_serde_serialize_problem()?;
        Ok(CborStructSerializer { serializer: self })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.event(Event::Tag(variant_index as u64)).into_serde_serialize_problem()?;
        self.event(Event::Map(1)).into_serde_serialize_problem()?;
        variant.serialize(&mut *self)?;
        self.event(Event::Map(len as u64)).into_serde_serialize_problem()?;
        Ok(CborStructVariantSerializer { serializer: self })
    }
}

//
// CborSeqSerializer
//

pub struct CborSeqSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
    known: bool,
}

impl<'this, WriteT> ser::SerializeSeq for CborSeqSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_element<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.known { Ok(()) } else { self.serializer.event(Event::Break).into_serde_serialize_problem() }
    }
}

//
// CborTupleSerializer
//

pub struct CborTupleSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
}

impl<'this, WriteT> ser::SerializeTuple for CborTupleSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_element<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborTupleStructSerializer
//

pub struct CborTupleStructSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
}

impl<'this, WriteT> ser::SerializeTupleStruct for CborTupleStructSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_field<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborTupleVariantSerializer
//

pub struct CborTupleVariantSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
}

impl<'this, WriteT> ser::SerializeTupleVariant for CborTupleVariantSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_field<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborMapSerializer
//

pub struct CborMapSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
    known: bool,
}

impl<'this, WriteT> ser::SerializeMap for CborMapSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_key<SerializableT>(&mut self, key: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<SerializableT>(&mut self, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.known { Ok(()) } else { self.serializer.event(Event::Break).into_serde_serialize_problem() }
    }
}

//
// CborStructSerializer
//

pub struct CborStructSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
}

impl<'this, WriteT> ser::SerializeStruct for CborStructSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_field<SerializableT>(&mut self, key: &'static str, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

//
// CborStructVariantSerializer
//

pub struct CborStructVariantSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    serializer: &'this mut CborSerializer<WriteT>,
}

impl<'this, WriteT> ser::SerializeStructVariant for CborStructVariantSerializer<'this, WriteT>
where
    WriteT: io::Write,
{
    type Ok = ();
    type Error = SerdeProblem;

    fn serialize_field<SerializableT>(&mut self, key: &'static str, value: &SerializableT) -> Result<(), Self::Error>
    where
        SerializableT: ?Sized + Serialize,
    {
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

// //
// // SerializeCborError
// //

// /// Serialize CBOR error.
// ///
// /// We can't use [Problem] because we need the error to implement [ser::Error].
// #[derive(Debug, Display, Error, From)]
// pub enum SerializeCborError {
//     /// Encode.
//     #[display("{_0:?}")]
//     Encode(borc::errors::EncodeError),

//     /// Custom Serde.
//     Custom(#[error(not(source))] String),
// }

// impl ser::Error for SerializeCborError {
//     fn custom<DisplayT>(custom: DisplayT) -> Self
//     where
//         DisplayT: fmt::Display,
//     {
//         Self::Custom(format!("{}", custom))
//     }
// }
