use super::super::annotations::*;

//
// AnnotatedStruct
//

/// Has [Annotations] for the struct and its fields.
pub trait AnnotatedStruct {
    /// A field's [Annotations].
    ///
    /// An empty name is used to refer to annotations for the struct itself.
    fn field_annotations(&self, name: &str) -> Option<&Annotations>;

    /// A field's [Annotations].
    ///
    /// An empty name is used to refer to annotations for the struct itself.
    fn field_annotations_mut(&mut self, name: &str) -> Option<&mut Annotations>;

    /// The struct's [Annotations].
    fn struct_annotations(&self) -> Option<&Annotations> {
        self.field_annotations("")
    }

    /// The struct's [Annotations].
    fn struct_annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.field_annotations_mut("")
    }

    /// The field's [Annotations] or the struct's if the field's aren't found.
    fn field_or_struct_annotations(&self, name: &str) -> Option<&Annotations> {
        self.field_annotations(name).or_else(|| self.struct_annotations())
    }

    /// Clone field [Annotations] another struct.
    fn clone_field_annotations<AnnotatedStructT>(&mut self, name: &str, other: &AnnotatedStructT)
    where
        AnnotatedStructT: AnnotatedStruct,
    {
        if let Some(other_annotations) = other.field_annotations(name)
            && let Some(annotations) = self.field_annotations_mut(name)
        {
            *annotations = other_annotations.clone();
        }
    }
}
