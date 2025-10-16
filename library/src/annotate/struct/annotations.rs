use super::super::annotations::*;

use kutil::std::{collections::*, immutable::*};

//
// StructAnnotations
//

/// Struct [Annotations].
pub type StructAnnotations = FastHashMap<ByteString, Annotations>;
