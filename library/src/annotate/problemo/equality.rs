use super::super::annotations::*;

use {problemo::*, std::error::Error};

//
// AnnotatedCauseEquality
//

/// Annotated cause equality attachment.
pub struct AnnotatedCauseEquality;

impl AnnotatedCauseEquality {
    /// Constructor.
    ///
    /// The cause's error type must implement [PartialEq].
    ///
    /// If the cause has an [Annotations] attachment it will also be used for comparison.
    pub fn new<ErrorT>() -> CauseEquality
    where
        ErrorT: 'static + Error + PartialEq,
    {
        CauseEquality(Box::new(Self::eq_::<ErrorT>))
    }

    fn eq_<ErrorT>(left: &Cause, right: &Cause) -> bool
    where
        ErrorT: 'static + Error + PartialEq,
    {
        if let Some(left_error) = left.error.downcast_ref::<ErrorT>()
            && let Some(right_error) = right.error.downcast_ref::<ErrorT>()
        {
            if left_error.eq(right_error) {
                if let Some(left_annotations) = left.attachment_of_type::<Annotations>()
                    && let Some(right_annotations) = right.attachment_of_type::<Annotations>()
                {
                    left_annotations.eq(right_annotations)
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}
