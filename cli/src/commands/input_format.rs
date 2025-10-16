use {clap::*, compris::*};

//
// InputFormat
//

#[derive(Clone, ValueEnum)]
pub enum InputFormat {
    YAML,
    JSON,
    XJSON,
    XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
}

impl InputFormat {
    pub fn to_compris(&self) -> Format {
        match self {
            Self::YAML => Format::YAML,
            Self::JSON => Format::JSON,
            Self::XJSON => Format::XJSON,
            Self::XML => Format::XML,
            Self::CBOR => Format::CBOR,
            Self::MessagePack => Format::MessagePack,
        }
    }
}

impl ToString for InputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
