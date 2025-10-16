use super::annotations::*;

use kutil::std::immutable::*;

//
// CloneFields
//

/// Clone fields.
pub trait CloneFields {
    /// Clone field [Annotations](super::super::Annotations).
    fn clone_field_from(&mut self, name: &'static str, other: &Self);

    /// Clone with only the provided field [Annotations](super::super::Annotations).
    fn clone_fields(&self, names: &[&'static str]) -> Self;
}

impl CloneFields for StructAnnotations {
    fn clone_field_from(&mut self, name: &'static str, other: &Self) {
        if let Some(annotations) = other.get(name) {
            self.insert(ByteString::from_static(name), annotations.clone());
        }
    }

    fn clone_fields(&self, names: &[&'static str]) -> Self {
        let mut clone = Self::default();
        for name in names {
            clone.clone_field_from(name, self);
        }
        clone
    }
}
