use super::{
    super::{super::annotate::*, variant::*},
    mode::*,
};

use {problemo::*, std::fmt};

impl<AnnotatedT> Variant<AnnotatedT> {
    /// Merge another [Variant] into this [Variant]. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_problems<ProblemReceiverT>(
        &mut self,
        other: &Self,
        merge_mode: &MergeMode,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        AnnotatedT: Annotated + Clone,
        ProblemReceiverT: ProblemReceiver,
    {
        match (self, other) {
            (Self::List(list), Self::List(other_list)) => list.merge_with_problems(other_list, merge_mode, problems),
            (Self::Map(map), Self::Map(other_map)) => map.merge_with_problems(other_map, merge_mode, problems),
            _ => Ok(false),
        }
    }

    /// Merge another [Variant] into this [Variant] while failing on the first encountered problem.
    /// Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_mode(&mut self, other: &Self, merge_mode: &MergeMode) -> Result<bool, Problem>
    where
        AnnotatedT: Annotated + Clone,
    {
        self.merge_with_problems(other, merge_mode, &mut FailFast)
    }

    /// Merge another [Variant] into this value. Return true if any change happened.
    ///
    /// This function only affects lists and maps.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool
    where
        AnnotatedT: Annotated + Clone + fmt::Debug,
    {
        // The default mode should never cause errors, so unwrap is safe
        self.merge_with_mode(other, &Default::default()).expect("merge_with_mode")
    }
}
