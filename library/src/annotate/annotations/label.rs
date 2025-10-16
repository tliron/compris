use kutil::std::immutable::*;

//
// Label
//

/// Label annotation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Label {
    /// Integer.
    Integer(i64),

    /// String.
    String(ByteString),
}
