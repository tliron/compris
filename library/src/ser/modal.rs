use super::{mode::*, serializer::Serializer as ModalSerializer};

use serde::ser::*;

//
// SerializeModal
//

/// Like [Serialize] but with support for a [SerializationMode].
pub trait SerializeModal {
    /// Serialize with [SerializationMode].
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer;
}

//
// SerializeModalRescursive
//

/// Like [Serialize] but with support for a [SerializationMode]
/// and an embedded [ModalSerializer].
pub trait SerializeModalRescursive {
    /// Serialize with [SerializationMode] and an embedded [Serializer](ModalSerializer).
    fn serialize_modal<SerializerT>(
        &self,
        serializer: SerializerT,
        mode: &SerializationMode,
        modal_serializer: &ModalSerializer,
    ) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer;
}

//
// ModalSerializable
//

/// Provides a [Serialize] implementation for a [SerializeModal].
pub struct ModalSerializable<'inner, InnerT>
where
    InnerT: SerializeModal,
{
    /// Inner.
    pub inner: &'inner InnerT,

    /// Serialization mode.
    pub mode: &'inner SerializationMode,
}

impl<'inner, InnerT> ModalSerializable<'inner, InnerT>
where
    InnerT: SerializeModal,
{
    /// Constructor.
    pub fn new(inner: &'inner InnerT, mode: &'inner SerializationMode) -> Self {
        Self { inner, mode }
    }
}

// Delegated

impl<'inner, InnerT> Serialize for ModalSerializable<'inner, InnerT>
where
    InnerT: SerializeModal,
{
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        self.inner.serialize_modal(serializer, self.mode)
    }
}

//
// RecursiveModalSerializable
//

/// Provides a [Serialize] implementation for a [SerializeModalRescursive].
pub struct RecursiveModalSerializable<'inner, InnerT>
where
    InnerT: SerializeModalRescursive,
{
    /// Inner.
    pub inner: &'inner InnerT,

    /// Serialization mode.
    pub mode: &'inner SerializationMode,

    /// Modal serializer.
    pub serializer: &'inner ModalSerializer,
}

impl<'inner, InnerT> RecursiveModalSerializable<'inner, InnerT>
where
    InnerT: SerializeModalRescursive,
{
    /// Constructor.
    pub fn new(inner: &'inner InnerT, mode: &'inner SerializationMode, serializer: &'inner ModalSerializer) -> Self {
        Self { inner, mode, serializer }
    }
}

// Delegated

impl<'inner, SerializeModalT> Serialize for RecursiveModalSerializable<'inner, SerializeModalT>
where
    SerializeModalT: SerializeModalRescursive,
{
    fn serialize<SerializerT>(&self, serializer: SerializerT) -> Result<SerializerT::Ok, SerializerT::Error>
    where
        SerializerT: Serializer,
    {
        self.inner.serialize_modal(serializer, self.mode, self.serializer)
    }
}

//
// Modal
//

/// Wraps a [SerializeModal] with a [ModalSerializable].
pub trait Modal<SerializeModalT>
where
    SerializeModalT: SerializeModal,
{
    /// Wraps a [SerializeModal] with a [ModalSerializable].
    fn modal<'this, 'mode, 'modal>(
        &'this self,
        mode: &'mode SerializationMode,
    ) -> ModalSerializable<'modal, SerializeModalT>
    where
        'this: 'modal,
        'mode: 'modal;
}

impl<SerializeModalT> Modal<SerializeModalT> for SerializeModalT
where
    SerializeModalT: SerializeModal,
{
    fn modal<'this, 'mode, 'modal>(
        &'this self,
        mode: &'mode SerializationMode,
    ) -> ModalSerializable<'modal, SerializeModalT>
    where
        'this: 'modal,
        'mode: 'modal,
    {
        ModalSerializable::new(self, mode)
    }
}

//
// RecursiveModal
//

/// Wraps a [SerializeModalRescursive] with a [RecursiveModalSerializable].
pub trait RecursiveModal<SerializeModalT>
where
    SerializeModalT: SerializeModalRescursive,
{
    /// Wraps a [SerializeModalRescursive] with a [RecursiveModalSerializable].
    fn modal<'this, 'mode, 'modal>(
        &'this self,
        mode: &'mode SerializationMode,
        serializer: &'mode ModalSerializer,
    ) -> RecursiveModalSerializable<'modal, SerializeModalT>
    where
        'this: 'modal,
        'mode: 'modal;
}

impl<ModalSerializeT> RecursiveModal<ModalSerializeT> for ModalSerializeT
where
    ModalSerializeT: SerializeModalRescursive,
{
    fn modal<'this, 'mode, 'modal>(
        &'this self,
        mode: &'mode SerializationMode,
        serializer: &'mode ModalSerializer,
    ) -> RecursiveModalSerializable<'modal, Self>
    where
        'this: 'modal,
        'mode: 'modal,
    {
        RecursiveModalSerializable::new(self, mode, serializer)
    }
}
