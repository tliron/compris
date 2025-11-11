use {
    kutil::std::{error::*, *},
    std::{convert::*, ffi::*, path::*},
};

//
// Format
//

/// CPS format.
#[derive(Clone, Copy, Debug, Default, Display, Eq, FromStr, PartialEq)]
#[display(lowercase)]
#[from_str(lowercase, error = UnknownFormatError)]
pub enum Format {
    /// CBOR.
    CBOR,

    /// MessagePack.
    MessagePack,

    /// YAML.
    #[default]
    YAML,

    /// JSON.
    JSON,

    /// XJSON.
    #[strings("xjson")]
    XJSON,

    /// XML.
    XML,
}

impl Format {
    /// Whether or not this is a binary format (CBOR or MessagePack).
    pub fn is_binary(&self) -> bool {
        (*self == Self::CBOR) || (*self == Self::MessagePack)
    }

    /// From path extension.
    pub fn from_path<PathT>(path: PathT) -> Option<Self>
    where
        PathT: AsRef<Path>,
    {
        if let Some(extension) = path.as_ref().extension() {
            if extension == OsStr::new("cbor") {
                return Some(Self::CBOR);
            } else if extension == OsStr::new("mpk") {
                return Some(Self::MessagePack);
            } else if (extension == OsStr::new("yaml")) || (extension == OsStr::new("yml")) {
                return Some(Self::YAML);
            } else if extension == OsStr::new("json") {
                return Some(Self::JSON);
            } else if extension == OsStr::new("xml") {
                return Some(Self::XML);
            }
        }

        None
    }

    /// From extension.
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "cbor" => Some(Self::CBOR),
            "mpk" => Some(Self::MessagePack),
            "yaml" | "yml" => Some(Self::YAML),
            "json" => Some(Self::JSON),
            "xml" => Some(Self::XML),
            _ => None,
        }
    }
}

//
// UnknownFormatError
//

string_error!(UnknownFormatError, "unknown format");
