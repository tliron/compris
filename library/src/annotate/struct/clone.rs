use super::annotations::*;

use kutil::std::immutable::*;

//
// CloneFields
//

/// Clone fields.
pub trait CloneFields {
    /// Clone fields.
    fn clone_fields(&self, field_names: &[&'static str]) -> Self;
}

impl CloneFields for StructAnnotations {
    fn clone_fields(&self, field_names: &[&'static str]) -> Self {
        let mut clone = Self::default();
        for field_name in field_names {
            if let Some(annotations) = self.get(*field_name) {
                clone.insert(ByteString::from_static(*field_name), annotations.clone());
            }
        }
        clone
    }
}
