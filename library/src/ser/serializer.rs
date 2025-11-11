use super::{
    super::{annotate::*, normal::*, *},
    modal::*,
    mode::*,
};

use {
    kutil::std::immutable::*,
    problemo::{common::*, *},
    serde::*,
    std::io,
};

const STRINGIFY_BUFFER_CAPACITY: usize = 1024;

//
// Serializer
//

/// General-purpose serde serializer supporting various formats.
#[derive(Clone)]
pub struct Serializer {
    /// Format.
    pub format: Format,

    /// Pretty output (for YAML, JSON, and XML). Defaults to false.
    pub pretty: bool,

    /// Indent for pretty output (for YAML, JSON, and XML). Defaults to 2.
    pub indent: u64,

    /// Base64 output (for CBOR and MessagePack). Defaults to false.
    pub base64: bool,
}

impl Serializer {
    /// Constructor.
    pub fn new(format: Format) -> Self {
        Self { format, pretty: false, indent: 2, base64: false }
    }

    /// Set format.
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Set pretty output (for YAML, JSON, and XML).
    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }

    /// Set indent for pretty output (for YAML, JSON, and XML).
    pub fn with_indent(mut self, indent: u64) -> Self {
        self.indent = indent;
        self
    }

    /// Set Base64 output (for CBOR and MessagePack).
    pub fn with_base64(mut self, base64: bool) -> Self {
        self.base64 = base64;
        self
    }

    /// Serializes the provided value to the writer according to [Serializer::format](Serializer).
    pub fn write<WriteT, SerializableT>(&self, value: &SerializableT, writer: &mut WriteT) -> Result<(), Problem>
    where
        WriteT: io::Write,
        SerializableT: Serialize,
    {
        match self.format {
            #[cfg(feature = "cbor")]
            Format::CBOR => self.write_cbor(value, writer),

            #[cfg(feature = "messagepack")]
            Format::MessagePack => self.write_message_pack(value, writer),

            #[cfg(feature = "yaml")]
            Format::YAML => self.write_yaml(value, writer),

            #[cfg(feature = "json")]
            Format::JSON | Format::XJSON => self.write_json(value, writer),

            #[cfg(feature = "xml")]
            Format::XML => self.write_xml(value, writer),

            #[cfg(not(all(
                feature = "cbor",
                feature = "messagepack",
                feature = "yaml",
                feature = "json",
                feature = "xml",
            )))]
            format => {
                use super::errors::*;
                Err(UnsupportedError::new("serialization format").into_serialize_problem(format))
            }
        }
    }

    /// Serializes the provided value to the writer according to [Serializer::format](Serializer).
    ///
    /// Uses the provided [SerializationMode].
    pub fn write_with_mode<WriteT, AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        mode: &SerializationMode,
        writer: &mut WriteT,
    ) -> Result<(), Problem>
    where
        WriteT: io::Write,
        AnnotatedT: Annotated + Clone + Default,
    {
        let value = value.modal(mode, self);
        self.write(&value, writer)
    }

    /// Serializes the provided value to the writer according to [Serializer::format](Serializer).
    ///
    /// Will use a [SerializationMode] if one is available for the format.
    pub fn write_modal<WriteT, AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        writer: &mut WriteT,
    ) -> Result<(), Problem>
    where
        WriteT: io::Write,
        AnnotatedT: Annotated + Clone + Default,
    {
        match SerializationMode::for_format(&self.format) {
            Some(serialization_mode) => self.write_with_mode(value, &serialization_mode, writer),
            None => self.write(value, writer),
        }
    }

    /// Convenience function to serialize to a string.
    ///
    /// Binary formats will always use Base64.
    pub fn stringify<SerializableT>(&self, value: &SerializableT) -> Result<ByteString, Problem>
    where
        SerializableT: Serialize,
    {
        let mut writer = Vec::with_capacity(STRINGIFY_BUFFER_CAPACITY);
        match self.clone().with_base64(true).write(value, &mut writer) {
            Ok(_) => ByteString::try_from(writer).via(LowLevelError),
            Err(error) => Err(error),
        }
    }

    /// Convenience function to serialize to a string.
    ///
    /// Binary formats will always use Base64.
    ///
    /// Uses the provided [SerializationMode].
    pub fn stringify_with_mode<AnnotatedT>(
        &self,
        value: &Variant<AnnotatedT>,
        mode: &SerializationMode,
    ) -> Result<ByteString, Problem>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let value = value.modal(mode, self);
        self.stringify(&value)
    }

    /// Convenience function to serialize to a string.
    ///
    /// Binary formats will always use Base64.
    ///
    /// Will use a [SerializationMode] if one is available for the format.
    pub fn stringify_modal<AnnotatedT>(&self, value: &Variant<AnnotatedT>) -> Result<ByteString, Problem>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match SerializationMode::for_format(&self.format) {
            Some(serialization_mode) => self.stringify_with_mode(value, &serialization_mode),
            None => self.stringify(value),
        }
    }

    // Utils

    #[allow(dead_code)]
    pub(crate) fn write_newline<WriteT>(writer: &mut WriteT) -> Result<(), Problem>
    where
        WriteT: io::Write,
    {
        writer.write(b"\n").via(LowLevelError)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn base64_writer<WriteT>(
        writer: &mut WriteT,
    ) -> base64::write::EncoderWriter<'_, base64::engine::GeneralPurpose, &mut WriteT>
    where
        WriteT: io::Write,
    {
        base64::write::EncoderWriter::new(writer, &base64::engine::general_purpose::STANDARD)
    }
}
