use super::{
    super::{super::annotate::*, list::*},
    mode::*,
};

use {
    problemo::{common::*, *},
    std::fmt,
};

impl<AnnotatedT> List<AnnotatedT> {
    /// Merge another list into this list. Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_problems<'this, ProblemReceiverT>(
        &mut self,
        other: &'this Self,
        merge_mode: &MergeMode,
        problems: &mut ProblemReceiverT,
    ) -> Result<bool, Problem>
    where
        AnnotatedT: Annotated + Clone,
        ProblemReceiverT: ProblemReceiver,
    {
        match merge_mode.list {
            ListMergeMode::Append => {
                if other.inner.is_empty() {
                    Ok(false)
                } else {
                    self.inner.extend(other.inner.iter().cloned());
                    Ok(true)
                }
            }

            ListMergeMode::SkipExisting => {
                let mut changed = false;

                for item in &other.inner {
                    if self.push_unique_clone(item) {
                        changed = true;
                    }
                }

                Ok(changed)
            }

            ListMergeMode::FailExisting => {
                let mut changed = false;

                for item in &other.inner {
                    if self.push_unique_clone(item) {
                        changed = true;
                    } else {
                        problems.give(MergingError::as_problem("list").maybe_with(item.annotations().cloned()))?;
                    }
                }

                Ok(changed)
            }

            ListMergeMode::Replace => {
                if self == other {
                    Ok(false)
                } else {
                    self.inner = other.inner.clone();
                    Ok(true)
                }
            }
        }
    }

    /// Merge another list into this list while failing on the first encountered problem.
    /// Return true if any change happened.
    ///
    /// The merging behavior depends on the [MergeMode].
    pub fn merge_with_mode<'this>(&mut self, other: &'this Self, merge_mode: &MergeMode) -> Result<bool, Problem>
    where
        AnnotatedT: Annotated + Clone,
    {
        self.merge_with_problems(other, merge_mode, &mut FailFast)
    }

    /// Merge another list into this list. Return true if any change happened.
    ///
    /// Uses the default [MergeMode].
    pub fn merge(&mut self, other: &Self) -> bool
    where
        AnnotatedT: Annotated + Clone + fmt::Debug,
    {
        // The default mode should never cause errors
        self.merge_with_mode(other, &Default::default()).expect("merge_with_mode")
    }
}
