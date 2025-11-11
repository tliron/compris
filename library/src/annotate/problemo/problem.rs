use super::super::{annotations::*, traits::*};

use problemo::*;

impl Annotated for Problem {
    fn can_have_annotations() -> bool {
        true
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.attachment_of_type()
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        Some(self.must_attachment_of_type_mut(|| Annotations::default()))
    }
}
