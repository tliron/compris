use {clap::*, compris::*};

//
// OutputFormat
//

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    YAML,
    JSON,
    XJSON,
    XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
    Debug,
}

impl OutputFormat {
    pub fn to_compris(&self) -> Option<Format> {
        match self {
            Self::YAML => Some(Format::YAML),
            Self::JSON => Some(Format::JSON),
            Self::XJSON => Some(Format::XJSON),
            Self::XML => Some(Format::XML),
            Self::CBOR => Some(Format::CBOR),
            Self::MessagePack => Some(Format::MessagePack),
            Self::Debug => None,
        }
    }
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
