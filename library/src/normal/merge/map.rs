use super::{
    super::{super::annotate::*, map::*, variant::*},
    mode::*,
};

use {
    problemo::{common::*, *},
    std::fmt,
};

impl<AnnotatedT> Map<AnnotatedT> {
    /// Merge another map into this map. Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_problems<'this, ProblemReceiverT>(
        &mut self,
        other: &'this Self,
        merge_mode: &MergeMode,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        Self: 'this,
        AnnotatedT: Annotated + Clone,
        ProblemReceiverT: ProblemReceiver,
    {
        let mut changed = false;

        for (other_key, other_value) in &other.inner {
            if self.merge_key(other_key, other_value, merge_mode, problems)? {
                changed = true;
            }
        }

        Ok(changed)
    }

    /// Merge another map into this map while failing on the first encountered problem.
    /// Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_mode<'this>(&mut self, other: &'this Self, merge_mode: &MergeMode) -> Result<bool, Problem>
    where
        AnnotatedT: Annotated + Clone,
    {
        self.merge_with_problems(other, merge_mode, &mut FailFast)
    }

    /// Merge another map into this map. Return true if any change happened.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool
    where
        AnnotatedT: Annotated + Clone + fmt::Debug,
    {
        // The default mode should never cause errors
        self.merge_with_mode(other, &Default::default()).expect("merge_with_mode")
    }

    fn merge_key<'this, ProblemReceiverT>(
        &mut self,
        other_key: &'this Variant<AnnotatedT>,
        other_value: &'this Variant<AnnotatedT>,
        merge_mode: &MergeMode,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        AnnotatedT: Annotated + Clone,
        ProblemReceiverT: ProblemReceiver,
    {
        match self.inner.get_mut(other_key) {
            Some(value) => {
                // We already have the key, so merge the value
                Ok(value.merge_with_problems(other_value, merge_mode, problems)?)
            }

            None => {
                // We don't have the key, so insert it
                if self.inner.insert(other_key.clone(), other_value.clone()).is_some() {
                    if merge_mode.map == MapMergeMode::FailExisting {
                        problems
                            .give(MergeError::new("map").into_problem().maybe_with(other_key.annotations().cloned()))?;
                    }
                }
                Ok(true)
            }
        }
    }
}
